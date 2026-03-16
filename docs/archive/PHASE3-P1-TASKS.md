# Phase 3 P1 Implementation Tasks

**Date:** 2026-03-15
**Status:** Planning
**Timeline:** 10-12 weeks

---

## Executive Summary

Phase 3 P1 focuses on transforming Coax from a pattern-based scanner to an intelligent vulnerability detection platform. Four major feature areas are planned:

1. **VulnLLM-R-7B Integration** - AI-powered vulnerability analysis
2. **TUI Dashboard** - Interactive terminal interface
3. **CFG-Based Slicing** - Control flow graph analysis
4. **Threat Model Integration** - STRIDE-based threat modeling

---

## P1.1: VulnLLM-R-7B Integration

**Priority:** P1 (High)
**Estimated Effort:** 3-4 weeks
**Dependencies:** None (can start immediately)
**Owner:** TBD

### Research Status

- ✅ Hosting options analyzed (see `docs/research/vulnllm-hosting-analysis.md`)
- ✅ Model specifications documented
- ✅ Cost estimates completed
- ✅ Integration approach defined

**Key Finding:** VulnLLM-R-7B is NOT available as hosted API. Self-hosting required.

### Recommended Approach

| Aspect | Decision |
|--------|----------|
| **Hosting** | Modal.com (testing) → Self-hosted RTX 4090 (production) |
| **Inference Engine** | vLLM |
| **Quantization** | INT4 for testing (6-8GB VRAM), FP16 for production |
| **Budget** | $30-50/month (testing with Modal free credits) |

---

### Tasks

#### P1.1.1: Research Hosting Options ✅
- [x] Research Hugging Face hosting options
- [x] Research Nvidia NGC/NIM
- [x] Research Modal.com
- [x] Research Together AI
- [x] Research Sambanova Cloud
- [x] Research Replicate
- [x] Analyze self-hosting costs
- [x] Create hosting analysis document

**Status:** ✅ COMPLETE
**Deliverable:** `docs/research/vulnllm-hosting-analysis.md`

---

#### P1.1.2: Set Up Hosting
- [ ] Create Modal.com account
- [ ] Deploy VulnLLM-R-7B on Modal
- [ ] Test API connectivity
- [ ] Benchmark latency and throughput
- [ ] Document deployment process

**Estimated Effort:** 2-3 days
**Deliverable:** Working VulnLLM-R-7B endpoint on Modal

---

#### P1.1.3: Implement LLM Client Module
- [ ] Create `crates/coax-llm` crate
- [ ] Implement HTTP client for vLLM API
- [ ] Add retry logic with exponential backoff
- [ ] Implement response caching (Redis or in-memory)
- [ ] Add rate limiting
- [ ] Write unit tests (10+ tests)

**Estimated Effort:** 4-5 days
**Deliverable:** `coax-llm` crate with working client

---

#### P1.1.4: Create Prompt Templates
- [ ] Design vulnerability analysis prompt template
- [ ] Design code slice analysis prompt
- [ ] Design remediation recommendation prompt
- [ ] Test prompts with VulnLLM-R-7B
- [ ] Optimize for accuracy vs. token cost
- [ ] Create prompt versioning system

**Estimated Effort:** 5-7 days
**Deliverable:** Validated prompt library

**Example Prompt:**
```
You are VulnLLM-R-7B, a specialized vulnerability detection model.
Analyze this code slice step-by-step:

```{language}
{code_slice}
```

Provide your analysis in JSON format:
{{
  "vulnerability_type": "...",
  "cwe_id": "CWE-XXX",
  "severity": "Critical|High|Medium|Low",
  "confidence": 0.0-1.0,
  "explanation": "...",
  "remediation": "..."
}}
```

---

#### P1.1.5: Integrate with Scanner
- [ ] Add `--llm` flag to CLI
- [ ] Implement slice extraction from findings
- [ ] Add LLM analysis pipeline
- [ ] Correlate LLM findings with regex detections
- [ ] Add `--llm-only` flag (LLM analysis only)
- [ ] Add `--llm-enhance` flag (enhance regex findings with LLM)

**Estimated Effort:** 5-7 days
**Deliverable:** Integrated LLM scanning

---

#### P1.1.6: Test with Real Codebases
- [ ] Test with Juliet Test Suite
- [ ] Test with PrimeVul dataset
- [ ] Test with internal codebases
- [ ] Measure accuracy (precision, recall, F1)
- [ ] Measure latency per analysis
- [ ] Measure cost per analysis

