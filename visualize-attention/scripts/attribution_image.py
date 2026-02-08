#!/usr/bin/env python3
"""
Chunk-occlusion attribution script that visualizes which parts of the input
were most important for the model's output.

This script:
1. Loads a prompt and an open-weights model
2. Runs baseline inference to get answer and logprobs
3. For each chunk (paragraph/sentence), removes it and re-runs
4. Computes importance scores as baseline_score - occluded_score
5. Generates a heatmap visualization saved to disk
"""

import argparse
import os
import re
from pathlib import Path
from typing import List, Tuple, Dict

import torch
import numpy as np
import matplotlib.pyplot as plt
from matplotlib.colors import LinearSegmentedColormap
from huggingface_hub.utils import logging as hf_logging
from transformers import AutoConfig, AutoModelForCausalLM, AutoTokenizer
from transformers.utils import logging as transformers_logging
from tqdm import tqdm


def chunk_by_paragraphs(text: str) -> List[Tuple[int, int, str]]:
    """
    Split text into paragraphs.
    Returns list of (start_char, end_char, chunk_text) tuples.
    """
    chunks = []
    paragraphs = text.split('\n\n')
    current_pos = 0
    
    for para in paragraphs:
        if para.strip():
            start = current_pos
            end = current_pos + len(para)
            chunks.append((start, end, para.strip()))
            current_pos = end + 2  # +2 for '\n\n'
    
    return chunks


def chunk_by_sentences(text: str) -> List[Tuple[int, int, str]]:
    """
    Split text into sentences.
    Returns list of (start_char, end_char, chunk_text) tuples.
    """
    chunks = []
    # Simple sentence splitting on . ! ? followed by space and capital
    sentences = re.split(r'([.!?]\s+)', text)
    
    # Recombine sentence with its punctuation
    i = 0
    current_pos = 0
    while i < len(sentences):
        if i + 1 < len(sentences) and re.match(r'[.!?]\s+', sentences[i + 1]):
            sent = sentences[i] + sentences[i + 1]
            i += 2
        else:
            sent = sentences[i]
            i += 1
        
        if sent.strip():
            start = current_pos
            end = current_pos + len(sent)
            chunks.append((start, end, sent.strip()))
            current_pos = end
    
    return chunks


def remove_chunk(text: str, start: int, end: int) -> str:
    """
    Remove a chunk from text by deletion (recommended method).
    """
    return text[:start] + text[end:]


def compute_answer_logprob_sum(
    model: AutoModelForCausalLM,
    tokenizer: AutoTokenizer,
    prompt: str,
    answer_tokens: List[int],
    max_new_tokens: int = 30,
    device: str = "cuda" if torch.cuda.is_available() else "cpu"
) -> float:
    """
    Compute the sum of logprobs for the baseline answer tokens given the prompt.
    
    This scores how likely the model is to produce the baseline answer tokens
    given the (possibly occluded) prompt.
    
    Args:
        model: The language model
        tokenizer: The tokenizer
        prompt: Input prompt
        answer_tokens: List of token IDs from the baseline answer to score
        max_new_tokens: Maximum tokens to score (default: 30)
        device: Device to run on
    
    Returns:
        Sum of logprobs for the answer tokens
    """
    inputs = tokenizer(prompt, return_tensors="pt").to(device)
    input_ids = inputs.input_ids
    attention_mask = inputs.attention_mask
    
    # Limit to max_new_tokens
    num_to_score = min(len(answer_tokens), max_new_tokens)
    if num_to_score == 0:
        return 0.0
    
    logprobs = []
    current_ids = input_ids.clone()
    current_attention = attention_mask.clone()
    
    with torch.no_grad():
        # Score each answer token one by one
        for i in range(num_to_score):
            # Forward pass
            outputs = model(current_ids, attention_mask=current_attention)
            logits = outputs.logits[0, -1, :]  # Last position logits [vocab_size]
            logprobs_t = torch.nn.functional.log_softmax(logits, dim=-1)
            
            # Get logprob for the target token
            target_token = answer_tokens[i]
            logprob = logprobs_t[target_token].item()
            logprobs.append(logprob)
            
            # Append this token to current_ids for next iteration
            current_ids = torch.cat(
                [current_ids, torch.tensor([[target_token]], device=device)],
                dim=1
            )
            current_attention = torch.cat(
                [current_attention, torch.ones((1, 1), device=device, dtype=current_attention.dtype)],
                dim=1
            )
    
    return sum(logprobs)


