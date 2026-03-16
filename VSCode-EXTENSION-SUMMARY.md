# VS Code Extension Development Plan - Summary Report

**Date:** 2026-03-16  
**Status:** ✅ Complete - Ready for Development  
**Version:** v0.8.0

---

## Executive Summary

Comprehensive VS Code Extension development plan has been created for Coax v0.8.0. The plan includes detailed research, complete specification, development timeline, and roadmap integration.

**Key Deliverables:**
- ✅ Extension research document (60+ pages of technical details)
- ✅ Complete specification (100+ sections covering all features)
- ✅ Development timeline (5-week detailed plan)
- ✅ Roadmap integration (Phase 3 priority updated)

---

## Documents Created

### 1. Research Document

**File:** `/home/shva/QwenDev/devshield-internal/docs/research/vscode-extension-research.md`

**Contents:**
- VS Code Extension Architecture Overview
- Key VS Code APIs (6 detailed sections)
  - Diagnostic Collection (squiggly underlines)
  - File System Watcher (real-time scanning)
  - Code Actions Provider (quick fixes)
  - Hover Provider (tooltips)
  - Status Bar Item (status indicator)
  - Command Palette (user commands)
- Competitive Analysis (GitGuardian, Snyk)
- Binary Bundling Strategy (5 platforms)
- Extension Manifest Configuration
- Estimated Development Effort
- Technical Challenges & Solutions
- Best Practices

**Key Insights:**
- GitGuardian uses bundled CLI architecture (same as planned for Coax)
- Scan on save (not on every keystroke) for performance
- Severity colors: Red (Error), Yellow (Warning), Blue (Info), Green (Hint)
- Binary bundling requires platform-specific folders
- File watcher should use `RelativePattern` for precision

---

### 2. Extension Specification

**File:** `/home/shva/QwenDev/devshield-internal/docs/VSCode-EXTENSION-SPEC.md`

**Contents:**
- Overview & Value Proposition
- Features (6 major feature areas)
  1. Real-time Scanning (triggers, file filtering)
  2. Inline Warnings (squiggly colors, hover tooltips)
  3. Problems Panel Integration
  4. Quick-Fix Actions (8 action types)
  5. Status Bar Indicator (5 display states)
  6. Command Palette (8 commands)
- Architecture (component diagram, data flow)
- Technical Requirements
  - Extension manifest configuration
  - Dependencies (runtime & dev)
  - Binary bundling (5 platforms)
- Development Phases (4 phases)
- Estimated Effort (19 developer days + 6 QA days)
- Success Criteria (functional, non-functional, UX)
- Risks & Mitigations
- Future Enhancements (v0.9.0, v1.0.0, v1.1.0)

**Key Specifications:**
- Extension size target: <50MB
- Activation time: <500ms
- Scan time (100KB file): <2s
- Memory usage: <100MB during scan
- Compatibility: VS Code >=1.85.0
- Platforms: Windows x64, macOS x64/arm64, Linux x64/arm64

---

### 3. Development Timeline

**File:** `/home/shva/QwenDev/devshield-internal/docs/VSCode-EXTENSION-TIMELINE.md`

**Contents:**
- Week-by-week breakdown (5 weeks total)
- Daily task lists with checkboxes
- Code snippets for each component
- Acceptance criteria for each milestone
- Dependencies tracking
- Risk mitigation strategies
- Success metrics

**Timeline Overview:**

| Week | Focus | Key Deliverables |
|------|-------|------------------|
| **Week 1** | Project Setup & Binary Integration | Extension scaffold, binary bundling, basic commands |
| **Week 2** | Core Scanning & Diagnostics | File watcher, CLI integration, Problems panel |
| **Week 3** | Inline Warnings & Status | Squiggly underlines, hover tooltips, status bar |
| **Week 4** | Quick-Fix Actions | Code actions provider, 8 quick-fix types |
| **Week 5** | Polish & Release | Testing, documentation, Marketplace submission |

**Key Milestones:**
- Week 2, Day 5: MVP Demo (basic scanning works)
- Week 4, Day 5: Feature Complete
- Week 5, Day 3: QA Complete
- Week 5, Day 5: Marketplace Release

---

### 4. Roadmap Integration

**File:** `/home/shva/QwenDev/devshield-internal/docs/PHASE3-PROPOSAL.md` (updated)

**Changes:**
- Added "Feature 0: VS Code Extension" as P0 priority
- Updated timeline from 8-12 weeks to 12-16 weeks
- Added competitive analysis table
- Updated executive summary with dual priorities