**Estimated Effort:** 3-4 days
**Deliverable:** Accuracy and performance benchmarks

---

#### P1.1.7: Benchmark Cost/Accuracy
- [ ] Track API costs
- [ ] Track token usage
- [ ] Calculate cost per finding
- [ ] Optimize prompt length
- [ ] Implement caching strategy
- [ ] Create cost dashboard

**Estimated Effort:** 2-3 days
**Deliverable:** Cost optimization report

---

### P1.1 Summary

| Metric | Target | Status |
|--------|--------|--------|
| **Effort** | 3-4 weeks | ⏳ TODO |
| **Accuracy** | F1 > 0.70 | ⏳ TBD |
| **Latency** | <5s per slice | ⏳ TBD |
| **Cost** | <$50/month (testing) | ⏳ TBD |

---

## P1.2: TUI Dashboard

**Priority:** P1 (High)
**Estimated Effort:** 5-6 weeks
**Dependencies:** None (can start immediately)
**Owner:** TBD

### Research Status

- ✅ TUI frameworks compared (see `docs/research/tui-dashboard-design.md`)
- ✅ Mockup designs completed
- ✅ Feature priorities defined
- ✅ Implementation approach documented

**Recommended Framework:** Ratatui (Rust native)

---

### Tasks

#### P1.2.1: Select TUI Framework ✅
- [x] Research Ratatui
- [x] Research TUI React
- [x] Research Blessed-contrib
- [x] Compare features and performance
- [x] Select Ratatui

**Status:** ✅ COMPLETE
**Deliverable:** Framework decision documented

---

#### P1.2.2: Create coax-tui Crate
- [ ] Create `crates/coax-tui` directory
- [ ] Add Cargo.toml with dependencies
- [ ] Set up module structure
- [ ] Add Ratatui and Crossterm dependencies
- [ ] Create basic main.rs

**Estimated Effort:** 0.5 days
**Deliverable:** Working crate structure

---

#### P1.2.3: Design Main Dashboard Layout
- [ ] Implement header with branding
- [ ] Implement repository info panel
- [ ] Implement scan results summary (severity counts)
- [ ] Implement recent findings list
- [ ] Implement quick actions panel
- [ ] Implement status bar with keyboard hints

**Estimated Effort:** 3-4 days
**Deliverable:** Main dashboard view

---

#### P1.2.4: Implement Finding List View
- [ ] Implement scrollable finding list
- [ ] Add severity icons (🚨 ⚠️ ⚡ ℹ️)
- [ ] Add file and line information
- [ ] Add verification status
- [ ] Add pagination (showing X of Y)

**Estimated Effort:** 2-3 days
**Deliverable:** Finding list view

---

#### P1.2.5: Implement Finding Detail View
- [ ] Implement code preview panel (4 lines around match)
- [ ] Add context analysis display
- [ ] Add recommendation display
- [ ] Add action buttons (Ignore, FP, Verify, etc.)
- [ ] Implement scroll navigation

**Estimated Effort:** 3-4 days
**Deliverable:** Finding detail view

---

#### P1.2.6: Add Search/Filter Functionality
- [ ] Implement fuzzy search (/)
- [ ] Implement severity filter
- [ ] Implement pattern filter
- [ ] Implement file filter
- [ ] Implement status filter (New, Verified, Ignored)
- [ ] Implement clear filters

**Estimated Effort:** 3-4 days
**Deliverable:** Full filtering capability

---

#### P1.2.7: Add Baseline Management
- [ ] Implement baseline view
- [ ] Show new/resolved/unchanged findings
- [ ] Add update baseline action
- [ ] Add create baseline action
- [ ] Add export baseline action
- [ ] Show baseline statistics

**Estimated Effort:** 3-4 days
**Deliverable:** Baseline management panel

---

#### P1.2.8: Add Watch Mode
- [ ] Implement file system watcher (notify crate)
- [ ] Add auto-rescan on file change
- [ ] Add notification for new findings
- [ ] Implement scan interval configuration
- [ ] Add watch mode indicator

**Estimated Effort:** 3-4 days
**Deliverable:** Real-time watch mode

---

#### P1.2.9: Add Settings Panel
- [ ] Implement settings view
- [ ] Add scanner configuration options
- [ ] Add threshold configuration
- [ ] Add display options (theme, colors)
- [ ] Add LLM configuration
- [ ] Implement save/reset actions

**Estimated Effort:** 2-3 days
**Deliverable:** Settings panel

