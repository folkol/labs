---
name: Attention Visualization Tool
overview: Build a web application that visualizes which input tokens mattered most for LLM inference, demonstrating how context isn't uniformly used. The app will support both chunk-occlusion attribution (works with APIs) and Integrated Gradients (for white-box models), with interactive visualizations including heatmaps and distance plots.
todos:
  - id: setup
    content: "Create project structure: backend/, frontend/ directories, requirements.txt, .gitignore"
    status: pending
  - id: backend-api
    content: Implement FastAPI server with /analyze, /models, /health endpoints
    status: pending
    dependencies:
      - setup
  - id: model-loader
    content: Create model_loader.py to load HuggingFace models and handle inference with logprob extraction
    status: pending
    dependencies:
      - setup
  - id: chunking
    content: Implement chunking.py with paragraph/sentence/token splitting utilities
    status: pending
    dependencies:
      - setup
  - id: chunk-occlusion
    content: Implement chunk-occlusion attribution method in attribution.py with two-stage refinement
    status: pending
    dependencies:
      - backend-api
      - model-loader
      - chunking
  - id: frontend-ui
    content: Create basic frontend UI (index.html, app.js) with input form and result display area
    status: pending
    dependencies:
      - setup
  - id: heatmap-viz
    content: Implement token heatmap visualization with color-coded importance overlay on prompt text
    status: pending
    dependencies:
      - frontend-ui
      - chunk-occlusion
  - id: distance-plot
    content: Add distance plot visualization showing token position vs importance using Chart.js
    status: pending
    dependencies:
      - frontend-ui
      - chunk-occlusion
  - id: integrated-gradients
    content: Implement Integrated Gradients attribution method for white-box models
    status: pending
    dependencies:
      - model-loader
      - chunk-occlusion
  - id: polish
    content: Add error handling, loading states, example prompts, and UI polish
    status: pending
    dependencies:
      - heatmap-viz
      - distance-plot
      - integrated-gradients
---

# Attention Visualization Tool - Implementation Plan

## Overview

Build a web application that visualizes token importance in LLM inference, showing which input tokens were most influential and which were ignored. The tool will demonstrate how AI agents can "forget" rules as context grows larger.

## Architecture

```mermaid
flowchart TD
    Frontend[Frontend UI] -->|HTTP Request| Backend[Backend API]
    Backend -->|Load Model| Model[LLM Model]
    Backend -->|Compute Attribution| Attribution[Attribution Engine]
    Attribution -->|Chunk Occlusion| ChunkOcclusion[Chunk Occlusion]
    Attribution -->|Integrated Gradients| IG[Integrated Gradients]
    Attribution -->|Results| Backend
    Backend -->|JSON Response| Frontend
    Frontend -->|Render| Heatmap[Token Heatmap]
    Frontend -->|Render| DistancePlot[Distance Plot]
    Frontend -->|Render| AnswerView[Answer Comparison]
```

## File Structure

```
visualize-attention-webapp/
├── backend/
│   ├── app.py                 # FastAPI server
│   ├── attribution.py          # Attribution methods (chunk-occlusion, IG)
│   ├── model_loader.py         # Model loading and inference
│   ├── chunking.py             # Text chunking utilities
│   └── requirements.txt        # Python dependencies
├── frontend/
│   ├── index.html              # Main UI
│   ├── app.js                  # Frontend logic
│   ├── styles.css              # Styling
│   └── visualization.js        # Visualization components
├── README.md                   # Project documentation
└── .gitignore
```

## Implementation Details

### Backend (`backend/app.py`)

- FastAPI server with endpoints:
  - `POST /analyze` - Main analysis endpoint
    - Input: `{prompt: str, model_name: str, method: "chunk_occlusion" | "integrated_gradients"}`
    - Output: `{baseline_answer: str, token_importances: List[TokenImportance], attribution_data: dict}`
  - `GET /models` - List available models
  - `GET /health` - Health check

### Attribution Engine (`backend/attribution.py`)

**Chunk Occlusion Method:**

1. Split input into chunks (paragraphs → sentences → tokens)
2. Run baseline inference, record answer and logprobs
3. For each chunk:

   - Remove chunk from prompt
   - Re-run inference
   - Compute importance = baseline_score - occluded_score

4. Use two-stage refinement: coarse chunks first, then refine top chunks

**Integrated Gradients Method:**

1. Load model with gradient access (transformers library)
2. Compute gradients w.r.t. input embeddings for target output tokens
3. Integrate gradients along path from baseline to input
4. Aggregate per-token importance scores

### Model Loader (`backend/model_loader.py`)

- Support for HuggingFace transformers models
- Caching mechanism for model instances
- Inference with logprob extraction
- Configurable temperature (0 for deterministic)

### Frontend (`frontend/index.html` + `frontend/app.js`)

**UI Components:**

1. **Input Panel:**

   - Text area for prompt input
   - Model selector dropdown
   - Attribution method toggle
   - Analyze button

2. **Visualization Panel:**

   - **Token Heatmap:** Color-coded overlay on prompt text showing importance
   - **Distance Plot:** Scatter/line plot of token position vs importance
   - **Answer View:** Side-by-side comparison of baseline vs occluded answers
   - **Chunk List:** Sortable list of chunks with importance bars

3. **Interactive Features:**

   - Click token/chunk → show detailed attribution info
   - Hover → highlight related tokens
   - Toggle between visualization modes

### Visualization (`frontend/visualization.js`)

- Heatmap rendering with color intensity based on importance
- Plot generation using Chart.js or similar
- Token-level highlighting and interaction handlers

## Technical Stack

- **Backend:** Python 3.9+, FastAPI, transformers, torch
- **Frontend:** Vanilla JavaScript (or React if preferred), Chart.js for plots
- **Model:** HuggingFace transformers (e.g., Llama 2/3, Mistral, or smaller models for testing)

## Implementation Steps

1. **Setup project structure** - Create directories and basic files
2. **Backend foundation** - FastAPI server, model loader, basic inference
3. **Chunk occlusion implementation** - Core attribution method
4. **Frontend UI** - Basic input/output interface
5. **Heatmap visualization** - Token-level importance overlay
6. **Distance plot** - Position vs importance visualization
7. **Integrated Gradients** - White-box attribution method
8. **Polish and testing** - Error handling, loading states, example prompts

## Key Design Decisions

- Start with chunk-occlusion as it's more universally applicable and demonstrates the core concept
- Use two-stage chunking (coarse → fine) to balance speed and precision
- Support both methods but make chunk-occlusion the default
- Cache model instances and baseline results to avoid redundant computation
- Use deletion-based occlusion (simplest, most reliable per discussion)

## Dependencies

**Backend:**

- fastapi, uvicorn
- transformers, torch
- numpy, scipy (for IG integration)

**Frontend:**

- Chart.js (or D3.js for more control)
- No framework required for MVP (vanilla JS)

## Future Enhancements (v2+)

- Activation patching / causal tracing
- Per-layer attribution visualization
- Attention vs attribution comparison
- Batch processing for multiple prompts
- Export results as JSON/CSV