def run_baseline(
    model: AutoModelForCausalLM,
    tokenizer: AutoTokenizer,
    prompt: str,
    max_new_tokens: int = 30,
    device: str = "cuda" if torch.cuda.is_available() else "cpu"
) -> Tuple[str, List[int], float]:
    """
    Run baseline inference and return answer text, token IDs, and score.
    """
    inputs = tokenizer(prompt, return_tensors="pt").to(device)
    
    with torch.no_grad():
        outputs = model.generate(
            inputs.input_ids,
            attention_mask=inputs.attention_mask,
            max_new_tokens=max_new_tokens,
            do_sample=False,
            pad_token_id=tokenizer.pad_token_id,
            return_dict_in_generate=True,
            output_scores=True,
        )
    
    # Extract generated tokens
    generated_ids = outputs.sequences[0][inputs.input_ids.shape[1]:]
    answer_tokens = generated_ids.tolist()
    
    # Decode answer
    answer_text = tokenizer.decode(generated_ids, skip_special_tokens=True)
    
    # Compute baseline score
    baseline_score = compute_answer_logprob_sum(
        model, tokenizer, prompt, answer_tokens, max_new_tokens, device
    )
    
    return answer_text, answer_tokens, baseline_score


def compute_attribution(
    model: AutoModelForCausalLM,
    tokenizer: AutoTokenizer,
    prompt: str,
    chunks: List[Tuple[int, int, str]],
    baseline_answer_tokens: List[int],
    baseline_score: float,
    max_new_tokens: int = 30,
    device: str = "cuda" if torch.cuda.is_available() else "cpu"
) -> List[Tuple[Tuple[int, int, str], float]]:
    """
    Compute importance scores for each chunk via occlusion.
    
    Returns:
        List of ((start, end, chunk_text), importance_score) tuples
    """
    results = []
    
    print(f"Computing attribution for {len(chunks)} chunks...")
    for start, end, chunk_text in tqdm(chunks):
        # Remove this chunk
        occluded_prompt = remove_chunk(prompt, start, end)
        
        # Re-run inference
        try:
            occluded_score = compute_answer_logprob_sum(
                model, tokenizer, occluded_prompt,
                baseline_answer_tokens, max_new_tokens, device
            )
            
            # Importance = how much removing it hurts
            importance = baseline_score - occluded_score
            results.append(((start, end, chunk_text), importance))
        except Exception as e:
            print(f"Warning: Error processing chunk at {start}-{end}: {e}")
            results.append(((start, end, chunk_text), 0.0))
    
    return results


def create_heatmap(
    prompt: str,
    chunk_results: List[Tuple[Tuple[int, int, str], float]],
    output_path: str,
    title: str = "Token Importance Heatmap"
):
    """
    Create a heatmap visualization showing importance scores overlaid on the prompt.
    """
    # Create a character-level importance array
    char_importance = np.zeros(len(prompt))
    
    for (start, end, _), importance in chunk_results:
        # Assign importance to all characters in this chunk
        char_importance[start:end] = max(char_importance[start:end].max(), importance)
    
    # Normalize for visualization (0-1 scale)
    if char_importance.max() > 0:
        char_importance_norm = char_importance / char_importance.max()
    else:
        char_importance_norm = char_importance
    
    # Create figure
    fig, ax = plt.subplots(figsize=(14, 8))
    
    # Create a colormap (green for low, yellow for medium, red for high)
    colors = ['#2ecc71', '#f39c12', '#e74c3c']  # green, orange, red
    n_bins = 100
    cmap = LinearSegmentedColormap.from_list('importance', colors, N=n_bins)
    
    # Create a text display with color overlay
    # Split prompt into lines for better display
    lines = prompt.split('\n')
    y_pos = len(lines)
    
    # Draw text with background colors
    current_pos = 0
    for line_idx, line in enumerate(lines):
        if not line.strip():
            y_pos -= 1
            continue
            
        # For each character in the line, get its importance
        line_importances = []
        for char in line:
            if current_pos < len(char_importance_norm):
                line_importances.append(char_importance_norm[current_pos])
            else:
                line_importances.append(0.0)
            current_pos += 1
        
        # Draw colored rectangles for each character
        x_start = 0
        for i, (char, imp) in enumerate(zip(line, line_importances)):
            color = cmap(imp)
            # Draw background rectangle
            ax.add_patch(plt.Rectangle(
                (x_start, y_pos - 0.4), 0.6, 0.8,
                facecolor=color, alpha=0.3, edgecolor='none'
            ))
            x_start += 0.6
        
        # Draw text on top
        ax.text(0, y_pos, line, fontsize=10, family='monospace',
                verticalalignment='center', zorder=10)
        
        y_pos -= 1
        current_pos += 1  # for newline
    
    # Add colorbar
    sm = plt.cm.ScalarMappable(cmap=cmap, norm=plt.Normalize(vmin=0, vmax=1))
    sm.set_array([])
    cbar = plt.colorbar(sm, ax=ax)
    cbar.set_label('Normalized Importance', rotation=270, labelpad=20)
    
    ax.set_xlim(0, 80)
    ax.set_ylim(0, len(lines) + 1)
    ax.set_title(title, fontsize=14, fontweight='bold')
    ax.axis('off')
    
    # Add summary statistics
    max_imp = max(imp for _, imp in chunk_results) if chunk_results else 0
    min_imp = min(imp for _, imp in chunk_results) if chunk_results else 0
    mean_imp = np.mean([imp for _, imp in chunk_results]) if chunk_results else 0
    
    stats_text = f"Max: {max_imp:.3f} | Min: {min_imp:.3f} | Mean: {mean_imp:.3f}"
    ax.text(0, len(lines) + 0.5, stats_text, fontsize=9, style='italic')
    
    plt.tight_layout()
    plt.savefig(output_path, dpi=150, bbox_inches='tight')
    print(f"Heatmap saved to {output_path}")
    plt.close()