---

#### P1.2.10: Add Trend Charts (P2 Stretch)
- [ ] Implement ASCII chart rendering
- [ ] Add findings over time chart
- [ ] Add severity distribution chart
- [ ] Add zoom/pan functionality
- [ ] Add statistics panel

**Estimated Effort:** 3-4 days
**Deliverable:** Trend visualization

---

#### P1.2.11: Testing and Polish
- [ ] Test on iTerm2 (macOS)
- [ ] Test on Windows Terminal
- [ ] Test on Linux terminals
- [ ] Performance optimization (60 FPS)
- [ ] Memory optimization (<50MB)
- [ ] Write documentation

**Estimated Effort:** 3-4 days
**Deliverable:** Production-ready TUI

---

### P1.2 Summary

| Metric | Target | Status |
|--------|--------|--------|
| **Effort** | 5-6 weeks | ⏳ TODO |
| **Performance** | 60 FPS | ⏳ TBD |
| **Memory** | <50MB | ⏳ TBD |
| **Terminal Support** | macOS, Linux, Windows | ⏳ TBD |

---

## P1.3: CFG-Based Slicing

**Priority:** P1 (High)
**Estimated Effort:** 7-8 weeks
**Dependencies:** None (can start immediately)
**Owner:** TBD

### Research Status

- ✅ Existing implementations analyzed (see `docs/research/cfg-slicing-research.md`)
- ✅ CFG construction approach defined
- ✅ Slicing algorithm designed
- ✅ Integration plan documented

**Recommended Approach:** tree-sitter + oxc_cfg (Rust native)

---

### Tasks

#### P1.3.1: Research CFG Construction ✅
- [x] Research Semgrep approach
- [x] Research CodeQL approach
- [x] Research Joern approach
- [x] Analyze oxc_cfg crate
- [x] Analyze tree-sitter integration
- [x] Create CFG slicing research document

**Status:** ✅ COMPLETE
**Deliverable:** `docs/research/cfg-slicing-research.md`

---

#### P1.3.2: Select tree-sitter + CFG Crate
- [ ] Add tree-sitter dependency
- [ ] Add tree-sitter-rust, tree-sitter-python, tree-sitter-javascript
- [ ] Add oxc_cfg dependency
- [ ] Add oxc_ast dependency
- [ ] Test basic parsing

**Estimated Effort:** 1 day
**Deliverable:** Dependencies configured

---

#### P1.3.3: Implement CFG Builder
- [ ] Create `crates/coax-cfg` crate
- [ ] Implement AST extraction from tree-sitter
- [ ] Implement CFG construction with oxc_cfg
- [ ] Add basic block representation
- [ ] Add edge representation
- [ ] Implement CFG visualization (debug)
- [ ] Write unit tests (15+ tests)

**Estimated Effort:** 5-7 days
**Deliverable:** Working CFG builder

---

#### P1.3.4: Identify Entry Points
- [ ] Implement HTTP route detection (Axum, Actix, Rocket)
- [ ] Implement CLI command detection (clap)
- [ ] Implement public function detection
- [ ] Implement API endpoint detection
- [ ] Implement event handler detection
- [ ] Create entry point database

**Estimated Effort:** 4-5 days
**Deliverable:** Entry point detection

---

#### P1.3.5: Identify Sink Points
- [ ] Implement SQL execution sink detection
- [ ] Implement command execution sink detection
- [ ] Implement file I/O sink detection
- [ ] Implement network sink detection
- [ ] Implement crypto sink detection
- [ ] Implement serialization sink detection
- [ ] Create sink point database

**Estimated Effort:** 4-5 days
**Deliverable:** Sink point detection

---

#### P1.3.6: Implement Backward Slicer
- [ ] Implement backward slice algorithm
- [ ] Implement variable definition tracking
- [ ] Implement variable use tracking
- [ ] Implement data flow analysis
- [ ] Add slice visualization (debug)
- [ ] Write unit tests (10+ tests)

**Estimated Effort:** 5-7 days
**Deliverable:** Working backward slicer

---

#### P1.3.7: Implement Forward Slicer
- [ ] Implement forward slice algorithm
- [ ] Integrate with backward slicer
- [ ] Implement slice intersection
- [ ] Extract vulnerability paths
- [ ] Add slice visualization (debug)
- [ ] Write unit tests (10+ tests)

**Estimated Effort:** 5-7 days
**Deliverable:** Working forward slicer

---