**New Feature Entry:**
```markdown
### Feature 0: VS Code Extension (NEW - P0 Priority)

**Status:** P0 (Critical for Adoption)
**Effort:** 4-5 weeks
**Dependencies:** Coax CLI binary builds for all platforms
**Version:** v0.8.0
```

---

## Technical Architecture Summary

### Extension Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    VS Code Extension                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ File Watcher │  │ Diagnostics  │  │ Code Actions │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│              ┌──────────────────────┐                   │
│              │   Coax CLI Binary    │                   │
│              │   (bundled)          │                   │
│              └──────────────────────┘                   │
└─────────────────────────────────────────────────────────┘
```

### Key VS Code APIs

| API | Purpose | Usage |
|-----|---------|-------|
| `createDiagnosticCollection()` | Squiggly underlines | Display findings inline |
| `createFileSystemWatcher()` | File monitoring | Trigger scans on save/open |
| `registerCodeActionsProvider()` | Quick fixes | Lightbulb actions |
| `registerHoverProvider()` | Tooltips | Show details on hover |
| `createStatusBarItem()` | Status bar | Display scan status |
| `registerCommand()` | Commands | Command palette integration |

### Binary Bundling Strategy

```
bundled/
├── darwin-arm64/coax
├── darwin-x64/coax
├── linux-x64/coax
├── linux-arm64/coax
└── win32-x64/coax.exe
```

**Platform Detection:**
- Auto-detect at runtime using `os.platform()` and `os.arch()`
- Fallback to system `coax` command if bundled binary fails
- Set executable permissions on Unix systems

---

## Development Effort Estimate

### By Phase

| Phase | Duration | Developer Days | QA Days |
|-------|----------|---------------|---------|
| Phase 1: MVP | 1 week | 5 days | 1 day |
| Phase 2: Diagnostics | 1 week | 5 days | 1 day |
| Phase 3: Quick-Fixes | 1 week | 5 days | 2 days |
| Phase 4: Polish | 1 week | 4 days | 2 days |
| **Total** | **4 weeks** | **19 days** | **6 days** |

### By Role

| Role | Effort |
|------|--------|
| TypeScript Developer | 19 days |
| QA Engineer | 6 days |
| Technical Writer | 3 days |
| Designer (icons) | 2 days |
| **Total** | **30 person-days** |

---

## Success Criteria

### Functional Requirements

| Requirement | Acceptance Criteria |
|-------------|---------------------|
| Installation | Installs from VS Code Marketplace without errors |
| Scan on Save | Triggers scan within 500ms of file save |
| Inline Warnings | Squiggly underlines appear for all findings |
| Severity Colors | Critical=red, High=orange, Medium=yellow, Low=green |
| Problems Panel | All findings visible in Problems panel |
| Quick Fixes | At least 4 quick-fix actions per finding |
| Status Bar | Accurate finding count and status |
| Stability | Zero crashes in 100+ test sessions |

### Non-Functional Requirements

| Requirement | Target |
|-------------|--------|
| Extension Size | <50MB (with bundled binaries) |
| Activation Time | <500ms |
| Scan Time (100KB) | <2s |
| Memory Usage | <100MB during scan |
| Compatibility | VS Code >=1.85.0 |

---

## Key Technical Challenges

### Challenge 1: Binary Bundling Complexity

**Risk:** Bundle Coax CLI for 5 platforms without bloating extension size

**Mitigation:**
- Use platform-specific folders
- Compress binaries with UPX
- Provide fallback to system binary
- Test on all platforms early

---

### Challenge 2: Performance Impact

**Risk:** Scanning large files blocks UI

**Mitigation:**
- Scan on save, not on every keystroke
- Implement file size limits (10MB default)
- Use debouncing for rapid saves
- Show progress indicators for long scans

---

### Challenge 3: False Positive Noise

**Risk:** Too many warnings annoy developers

**Mitigation:**
- Configurable severity threshold
- "Ignore for session" quick fix
- `.coax.yaml` for permanent ignores
- Smart filtering (test files, fixtures)

---

## Competitive Analysis

### Feature Comparison

| Feature | GitGuardian | Snyk | Coax (Planned) |
|---------|-------------|------|----------------|
| Real-time scanning | ✅ | ✅ | ✅ |
| Inline diagnostics | ✅ | ✅ | ✅ |
| Problems panel | ✅ | ✅ | ✅ |
| Quick fixes | ⚠️ Limited | ✅ | ✅ (8 types) |
| Status bar | ✅ | ✅ | ✅ |
| Binary bundling | ✅ | ✅ | ✅ |
| Baseline files | ❌ | ❌ | ✅ (future) |
| Unicode detection | ❌ | ❌ | ✅ (unique) |
| Local-only mode | ⚠️ Partial | ❌ | ✅ (default) |

### Key Differentiators

1. **Unicode Confusable Detection** - Unique to Coax
2. **Local-Only Scanning** - Privacy-focused, no cloud required
3. **Baseline Files** - Track known findings across scans
4. **Live Verification** - Confirm if secrets are actually active

---

## Resource Requirements

### Development Resources

| Resource | Quantity | Duration |
|----------|----------|----------|
| TypeScript Developer | 1 FTE | 4-5 weeks |
| QA Engineer | 0.5 FTE | 3 weeks |
| Technical Writer | 0.2 FTE | 2 weeks |
| Designer | 0.2 FTE | 1 week |

### Infrastructure Resources

| Resource | Purpose |
|----------|---------|
| Coax CLI builds (5 platforms) | Binary bundling |
| VS Code Marketplace account | Extension publishing |
| Test accounts (AWS, GitHub, etc.) | Testing secret detection |
| CI/CD pipeline | Automated testing & packaging |

---

## Next Steps

### Immediate Actions (This Week)

1. **Review Documentation**
   - Read `docs/VSCode-EXTENSION-SPEC.md`
   - Review `docs/VSCode-EXTENSION-TIMELINE.md`
   - Study `docs/research/vscode-extension-research.md`

2. **Prepare Coax CLI Builds**
   - Build for all 5 platforms
   - Test binary execution
   - Verify JSON output format

3. **Set Up Development Environment**
   - Install VS Code Extension Development Tools
   - Create GitHub repository for extension
   - Set up TypeScript project

4. **Begin Week 1 Tasks**
   - Initialize extension project
   - Configure package.json
   - Create directory structure

### Week 1 Deliverables

- [ ] Extension scaffold created
- [ ] Binary bundling complete
- [ ] Basic "Hello World" extension works
- [ ] Commands registered and functional

---

## Risk Assessment

### High Priority Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Coax CLI builds delayed | Medium | High | Start builds immediately, use cross-compilation |
| Binary execution fails on some platforms | Medium | High | Test early on all platforms, provide fallback |
| Performance issues with large files | Medium | Medium | Implement file size limits, progress indicators |

### Medium Priority Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| False positives annoy users | High | High | Configurable threshold, easy ignore |
| TypeScript learning curve | Low | Low | Use experienced developer, reference examples |
| Testing takes longer than expected | Medium | Medium | Start testing early, automate where possible |

---

## Success Metrics

### Development Metrics

| Metric | Target |
|--------|--------|
| On-time delivery | 100% of milestones met |
| Bug count at release | 0 critical, <5 minor |
| Test coverage | >80% of core logic |
| Documentation completeness | 100% of features documented |

### Post-Release Metrics (30 days)

| Metric | Target |
|--------|--------|
| Downloads | 1,000+ |
| Active users | 500+ |
| Marketplace rating | 4.0+ stars |
| Issue reports | <10 critical |

---

## References

### Internal Documents

- `docs/VSCode-EXTENSION-SPEC.md` - Complete specification
- `docs/VSCode-EXTENSION-TIMELINE.md` - Development timeline
- `docs/research/vscode-extension-research.md` - Technical research
- `docs/PHASE3-PROPOSAL.md` - Phase 3 roadmap (updated)

### External Resources

- [VS Code Extension API](https://code.visualstudio.com/api)
- [GitGuardian VS Code Extension](https://docs.gitguardian.com/ggshield-docs/integrations/ide-integrations/vscode)
- [Snyk VS Code Extension](https://github.com/snyk/vscode-extension)
- [VS Code Extension Samples](https://github.com/microsoft/vscode-extension-samples)

---

## Conclusion

The VS Code Extension development plan is **complete and ready for implementation**. All necessary documentation has been created:

1. ✅ **Research** - Comprehensive analysis of VS Code APIs and competitive landscape
2. ✅ **Specification** - Detailed feature specifications and technical requirements
3. ✅ **Timeline** - Week-by-week development plan with daily tasks
4. ✅ **Roadmap** - Phase 3 priorities updated with VS Code Extension as P0

**Recommended Next Step:** Begin Week 1 development tasks (Project Setup & Binary Integration)

**Estimated Time to Market:** 5 weeks from development start to Marketplace release

**Confidence Level:** High - Based on proven architecture (GitGuardian model), comprehensive planning, and clear success criteria.

---

**Report Prepared:** 2026-03-16  
**Status:** ✅ Ready for Development  
**Approval:** Pending
