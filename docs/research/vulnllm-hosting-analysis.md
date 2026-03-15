# VulnLLM-R-7B Hosting Analysis

**Date:** 2026-03-15
**Author:** Coax Research Team
**Status:** Complete

---

## Executive Summary

**VulnLLM-R-7B** is a specialized vulnerability detection LLM developed by UCSB-SURFI. This analysis evaluates hosting options for integrating VulnLLM-R-7B into Coax's Phase 3 P1 features.

**Key Finding:** VulnLLM-R-7B is **NOT available as a hosted API**. Self-hosting is the only viable option for Phase 3 integration.

---

## Model Overview

| Attribute | Details |
|-----------|---------|
| **Model Name** | VulnLLM-R-7B |
| **Full Title** | VulnLLM-R: Specialized Reasoning LLM for Vulnerability Detection |
| **Model Type** | Text Generation / Code Analysis LLM |
| **Parameters** | 8B (marketed as 7B class) |
| **Base Model** | Qwen/Qwen2.5-7B → Qwen2.5-7B-Instruct → VulnLLM-R-7B |
| **License** | Apache-2.0 ✅ (commercial use allowed) |
| **Architecture** | Qwen2 |
| **Tensor Format** | Safetensors (BF16) |
| **Chat Template** | Available |

### Key Features

- **Reasoning-Based Detection**: Generates Chain-of-Thought analysis for vulnerabilities
- **Superior Accuracy**: Outperforms Claude-3.7-Sonnet, o3-mini, CodeQL, AFL++ on benchmarks
- **Efficiency**: 30x smaller than general-purpose reasoning models
- **Broad Coverage**: C, C++, Python, Java (zero-shot generalization)
- **Specialized Tags**: `security`, `vulnerability-detection`, `code-analysis`, `reasoning`, `llm`, `conversational`

### Performance Benchmarks

| Benchmark | VulnLLM-R-7B | Baseline | Improvement |
|-----------|--------------|----------|-------------|
| **PrimeVul (Python)** | F1: 0.723 | CodeQL: 0.521 | +38.8% |
| **PrimeVul (C/C++)** | F1: 0.737 | CodeQL: 0.548 | +34.5% |
| **PrimeVul (Java)** | F1: 0.870 | CodeQL: 0.612 | +42.2% |
| **OOD Generalization (C)** | +5.29% | Baseline: -36.9% | +42.19% |

**Zero-Day Discoveries:** 15 undisclosed vulnerabilities found in 5 active repositories during research.

---

## API Availability

| Service | Status | Notes |
|---------|--------|-------|
| **Hugging Face Inference API** | ❌ Not Deployed | Model page states: "This model isn't deployed by any Inference Provider" |
| **Hugging Face Inference Endpoints** | ❌ Not Deployed | Would require manual deployment |
| **Nvidia NGC/NIM** | ❌ Not Available | Not listed in Nvidia catalog |
| **Together AI** | ❌ Not Available | Not in model catalog |
| **Sambanova Cloud** | ❌ Not Available | Proprietary models only |
| **Replicate** | ❌ Not Available | Would require custom deployment |
| **Modal** | ⚠️ Self-Deploy | Can deploy custom models (see self-hosting section) |

---

## Hosting Options Comparison

### Option 1: Hugging Face Inference API (Free Tier)

| Aspect | Details |
|--------|---------|
| **Availability** | ❌ Model not deployed |
| **Free Tier** | N/A |
| **Rate Limits** | N/A |
| **Pricing** | N/A |
| **Setup Complexity** | N/A |
| **Recommendation** | Not viable - model not available |

**Hugging Face Free Tier (General):**
- Rate-limited, cold starts on unpopular models
- Limited to models under ~10B parameters on free tier
- Estimated cost for dedicated endpoint: $1,500-2,000/month
- Variable latency: 200ms-2s
- No volume discounts on community models

---

### Option 2: Hugging Face Inference Endpoints (Paid)

| Aspect | Details |
|--------|---------|
| **Availability** | ⚠️ Manual deployment required |
| **Free Tier** | $0.10 monthly credit cap (insufficient) |
| **Pay-As-You-Go** | Yes, but no free tier + pay-as-you-go combination |
| **Estimated Cost** | $500-1,500/month (depends on usage) |
| **Setup Complexity** | Medium (requires HF account + deployment) |
| **Recommendation** | Viable but expensive for testing |