def main():
    parser = argparse.ArgumentParser(
        description="Compute chunk-occlusion attribution and generate heatmap"
    )
    parser.add_argument(
        "--prompt", type=str, required=True,
        help="Path to prompt text file, or prompt text directly"
    )
    parser.add_argument(
        "--model", type=str, default="microsoft/DialoGPT-small",
        help="HuggingFace model name (default: microsoft/DialoGPT-small)"
    )
    parser.add_argument(
        "--output", type=str, default="outputs/heatmap.png",
        help="Output path for heatmap image (default: outputs/heatmap.png)"
    )
    parser.add_argument(
        "--max-new-tokens", type=int, default=30,
        help="Maximum tokens to generate (default: 30)"
    )
    parser.add_argument(
        "--chunk-method", type=str, choices=["paragraph", "sentence"], default="paragraph",
        help="Chunking method: paragraph or sentence (default: paragraph)"
    )
    parser.add_argument(
        "--device", type=str, default=None,
        help="Device to use (cuda/cpu). Auto-detected if not specified"
    )
    
    args = parser.parse_args()

    # Keep HF Hub / Transformers warnings quiet unless explicitly needed.
    hf_logging.set_verbosity_error()
    transformers_logging.set_verbosity_error()
    
    # Determine device
    if args.device is None:
        device = "cuda" if torch.cuda.is_available() else "cpu"
    else:
        device = args.device
    
    print(f"Using device: {device}")
    
    # Load prompt
    prompt_path = Path(args.prompt)
    if prompt_path.exists():
        prompt = prompt_path.read_text()
    else:
        # Treat as direct prompt text
        prompt = args.prompt
    
    print(f"Prompt length: {len(prompt)} characters")
    
    # Load model and tokenizer
    print(f"Loading model: {args.model}")
    hf_token = os.getenv("HF_TOKEN")
    config = AutoConfig.from_pretrained(
        args.model,
        tie_word_embeddings=False,
        token=hf_token
    )
    tokenizer = AutoTokenizer.from_pretrained(args.model, token=hf_token)
    model = AutoModelForCausalLM.from_pretrained(
        args.model,
        config=config,
        token=hf_token
    ).to(device)
    model.eval()
    
    # Set padding token if not set
    if tokenizer.pad_token is None:
        tokenizer.pad_token = tokenizer.eos_token
    
    # Chunk the prompt
    if args.chunk_method == "paragraph":
        chunks = chunk_by_paragraphs(prompt)
    else:
        chunks = chunk_by_sentences(prompt)
    
    print(f"Split prompt into {len(chunks)} chunks using {args.chunk_method} method")
    
    # Run baseline
    print("Running baseline inference...")
    baseline_answer, baseline_answer_tokens, baseline_score = run_baseline(
        model, tokenizer, prompt, args.max_new_tokens, device
    )
    print(f"Baseline answer: {baseline_answer[:100]}...")
    print(f"Baseline score: {baseline_score:.3f}")
    
    # Compute attribution
    chunk_results = compute_attribution(
        model, tokenizer, prompt, chunks,
        baseline_answer_tokens, baseline_score,
        args.max_new_tokens, device
    )
    
    # Print top chunks
    sorted_results = sorted(chunk_results, key=lambda x: x[1], reverse=True)
    print("\nTop 5 most important chunks:")
    for i, ((start, end, chunk_text), importance) in enumerate(sorted_results[:5], 1):
        preview = chunk_text[:60] + "..." if len(chunk_text) > 60 else chunk_text
        print(f"{i}. Importance: {importance:.3f} | {preview}")
    
    # Create visualization
    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    create_heatmap(prompt, chunk_results, str(output_path))
    
    print(f"\nDone! Heatmap saved to {output_path}")


if __name__ == "__main__":
    main()
