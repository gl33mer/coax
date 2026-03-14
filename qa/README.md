# Coax QA

Quality Assurance infrastructure for testing the Coax security scanner.

## Overview

This directory contains all QA resources for validating Coax functionality, performance, and accuracy.

## Structure

```
qa/
├── README.md              # This file - QA overview
├── qa-template.md         # QA test template
├── test-repos/            # Test repositories
│   ├── small/             # <100 files (quick validation)
│   ├── medium/            # 100-1000 files (standard tests)
│   ├── large/             # >1000 files (performance tests)
│   ├── mixed-language/    # Multi-language projects
│   └── legacy/            # Old codebases with outdated patterns
└── results/               # Test results
    ├── TEMPLATE.md        # Results template
    ├── small.json         # Scan results
    ├── medium.json
    ├── large.json
    ├── mixed-language.json
    └── legacy.json
```

## Test Repositories

### 1. Small (<100 files)
**Purpose:** Quick validation, development testing
**Expected scan time:** <1 second
**Languages:** Single language (Rust or Python)
**Use cases:**
- Rapid iteration during development
- CI/CD quick checks
- Pattern testing

### 2. Medium (100-1000 files)
**Purpose:** Standard testing, feature validation
**Expected scan time:** 1-5 seconds
**Languages:** 1-2 languages
**Use cases:**
- Pre-release testing
- Feature validation
- Regression testing

### 3. Large (>1000 files)
**Purpose:** Performance testing, stress testing
**Expected scan time:** 5-30 seconds
**Languages:** Multiple languages
**Use cases:**
- Performance benchmarks
- Memory usage testing
- Thread pool optimization

### 4. Mixed-Language
**Purpose:** Multi-language support validation
**Expected scan time:** 5-15 seconds
**Languages:** JavaScript + Python + Rust
**Use cases:**
- Full-stack application testing
- Language-specific pattern validation
- Cross-language secret detection

### 5. Legacy
**Purpose:** Outdated pattern detection
**Expected scan time:** 2-10 seconds
**Languages:** Various (older versions)
**Use cases:**
- Legacy secret format detection
- Deprecated API key detection
- Historical vulnerability patterns

## Running QA

### Quick Test
```bash
cd /home/shva/QwenDev/coax-internal/coax

# Scan small test repo
./target/release/coax scan -p qa/test-repos/small
```

### Full QA Suite
```bash
# Scan all repositories with JSON output
for repo in qa/test-repos/*/; do
    name=$(basename "$repo")
    echo "Scanning $name..."
    ./target/release/coax scan -p "$repo" \
        -f json \
        -o "qa/results/${name}.json" \
        --quiet
done

echo "QA complete! Results in qa/results/"
```

### Performance Benchmark
```bash
# With timing and verbose output
time ./target/release/coax scan -p qa/test-repos/large -v

# With thread count tuning
./target/release/coax scan -p qa/test-repos/large -t 4
```

### Specific Test Cases
```bash
# Test JSON output
./target/release/coax scan -p qa/test-repos/small -f json

# Test YAML output
./target/release/coax scan -p qa/test-repos/small -f yaml

# Test with line content
./target/release/coax scan -p qa/test-repos/small --with-content

# Test exclude patterns
./target/release/coax scan -p qa/test-repos/small -e "test_*,*.min.js"

# Test hidden files
./target/release/coax scan -p qa/test-repos/small --hidden
```

## QA Checklist

Before each release, verify:

- [ ] All 5 test repos scan without errors
- [ ] Expected secrets are detected
- [ ] No false positives in clean files
- [ ] Performance within targets (<30s for large)
- [ ] Memory usage <100MB
- [ ] JSON/YAML output is valid
- [ ] Exit codes correct (1 for findings, 0 for clean)
- [ ] Help documentation accurate

## Performance Targets

| Repository | Files | Target Time | Memory |
|------------|-------|-------------|--------|
| Small | <100 | <1s | <20MB |
| Medium | 100-1000 | <5s | <50MB |
| Large | >1000 | <30s | <100MB |
| Mixed | 500-2000 | <15s | <80MB |
| Legacy | 200-800 | <10s | <50MB |

## Reporting Issues

When reporting QA issues, include:

1. Test repository name
2. Coax version (`coax version`)
3. Command run
4. Expected vs actual results
5. Performance metrics (time, memory)
6. Any false positives/negatives

## Adding New Test Repositories

1. Create directory in `qa/test-repos/`
2. Add README.md with repo details
3. Add test_secrets.txt with known secrets (for validation)
4. Add clean files for false positive testing
5. Update this README with repo info
6. Run initial scan and save baseline

## Continuous Integration

QA tests should be run:
- Before each release
- After major changes
- Weekly (automated)
- When adding new patterns

---

**Last Updated:** 2026-03-14
**Version:** 0.2.0
**Maintainer:** Coax Team