#### P1.3.8: Integrate with Scanner
- [ ] Add `--cfg` flag to CLI
- [ ] Implement CFG-based analysis pipeline
- [ ] Correlate CFG findings with regex findings
- [ ] Add `--cfg-only` flag
- [ ] Add `--slice` flag (show vulnerability slice)
- [ ] Integrate with output formatting

**Estimated Effort:** 4-5 days
**Deliverable:** CFG-integrated scanner

---

#### P1.3.9: Test with Vulnerability Datasets
- [ ] Test with Juliet Test Suite
- [ ] Test with PrimeVul dataset
- [ ] Measure precision/recall/F1
- [ ] Compare with regex-only detection
- [ ] Document false positives/negatives
- [ ] Optimize detection rules

**Estimated Effort:** 4-5 days
**Deliverable:** Accuracy benchmarks

---

### P1.3 Summary

| Metric | Target | Status |
|--------|--------|--------|
| **Effort** | 7-8 weeks | ⏳ TODO |
| **Accuracy** | F1 > 0.75 | ⏳ TBD |
| **Language Support** | Rust, Python, JavaScript | ⏳ TBD |
| **Performance** | <10s per file | ⏳ TBD |

---

## P1.4: Threat Model Integration

**Priority:** P1 (High)
**Estimated Effort:** 3-4 weeks
**Dependencies:** None (can start immediately)
**Owner:** TBD

### Research Status

- ✅ Integration approach defined (see `docs/research/threat-model-integration.md`)
- ✅ Command design completed
- ✅ Output format specified
- ✅ Risk scoring algorithm designed

**Recommended Approach:** Create `opendev-threat-model` crate

---

### Tasks

#### P1.4.1: Import/Create opendev-threat-model
- [ ] Create `crates/opendev-threat-model` crate
- [ ] Implement STRIDE category enum
- [ ] Implement RiskLevel enum
- [ ] Implement Threat struct
- [ ] Implement Component struct
- [ ] Implement ThreatModel struct
- [ ] Implement ThreatModelBuilder
- [ ] Write unit tests (15+ tests)

**Estimated Effort:** 4-5 days
**Deliverable:** Working threat model crate

---

#### P1.4.2: Add `coax threat-model` Command
- [ ] Add ThreatModel subcommand to CLI
- [ ] Implement `generate` action
- [ ] Implement `correlate` action
- [ ] Implement `summary` action
- [ ] Add help documentation
- [ ] Write integration tests

**Estimated Effort:** 3-4 days
**Deliverable:** CLI command

---

#### P1.4.3: Implement Correlation Engine
- [ ] Create finding-to-threat mapping
- [ ] Implement STRIDE category assignment
- [ ] Implement risk level assignment
- [ ] Implement CWE ID mapping
- [ ] Add correlation confidence scoring
- [ ] Write unit tests (10+ tests)

**Estimated Effort:** 3-4 days
**Deliverable:** Correlation engine

---

#### P1.4.4: Add Risk Scoring
- [ ] Implement risk score calculation
- [ ] Implement category-based weighting
- [ ] Implement severity multipliers
- [ ] Implement context factors
- [ ] Implement risk recommendation engine
- [ ] Write unit tests (10+ tests)

**Estimated Effort:** 3-4 days
**Deliverable:** Risk scoring system

---

#### P1.4.5: Generate Enhanced Reports
- [ ] Implement YAML output format
- [ ] Implement JSON output format
- [ ] Implement ASCII DFD generation
- [ ] Add threat summary view
- [ ] Add recommendation list
- [ ] Write integration tests

**Estimated Effort:** 4-5 days
**Deliverable:** Report generation

---

#### P1.4.6: Test with Real Codebases
- [ ] Test with internal codebases
- [ ] Test with open source projects
- [ ] Validate STRIDE categorization
- [ ] Validate risk scores
- [ ] Gather user feedback
- [ ] Document use cases

**Estimated Effort:** 3-4 days
**Deliverable:** Validation report

---

### P1.4 Summary

| Metric | Target | Status |
|--------|--------|--------|
| **Effort** | 3-4 weeks | ⏳ TODO |
| **STRIDE Coverage** | 6 categories | ⏳ TBD |
| **Output Formats** | YAML, JSON, ASCII DFD | ⏳ TBD |
| **Risk Score Accuracy** | >80% alignment | ⏳ TBD |

---

## Dependencies