---

### Option 3: Nvidia NGC/NIM

| Aspect | Details |
|--------|---------|
| **Availability** | ❌ VulnLLM-R-7B not in catalog |
| **Free Tier** | Some models have free tier |
| **Pricing** | Pay-per-token or subscription |
| **Setup Complexity** | Low (if available) |
| **Recommendation** | Not viable - model not available |

---

### Option 4: Together AI

| Aspect | Details |
|--------|---------|
| **Availability** | ❌ VulnLLM-R-7B not in catalog |
| **Pricing Model** | Pay-as-you-go (good for variable traffic) |
| **Custom Models** | Fine-tuning supported, not custom uploads |
| **Setup Complexity** | Low (if available) |
| **Recommendation** | Not viable - model not available |

---

### Option 5: Sambanova Cloud

| Aspect | Details |
|--------|---------|
| **Availability** | ❌ Proprietary models only |
| **Pricing** | 3X lower TCO claimed (vs. GPU clouds) |
| **Free Tier** | Not publicly disclosed |
| **Setup Complexity** | Medium |
| **Recommendation** | Not viable - doesn't support custom models |

---

### Option 6: Replicate

| Aspect | Details |
|--------|---------|
| **Availability** | ⚠️ Custom deployment possible |
| **Pricing Model** | Pay-per-second GPU time |
| **GPU Options** | T4: $0.000225/sec, A40, A100 available |
| **Estimated Cost** | $50-200/month (light usage) |
| **Setup Complexity** | Medium-High (requires Docker deployment) |
| **Recommendation** | Viable for production, overkill for testing |

**Replicate Cost Estimate:**
```
RTX 4090: ~$0.34/hour
8 hours/day testing: $2.72/day = ~$82/month
24/7 production: $19.72/day = ~$592/month
```

---

### Option 7: Modal.com

| Aspect | Details |
|--------|---------|
| **Availability** | ✅ Supports custom model deployment |
| **Free Tier** | $30/month compute credits |
| **Pricing** | Pay per second: CPU $0.00003942/core/sec, GPU varies |
| **GPU Options** | T4, A10G, A100, H100 |
| **Setup Complexity** | Medium (Python-based deployment) |
| **Recommendation** | ✅ BEST CLOUD OPTION for testing |

**Modal Cost Estimate:**
```
Starter Plan: $0/month + $30 free credits
GPU (T4): ~$0.0003/sec = ~$1.08/hour
8 hours/day testing: $8.64/day = ~$259/month (after free credits)
Weekend testing only: ~$50-75/month
```

**Modal Advantages:**
- $30/month free credits (good for 27+ hours of GPU time)
- Simple Python deployment scripts
- Automatic scaling
- No minimum commitment

---

## Self-Hosting Analysis

### GPU Requirements

| Precision | VRAM Required | GPU Options |
|-----------|---------------|-------------|
| **FP16 (Full)** | 14-16 GB | RTX 3090 (24GB), RTX 4090 (24GB), A10G (24GB) |
| **INT8 (Quantized)** | 8-10 GB | RTX 3080 (10GB), RTX 4070 Ti (12GB) |
| **INT4 (Quantized)** | 6-8 GB | RTX 3060 (12GB), RTX 4060 Ti (16GB) |

**Recommended:** INT4 quantization for testing (7-8GB VRAM), FP16 for production.

---

### Consumer GPU Options

| GPU | VRAM | Price (Used) | Price (New) | Suitable For |
|-----|------|--------------|-------------|--------------|
| **RTX 3090** | 24GB | $700-850 | N/A | FP16 production |
| **RTX 4090** | 24GB | N/A | $1,600-2,000 | FP16 production |
| **RTX 3080** | 10GB | $400-500 | N/A | INT8 testing |
| **RTX 4070 Ti** | 12GB | N/A | $800-900 | INT8 testing |
| **RTX 3060** | 12GB | $200-250 | $290-330 | INT4 testing |
| **RTX 4060 Ti** | 16GB | N/A | $400-450 | INT4/INT8 testing |

**Best Value for Testing:** RTX 3060 12GB ($200-250 used)

---

### Cloud GPU Pricing

