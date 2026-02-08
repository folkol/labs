# Visualize which input tokens mattered most for inference

Show heat map for which input tokens were mostly ignored by the LLM.

## Overview

This tool uses chunk-occlusion attribution to visualize which parts of the input context were most important for the model's output. It demonstrates how context isn't uniformly used—some input chunks are ignored while others are critical.

## Installation

```bash
pip install -r requirements.txt
```

## Usage

Run the attribution script with a prompt file:

```bash
python scripts/attribution_image.py \
    --prompt data/example_prompt.txt \
    --model microsoft/DialoGPT-small \
    --output outputs/heatmap.png \
    --max-new-tokens 30 \
    --chunk-method paragraph
```

### Arguments

- `--prompt`: Path to prompt text file, or prompt text directly
- `--model`: HuggingFace model name (default: `microsoft/DialoGPT-small`)
- `--output`: Output path for heatmap image (default: `outputs/heatmap.png`)
- `--max-new-tokens`: Maximum tokens to generate (default: 30)
- `--chunk-method`: Chunking method: `paragraph` or `sentence` (default: `paragraph`)
- `--device`: Device to use (`cuda`/`cpu`). Auto-detected if not specified

### Example

```bash
# Run with example prompt
python scripts/attribution_image.py --prompt data/example_prompt.txt

# Use a different model
python scripts/attribution_image.py \
    --prompt data/example_prompt.txt \
    --model gpt2

# Use sentence-level chunking for finer granularity
python scripts/attribution_image.py \
    --prompt data/example_prompt.txt \
    --chunk-method sentence
```

## How It Works

1. **Baseline**: Run the model on the full prompt and record the answer and logprob scores
2. **Occlusion**: For each chunk (paragraph or sentence), remove it and re-run the model
3. **Scoring**: Compute importance as `baseline_score - occluded_score`
4. **Visualization**: Generate a heatmap showing importance scores overlaid on the original prompt

The heatmap uses color intensity to show importance:
- **Green**: Low importance (chunk was largely ignored)
- **Orange**: Medium importance
- **Red**: High importance (chunk was critical for the output)

## Output

The script generates a PNG image showing:
- The original prompt text with color-coded importance overlay
- Summary statistics (max, min, mean importance scores)
- Top 5 most important chunks printed to console