```
┌─────────────────────────────────────────────────────────┐
│                    Phase 3 P1 Timeline                  │
└─────────────────────────────────────────────────────────┘

Week 1-2:  P1.1.2 Set Up Hosting
           P1.2.2-4 TUI Basic Views
           P1.3.2-3 CFG Builder
           P1.4.1 Threat Model Crate

Week 3-4:  P1.1.3-4 LLM Client + Prompts
           P1.2.5-7 TUI Detail + Filter
           P1.3.4-5 Entry/Sink Detection
           P1.4.2-3 CLI + Correlation

Week 5-6:  P1.1.5-6 Scanner Integration + Testing
           P1.2.8-10 TUI Watch + Settings
           P1.3.6-7 Slicing Engine
           P1.4.4-5 Risk Scoring + Reports

Week 7-8:  P1.1.7 Benchmark
           P1.2.11 TUI Polish
           P1.3.8-9 Scanner Integration + Testing
           P1.4.6 Validation

Week 9-10: Integration Testing
           Documentation
           Bug Fixes
           Release Preparation

Week 11-12: Buffer / Stretch Goals
```

---

## Critical Path

```
P1.3 CFG Slicing (7-8 weeks) ──┬──▶ P1.1 LLM Integration
                                │
P1.2 TUI Dashboard (5-6 weeks) ─┤──▶ Final Integration
                                │
P1.4 Threat Model (3-4 weeks) ──┘
```

**Critical Path:** CFG Slicing → LLM Integration → Final Integration

---

## Estimated Timeline Summary

| Feature | Effort | Start | End |
|---------|--------|-------|-----|
| **P1.1 VulnLLM-R-7B** | 3-4 weeks | Week 1 | Week 4 |
| **P1.2 TUI Dashboard** | 5-6 weeks | Week 1 | Week 6 |
| **P1.3 CFG Slicing** | 7-8 weeks | Week 1 | Week 8 |
| **P1.4 Threat Model** | 3-4 weeks | Week 1 | Week 4 |
| **Integration** | 2 weeks | Week 9 | Week 10 |
| **Buffer** | 2 weeks | Week 11 | Week 12 |

**Total Timeline:** 12 weeks

---

## Resource Requirements

| Resource | Quantity | Notes |
|----------|----------|-------|
| **Developers** | 2-3 | Parallel feature development |
| **GPU (Testing)** | 1x RTX 3060+ or Modal credits | VulnLLM-R-7B testing |
| **GPU (Production)** | 1x RTX 4090 | For self-hosting |
| **Budget** | $50-100/month | Cloud GPU + API costs |

---

## Success Criteria

### End of Phase 3 P1

- [ ] `coax scan --llm` works with VulnLLM-R-7B
- [ ] `coax-tui` launches interactive dashboard
- [ ] `coax scan --cfg` performs CFG-based analysis
- [ ] `coax threat-model generate` creates threat models
- [ ] All features integrated and tested
- [ ] 100+ unit tests passing
- [ ] Documentation complete
- [ ] Release notes prepared

---

## Stretch Goals (P2)

If time permits:

- [ ] TUI trend charts
- [ ] Mouse support in TUI
- [ ] Multiple TUI themes
- [ ] CFG support for more languages (Go, Java, C++)
- [ ] Live secret verification
- [ ] CI/CD pipeline templates
- [ ] Automatic pattern updates
- [ ] Machine learning classifier

---

## Risk Management

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| VulnLLM hosting complexity | High | Medium | Use Modal (simpler than self-hosting) |
| CFG slicing accuracy | High | Medium | Start with simple patterns, iterate |
| TUI terminal compatibility | Medium | Medium | Test on multiple terminals early |
| Timeline slippage | High | High | Prioritize P0 features, defer P2 |
| Resource constraints | Medium | Medium | Start with 1 developer, add if needed |

---

## Next Steps

1. **Immediate (This Week):**
   - [ ] Review and approve this plan
   - [ ] Set up Modal.com account
   - [ ] Create coax-tui crate
   - [ ] Create coax-cfg crate
   - [ ] Create opendev-threat-model crate

2. **Week 1-2:**
   - [ ] Deploy VulnLLM-R-7B on Modal
   - [ ] Implement basic TUI dashboard
   - [ ] Build CFG from AST
   - [ ] Implement threat model data structures

3. **Week 3-4:**
   - [ ] Complete LLM client
   - [ ] Add TUI filtering
   - [ ] Implement entry/sink detection
   - [ ] Add threat correlation

---

*Task list created: 2026-03-15*
*Next review: Before starting implementation*