| Provider | GPU | Price/Hour | Monthly (24/7) | Monthly (8hr/day) |
|----------|-----|------------|----------------|-------------------|
| **RunPod** | RTX 3090 | $0.34-0.40 | ~$245-288 | ~$82-96 |
| **RunPod** | RTX 4090 | $0.34-0.50 | ~$245-360 | ~$82-120 |
| **RunPod** | A100 40GB | $0.60-0.80 | ~$432-576 | ~$144-192 |
| **Vast.ai** | RTX 3090 | $0.25-0.35 | ~$180-252 | ~$60-84 |
| **Vast.ai** | RTX 4090 | $0.35-0.45 | ~$252-324 | ~$84-108 |
| **Vast.ai** | A100 80GB | $0.52-0.70 | ~$374-504 | ~$125-168 |
| **Lambda Labs** | A100 40GB | $0.80-1.00 | ~$576-720 | ~$192-240 |
| **Lambda Labs** | H100 80GB | $1.80-2.20 | ~$1,296-1,584 | ~$432-528 |

**Best Value:** Vast.ai RTX 3090 ($0.25-0.35/hr)

---

### vLLM Deployment

**vLLM** is the recommended inference engine for VulnLLM-R-7B.

**Installation:**
```bash
pip install vllm
```

**Single GPU Deployment:**
```bash
python -m vllm.entrypoints.api_server \
    --model UCSB-SURFI/VulnLLM-R-7B \
    --tensor-parallel-size 1 \
    --dtype bfloat16 \
    --max-model-len 16384 \
    --port 8000
```

**Multi-GPU Deployment:**
```bash
python -m vllm.entrypoints.api_server \
    --model UCSB-SURFI/VulnLLM-R-7B \
    --tensor-parallel-size 2 \
    --dtype bfloat16 \
    --max-model-len 32768 \
    --port 8000
```

**Docker Deployment:**
```bash
docker run --gpus all \
    -p 8000:8000 \
    -v ~/.cache/huggingface:/root/.cache/huggingface \
    vllm/vllm-openai:latest \
    --model UCSB-SURFI/VulnLLM-R-7B \
    --tensor-parallel-size 1 \
    --dtype bfloat16
```

**Performance Estimates:**
- 7B model on RTX 4090: ~30-50 tokens/second (GPU-only)
- 7B model on RTX 3060: ~15-25 tokens/second (GPU-only)
- With CPU offload: ~1-5 tokens/second (not recommended)

---

### Self-Hosting Cost Breakdown

#### Scenario 1: Home Lab (One-Time Purchase)

| Component | Cost | Notes |
|-----------|------|-------|
| RTX 3060 12GB | $200-250 | Used market |
| Compatible PSU (650W+) | $50-100 | If needed |
| System (if building) | $300-500 | CPU, RAM, motherboard |
| **Total** | **$550-850** | One-time cost |

**Operating Costs:**
- Electricity: ~$15-25/month (8 hours/day)
- Internet: Existing
- **Total Monthly:** ~$15-25

**Break-even:** 24-36 months vs. cloud hosting

---

#### Scenario 2: Cloud GPU (Vast.ai RTX 3090)

| Usage | Hours/Month | Cost/Month |
|-------|-------------|------------|
| Testing (weekends only) | 64 | ~$16-22 |
| Part-time (8hr/day) | 240 | ~$60-84 |
| Full-time (24/7) | 720 | ~$180-252 |

**Recommendation:** Start with weekend testing (~$20/month)

---

#### Scenario 3: Modal.com (Recommended for Testing)

| Usage | Hours/Month | Cost/Month |
|-------|-------------|------------|
| Testing (with $30 free credits) | 27 | $0 (covered by credits) |
| Part-time (8hr/day) | 240 | ~$230-260 (after credits) |
| Weekend only | 64 | ~$40-50 (after credits) |

**Recommendation:** Use free credits for initial testing

---

## Integration Approach for Coax

### Architecture

```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Coax Scanner   │────▶│  LLM Client      │────▶│  VulnLLM-R-7B   │
│  (regex/entropy)│     │  (HTTP/gRPC)     │     │  (vLLM/Modal)   │
└─────────────────┘     └──────────────────┘     └─────────────────┘
                               │
                               ▼
                        ┌──────────────────┐
                        │  Prompt Template │
                        │  + Context Slice │
                        └──────────────────┘
```

### Implementation Steps

1. **Week 1: Setup Hosting**
   - Deploy VulnLLM-R-7B on Modal (use $30 free credits)
   - Test basic API connectivity
   - Benchmark latency and throughput

2. **Week 2: LLM Client Module**
   - Implement HTTP client for vLLM API
   - Add retry logic with exponential backoff
   - Implement response caching

3. **Week 3: Prompt Engineering**
   - Design vulnerability analysis prompts
   - Test with known vulnerable code (Juliet, PrimeVul)
   - Optimize for accuracy vs. token cost

4. **Week 4: Integration**
   - Add `--llm` flag to CLI
   - Implement slice-based code context
   - Correlate LLM findings with regex detections

---

## Cost-Benefit Analysis

### Development/Testing Phase (3 months)

| Option | Setup Cost | Monthly Cost | 3-Month Total |
|--------|------------|--------------|---------------|
| **Modal (free credits)** | $0 | $0-50 | $0-150 |
| **Vast.ai (weekend)** | $0 | $20 | $60 |
| **Home lab (RTX 3060)** | $550-850 | $15-25 | $595-925 |
| **RunPod (part-time)** | $0 | $82-96 | $246-288 |

**Recommendation:** Modal for testing (lowest upfront cost)

---

### Production Phase (12 months)

| Option | Setup Cost | Monthly Cost | 12-Month Total |
|--------|------------|--------------|----------------|
| **Home lab (RTX 4090)** | $1,600-2,000 | $25-35 | $1,900-2,420 |
| **Vast.ai (24/7)** | $0 | $180-252 | $2,160-3,024 |
| **Modal (24/7)** | $0 | $500-700 | $6,000-8,400 |
| **Lambda Labs (A100)** | $0 | $576-720 | $6,912-8,640 |

**Recommendation:** Home lab for production (break-even at ~10 months)

---

## Recommendations

### Phase 3 P1 (Testing/Development)

| Priority | Recommendation | Rationale |
|----------|----------------|-----------|
| **P0** | Use Modal.com with free credits | $0 upfront, simple deployment |
| **P1** | Test with INT4 quantization | Reduces VRAM requirements to 6-8GB |
| **P2** | Limit to weekend testing | Keeps costs under $50/month |

### Production Deployment

| Priority | Recommendation | Rationale |
|----------|----------------|-----------|
| **P0** | Purchase RTX 4090 workstation | Best performance/cost for 24/7 |
| **P1** | Deploy vLLM with tensor parallelism | Optimize throughput |
| **P2** | Implement request caching | Reduce redundant API calls |

---

## Implementation Complexity

| Aspect | Complexity | Notes |
|--------|------------|-------|
| **Modal Deployment** | Low-Medium | Python scripts, good documentation |
| **vLLM Setup** | Low | Well-documented, Docker support |
| **LLM Client (Rust)** | Medium | HTTP client, retry logic, caching |
| **Prompt Engineering** | Medium-High | Requires iteration for accuracy |
| **Slice-based Context** | High | Requires CFG implementation |

**Estimated Effort:**
- Modal deployment: 1-2 days
- vLLM setup: 1 day
- LLM client: 3-5 days
- Prompt engineering: 1-2 weeks
- Full integration: 2-3 weeks

---

## Conclusion

**Best Option for Phase 3 P1:** Modal.com

- ✅ $30/month free credits (covers initial testing)
- ✅ Simple Python deployment
- ✅ No upfront hardware cost
- ✅ Scalable to production
- ⚠️ More expensive than self-hosting long-term

**Transition Path:**
1. Start with Modal (testing)
2. Validate accuracy and utility
3. Purchase RTX 4090 for production
4. Migrate deployment to local vLLM

**Estimated Timeline:**
- Week 1-2: Modal deployment + testing
- Week 3-4: LLM client implementation
- Week 5-6: Integration with Coax scanner
- Week 7-8: Accuracy validation + optimization

---

## References

- **Model Card:** https://huggingface.co/UCSB-SURFI/VulnLLM-R-7B
- **Paper:** https://arxiv.org/abs/2512.07533
- **vLLM Docs:** https://docs.vllm.ai/
- **Modal Pricing:** https://modal.com/pricing
- **Vast.ai:** https://vast.ai/
- **RunPod:** https://runpod.io/

---

*Analysis completed: 2026-03-15*
