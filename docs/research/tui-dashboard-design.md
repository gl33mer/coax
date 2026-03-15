# TUI Dashboard Design for Coax

**Date:** 2026-03-15
**Author:** Coax Research Team
**Status:** Complete

---

## Executive Summary

This document designs a comprehensive Terminal User Interface (TUI) dashboard for Coax, enabling interactive security scanning, finding management, and real-time monitoring. The design prioritizes developer experience, performance, and feature completeness.

**Recommended Framework:** Ratatui (Rust)

---

## TUI Framework Comparison

### Framework Options

| Framework | Language | Maturity | Performance | Documentation | Community | Learning Curve |
|-----------|----------|----------|-------------|---------------|-----------|----------------|
| **Ratatui** | Rust | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | Low-Medium |
| **TUI React** | TypeScript | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | Medium |
| **Blessed-contrib** | Node.js | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | Low |
| **Bubbletea** | Go | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | Low |
| **Textual** | Python | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | Low |

---

### Ratatui (Recommended)

**Repository:** https://github.com/ratatui-org/ratatui

| Aspect | Details |
|--------|---------|
| **Language** | Rust (native integration with Coax) |
| **Maturity** | Fork of tui-rs (2023), highly active development |
| **Performance** | Excellent (Rust, no GC pauses) |
| **Documentation** | Comprehensive docs, examples, cookbook |
| **Community** | Active Discord, regular releases |
| **Widgets** | 20+ built-in widgets |
| **Learning Curve** | Low-Medium (good tutorials available) |

**Built-in Widgets:**
- Block, Paragraph, Text
- List, Table
- Chart (line, bar, scatter)
- Gauge, Sparkline
- Tabs, SelectableList
- Canvas (for custom drawing)
- Calendar, Barchart
- Graph (axis, grids)

**Advantages for Coax:**
1. **Zero FFI overhead** - Native Rust, no language boundary
2. **Single binary** - No Node.js/Python runtime required
3. **Performance** - 60 FPS rendering, minimal memory
4. **Type safety** - Compile-time guarantees
5. **Existing ecosystem** - Coax already in Rust

**Example Code:**
```rust
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph, List, ListItem},
    Frame,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;
    
    terminal.draw(|f| {
        let area = f.size();
        let block = Block::default()
            .title("🛡️ Coax Security Dashboard")
            .borders(Borders::ALL);
        f.render_widget(block, area);
    })?;
    
    Ok(())
}
```

---

### TUI React (Alternative)

**Repository:** https://github.com/yaron101/tui-react

| Aspect | Details |
|--------|---------|
| **Language** | TypeScript/JavaScript |
| **Maturity** | Early stage |
| **Performance** | Good (but requires Node.js runtime) |
| **Documentation** | Limited |
| **Community** | Small |
| **Learning Curve** | Medium (React knowledge required) |

**Disadvantages for Coax:**
- Requires separate Node.js binary or bundling
- FFI overhead for Rust ↔ JS communication
- Larger binary size
- Not recommended for Coax

---

### Blessed-contrib (Not Recommended)

**Repository:** https://github.com/yaronn/blessed-contrib

| Aspect | Details |
|--------|---------|
| **Language** | Node.js |
| **Maturity** | Mature (but aging) |
| **Performance** | Fair (JavaScript, GC pauses) |
| **Documentation** | Good |
| **Community** | Moderate |

**Disadvantages:**
- Node.js runtime required
- Not suitable for Rust project
- Aging codebase

---

## TUI Dashboard Design

### Main Dashboard View

```
╔══════════════════════════════════════════════════════════════════════════╗
║  🛡️  Coax Security Dashboard                              [q] Quit      ║
║  v0.3.0 | Phase 3 P1                                     [?] Help      ║
╠══════════════════════════════════════════════════════════════════════════╣
║                                                                          ║
║  📁 Repository: /home/user/my-project                                    ║
║  🕐 Last Scan: 2026-03-15 16:00 (2 minutes ago)                         ║
║  📊 Total Files: 1,247 | Scanned: 1,247 (100%)                          ║
║                                                                          ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │  Scan Results Summary                                           │    ║
║  │  ┌──────────────┬──────────────┬──────────────┬──────────────┐ │    ║
║  │  │   Critical   │     High     │    Medium    │     Low      │ │    ║
║  │  ├──────────────┼──────────────┼──────────────┼──────────────┤ │    ║
║  │  │      3 🚨    │      7 ⚠️    │     12 ⚡    │      5 ℹ️    │ │    ║
║  │  └──────────────┴──────────────┴──────────────┴──────────────┘ │    ║
║  │                                                                 │    ║
║  │  Total Findings: 27  |  False Positives: 2  |  Ignored: 5      │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │  Recent Findings (↑↓ Navigate, Enter View, / Search)            │    ║
║  │  ┌─────────────────────────────────────────────────────────────┐│    ║
║  │  │ 🚨 AWS_ACCESS_KEY       config.yml:45:12      [Unverified]  ││    ║
║  │  │ 🚨 GITHUB_PAT           .env:12:20          [Verified]      ││    ║
║  │  │ ⚠️  GENERIC_SECRET      src/utils.py:89:8   [Unverified]    ││    ║
║  │  │ ⚡ HIGH_ENTROPY         data.json:234:5     [Unverified]    ││    ║
║  │  │ ℹ️  LOW_CONFIDENCE      test.js:56:1        [Ignored]       ││    ║
║  │  └─────────────────────────────────────────────────────────────┘│    ║
║  │                                                                 │    ║
║  │  Showing 5 of 27 findings                                        │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │  Quick Actions                                                  │    ║
║  │  [R] Rescan  [F] Filter  [S] Sort  [E] Export  [B] Baseline    │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║  [↑↓] Navigate  [Enter] View  [/] Search  [R] Rescan  [q] Quit         ║
║                                                                          ║
╚══════════════════════════════════════════════════════════════════════════╝
```

---

### Finding Detail View

```
╔══════════════════════════════════════════════════════════════════════════╗
║  Finding Detail                                           [←] Back      ║
╠══════════════════════════════════════════════════════════════════════════╣
║                                                                          ║
║  🚨 AWS_ACCESS_KEY (Critical)                                            ║
║  ───────────────────────────────────────────────────────────────────     ║
║                                                                          ║
║  📍 Location: config.yml:45:12                                           ║
║  🔍 Pattern: AKIA[0-9A-Z]{16}                                            ║
║  📊 Confidence: 99%  |  Entropy: 4.2  |  Token Efficiency: 3.1          ║
║  ✅ Verification: Unverified  |  Baseline: New Finding                  ║
║                                                                          ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │  Code Preview (4 lines around match)                            │    ║
║  │  ─────────────────────────────────────────────────────────────  │    ║
║  │  44: # AWS Configuration                                        │    ║
║  │  45: aws_access_key_id = AKIAIOSFODNN7EXAMPLE      │ ← Match    │    ║
║  │  46: aws_secret_access_key = wJalrXUtnFEMI/K7MDENG/             │    ║
║  │  47: region = us-east-1                                         │    ║
║  │  ─────────────────────────────────────────────────────────────  │    ║
║  │  [←↑↓→ Scroll]  [L] Show Line Numbers  [C] Copy Secret         │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │  Context Analysis                                                │    ║
║  │  ─────────────────────────────────────────────────────────────  │    ║
║  │  • Location: Configuration file (YAML)                          │    ║
║  │  • Context: Not in comment or test file                         │    ║
║  │  • Risk: High - appears in production config                    │    ║
║  │  • Similar findings: 2 other AWS keys in repository             │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │  Recommendation                                                  │    ║
║  │  ─────────────────────────────────────────────────────────────  │    ║
║  │  Remove immediately and rotate the key via AWS IAM Console.     │    ║
║  │  1. Go to AWS IAM Console → Security Credentials                │    ║
║  │  2. Deactivate the compromised access key                       │    ║
║  │  3. Create new access key                                       │    ║
║  │  4. Update all applications using this key                      │    ║
║  │  5. Delete old key after verification                           │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │  Actions                                                         │    ║
║  │  ─────────────────────────────────────────────────────────────  │    ║
║  │  [I] Ignore Finding  [F] Mark False Positive  [V] Verify Now   │    ║
║  │  [B] Add to Baseline  [O] Open in Editor  [U] View URI         │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
╚══════════════════════════════════════════════════════════════════════════╝
```

---

### Finding List View (with Filtering)

```
╔══════════════════════════════════════════════════════════════════════════╗
║  Findings List                           [←] Back  [N] New Filter       ║
╠══════════════════════════════════════════════════════════════════════════╣
║                                                                          ║
║  Active Filters:                                                         ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │  Severity: [x] Critical  [x] High  [ ] Medium  [ ] Low          │    ║
║  │  Status:   [x] New  [x] Unverified  [ ] Verified  [ ] Ignored   │    ║
║  │  Pattern:  [AWS_*] [GITHUB_*]                                   │    ║
║  │  File:     [*.yml] [*.env]                                      │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║  Results: 10 of 27 findings (showing filtered)                          ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │ # │ Severity │ Pattern        │ File            │ Line │ Status │    ║
║  │───┼──────────┼────────────────┼─────────────────┼──────┼────────│    ║
║  │ 1 │ 🚨 Crit  │ AWS_ACCESS_KEY │ config.yml      │   45 │ New    │    ║
║  │ 2 │ 🚨 Crit  │ AWS_SECRET_KEY │ config.yml      │   46 │ New    │    ║
║  │ 3 │ 🚨 Crit  │ GITHUB_PAT     │ .env            │   12 │ Verif. │    ║
║  │ 4 │ ⚠️ High  │ STRIPE_KEY     │ payment.yml     │   23 │ Unver. │    ║
║  │ 5 │ ⚠️ High  │ GENERIC_SECRET │ src/auth.rs     │   89 │ Unver. │    ║
║  │ 6 │ ⚠️ High  │ JWT_SECRET     │ appsettings.json│  156 │ New    │    ║
║  │ 7 │ ⚠️ High  │ PRIVATE_KEY    │ deploy.sh       │   34 │ Unver. │    ║
║  │ 8 │ ⚠️ High  │ API_KEY        │ src/config.ts   │   78 │ Ignored│    ║
║  │ 9 │ ⚠️ High  │ DATABASE_URL   │ .env.production │    5 │ Unver. │    ║
║  │10 │ ⚠️ High  │ SLACK_TOKEN    │ notifications.js│  112 │ Unver. │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║  [↑↓] Navigate  [Enter] View  [S] Sort  [E] Export  [C] Clear Filters  ║
║                                                                          ║
╚══════════════════════════════════════════════════════════════════════════╝
```

---

### Trend View (Findings Over Time)

```
╔══════════════════════════════════════════════════════════════════════════╗
║  Findings Trend                                           [←] Back      ║
╠══════════════════════════════════════════════════════════════════════════╣
║                                                                          ║
║  📈 Findings Over Time (Last 30 Days)                                   ║
║                                                                          ║
║  Count                                                                  ║
║    50 ┤                                                                ║
║       │                                                                ║
║    40 ┤                              ╭───╮                             ║
║       │                              │   │                             ║
║    30 ┤                    ╭───╮     │   │     ╭───╮                   ║
║       │                    │   │     │   │     │   │                   ║
║    20 ┤          ╭───╮     │   │     │   │     │   │     ╭───╮         ║
║       │          │   │     │   │     │   │     │   │     │   │         ║
║    10 ┤   ╭───╮  │   │  ╭──╯   │  ╭──╯   │  ╭──╯   │  ╭──╯   │  ╭───╮  ║
║       │   │   │  │   │  │      │  │      │  │      │  │      │  │   │  ║
║     0 ┼───┴───┴──┴───┴──┴──────┴──┴──────┴──┴──────┴──┴──────┴──┴───┴──╱ ║
║       Mar 1   Mar 5   Mar 10   Mar 15   Mar 20   Mar 25   Mar 30   Apr  ║
║                                                                          ║
║  ───── Critical  ━━━━━ High  ········ Medium  - - - - Low               ║
║                                                                          ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │  Statistics                                                     │    ║
║  │  ─────────────────────────────────────────────────────────────  │    ║
║  │  • Peak findings: Mar 18 (42 findings)                          │    ║
║  │  • Current trend: ↓ Decreasing (-15% from last week)            │    ║
║  │  • Average per scan: 28 findings                                │    ║
║  │  • Resolution rate: 67% (18/27 resolved this week)              │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║  [←→] Pan  [+/−] Zoom  [D] Change Period  [E] Export Chart             ║
║                                                                          ║
╚══════════════════════════════════════════════════════════════════════════╝
```

---

### Settings View

```
╔══════════════════════════════════════════════════════════════════════════╗
║  Settings                                                 [←] Back      ║
╠══════════════════════════════════════════════════════════════════════════╣
║                                                                          ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │  Scanner Configuration                                          │    ║
║  │  ─────────────────────────────────────────────────────────────  │    ║
║  │  [x] Enable Token Efficiency Filter                             │    ║
║  │  [x] Enable Word Filter                                         │    ║
║  │  [x] Enable Context Detection                                   │    ║
║  │  [ ] Enable Encoded Secret Detection                            │    ║
║  │  [ ] Enable LLM Analysis (requires API key)                     │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │  Thresholds                                                     │    ║
║  │  ─────────────────────────────────────────────────────────────  │    ║
║  │  Entropy Threshold:        [4.0    ] (0.0 - 8.0)                │    ║
║  │  Token Efficiency Threshold: [2.5    ] (0.0 - 5.0)              │    ║
║  │  Confidence Threshold:     [Medium ] (Low/Medium/High)          │    ║
║  │  Max File Size:            [10 MB  ] (1 - 100 MB)               │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │  Display Options                                                │    ║
║  │  ─────────────────────────────────────────────────────────────  │    ║
║  │  [x] Show Line Numbers                                          │    ║
║  │  [x] Mask Secrets in Output                                     │    ║
║  │  [x] Color Code Severity                                        │    ║
║  │  [x] Show Progress Bar                                          │    ║
║  │  Theme: [Dark    ] (Dark/Light/Blue/Purple)                     │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │  Watch Mode                                                     │    ║
║  │  ─────────────────────────────────────────────────────────────  │    ║
║  │  [x] Enable Watch Mode                                          │    ║
║  │  Scan Interval: [5 seconds] (1 - 60 seconds)                    │    ║
║  │  [x] Notify on New Findings                                     │    ║
║  │  [x] Auto-rescan on File Save                                   │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │  LLM Integration (Phase 3 P1)                                   │    ║
║  │  ─────────────────────────────────────────────────────────────  │    ║
║  │  API Endpoint: [http://localhost:8000/v1/chat/completions     ] │    ║
║  │  Model:        [UCSB-SURFI/VulnLLM-R-7B                       ] │    ║
║  │  API Key:      [********************************************]   │    ║
║  │  [Test Connection]                                              │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║  [S] Save Settings  [R] Reset to Defaults  [←] Back                    ║
║                                                                          ║
╚══════════════════════════════════════════════════════════════════════════╝
```

---

### Baseline Management View

```
╔══════════════════════════════════════════════════════════════════════════╗
║  Baseline Management                                      [←] Back      ║
╠══════════════════════════════════════════════════════════════════════════╣
║                                                                          ║
║  📁 Current Baseline: .coax-baseline.json                               ║
║  📅 Created: 2026-03-01 10:00  |  Updated: 2026-03-15 14:30            ║
║  📊 Version: 1.0.0  |  Findings Tracked: 45                            ║
║                                                                          ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │  Baseline Comparison                                            │    ║
║  │  ─────────────────────────────────────────────────────────────  │    ║
║  │  ┌─────────────────┬──────────┬──────────┬──────────┐          │    ║
║  │  │ Status          │ Count    │ Change   │ Trend    │          │    ║
║  │  ├─────────────────┼──────────┼──────────┼──────────┤          │    ║
║  │  │ New Findings    │    3     │   +3     │   ↑      │          │    ║
║  │  │ Resolved        │   12     │  -12     │   ↓      │          │    ║
║  │  │ Unchanged       │   30     │    0     │   →      │          │    ║
║  │  │ False Positives │    2     │   +1     │   ↑      │          │    ║
║  │  │ Ignored         │    5     │   +2     │   ↑      │          │    ║
║  │  └─────────────────┴──────────┴──────────┴──────────┘          │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │  New Findings (Not in Baseline)                                 │    ║
║  │  ─────────────────────────────────────────────────────────────  │    ║
║  │  🚨 AWS_ACCESS_KEY      config.yml:45                           │    ║
║  │  ⚠️  GENERIC_SECRET     src/utils.py:89                         │    ║
║  │  ⚡ HIGH_ENTROPY        data.json:234                           │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║  ┌─────────────────────────────────────────────────────────────────┐    ║
║  │  Actions                                                         │    ║
║  │  ─────────────────────────────────────────────────────────────  │    ║
║  │  [U] Update Baseline (add new, remove resolved)                 │    ║
║  │  [E] Export Baseline                                            │    ║
║  │  [C] Create New Baseline                                        │    ║
║  │  [D] Delete Baseline                                            │    ║
║  │  [V] View Baseline File                                         │    ║
║  └─────────────────────────────────────────────────────────────────┘    ║
║                                                                          ║
║  [↑↓] Navigate  [Enter] View Details                                    ║
║                                                                          ║
╚══════════════════════════════════════════════════════════════════════════╝
```

---

## Interactive Features

### Keyboard Navigation

| Key | Action | Context |
|-----|--------|---------|
| `↑/↓` or `j/k` | Navigate up/down | All list views |
| `←/→` or `h/l` | Navigate left/right | Detail views, charts |
| `Enter` | View details / Select | All views |
| `Esc` or `q` | Back / Quit | All views |
| `/` | Search | List views |
| `n/N` | Next/Previous match | Search results |
| `Tab` | Switch panel | Dashboard view |

### Filtering

| Filter Type | Options |
|-------------|---------|
| **Severity** | Critical, High, Medium, Low |
| **Status** | New, Unverified, Verified, Ignored, False Positive |
| **Pattern** | AWS_*, GITHUB_*, STRIPE_*, etc. |
| **File** | Glob patterns (*.yml, *.env, src/*) |
| **Date** | Today, This Week, This Month, Custom Range |

### Sorting

| Sort Field | Order |
|------------|-------|
| **Severity** | Critical → Low (or reverse) |
| **File** | A-Z (or reverse) |
| **Line Number** | Low → High (or reverse) |
| **Pattern** | A-Z (or reverse) |
| **Date Found** | Newest → Oldest (or reverse) |
| **Confidence** | High → Low (or reverse) |

### Actions

| Action | Shortcut | Description |
|--------|----------|-------------|
| **Ignore Finding** | `I` | Mark as ignored (won't show in future scans) |
| **Mark False Positive** | `F` | Label as false positive |
| **Add to Baseline** | `B` | Add current finding to baseline |
| **Verify Now** | `V` | Trigger live verification (if available) |
| **Open in Editor** | `O` | Open file at finding location |
| **Copy Secret** | `C` | Copy secret to clipboard (masked) |
| **Export** | `E` | Export findings to JSON/SARIF/YAML |
| **Rescan** | `R` | Trigger new scan |

---

## Visual Design

### Color Scheme (Dark Theme)

| Element | Foreground | Background |
|---------|------------|------------|
| **Critical Severity** | Red (#FF0000) | Dark Red (#330000) |
| **High Severity** | Orange (#FF8800) | Dark Orange (#331100) |
| **Medium Severity** | Yellow (#FFFF00) | Dark Yellow (#333300) |
| **Low Severity** | Blue (#0088FF) | Dark Blue (#001133) |
| **Text (Primary)** | White (#FFFFFF) | - |
| **Text (Secondary)** | Gray (#AAAAAA) | - |
| **Borders** | Gray (#555555) | - |
| **Selected Item** | Black (#000000) | Blue (#0066CC) |
| **Success** | Green (#00FF00) | Dark Green (#003300) |
| **Error** | Red (#FF0000) | - |

### Icons (Emoji)

| Icon | Meaning |
|------|---------|
| 🚨 | Critical severity |
| ⚠️ | High severity |
| ⚡ | Medium severity |
| ℹ️ | Low severity / Info |
| 📁 | Repository / File |
| 🕐 | Time / Date |
| 📊 | Statistics / Chart |
| 🔍 | Pattern / Search |
| 📍 | Location |
| ✅ | Verified / Success |
| ❌ | Error / Failed |
| ⚙️ | Settings |
| 🛡️ | Coax branding |

### Progress Indicators

```
Scan Progress:
┌────────────────────────────────────────────────────────┐
│  Scanning...                                           │
│  ████████████████████████░░░░░░░░░░░░░░░░  65%        │
│  812 / 1,247 files | 23 findings so far               │
└────────────────────────────────────────────────────────┘

Confidence Gauge:
┌────────────────────────────────┐
│  Confidence: 99%               │
│  ████████████████████░░░░░░░░  │
└────────────────────────────────┘
```

### ASCII Charts

```
Findings Trend (Sparkline):
  ▁▃▅▆▄▃▂▁▂▃▅▆▇▆▅▄▃▂

Severity Distribution (Bar Chart):
  Critical: ████████ 3
  High:     ████████████████ 7
  Medium:   ████████████████████████ 12
  Low:      ██████████ 5
```

---

## Implementation Approach

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Coax TUI Binary                      │
│                    (coax-tui)                           │
└─────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
┌───────────────┐  ┌────────────────┐  ┌───────────────┐
│  UI Layer     │  │  State Mgmt    │  │  Data Layer   │
│  (Ratatui)    │  │  (App State)   │  │  (Scanner)    │
├───────────────┤  ├────────────────┤  ├───────────────┤
│ • Rendering   │  │ • Findings     │  │ • coax-scanner│
│ • Input       │  │ • Filters      │  │ • Baseline    │
│ • Events      │  │ • Settings     │  │ • LLM Client  │
│ • Themes      │  │ • Navigation   │  │ • Watch Mode  │
└───────────────┘  └────────────────┘  └───────────────┘
```

### Crate Structure

```
coax/
├── crates/
│   ├── coax-scanner/      # Existing scanner library
│   ├── coax-cli/          # Existing CLI
│   └── coax-tui/          # NEW: TUI dashboard
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs           # Entry point
│           ├── app.rs            # Application state
│           ├── ui/               # UI components
│           │   ├── mod.rs
│           │   ├── dashboard.rs  # Main dashboard
│           │   ├── findings.rs   # Finding list/detail
│           │   ├── settings.rs   # Settings panel
│           │   ├── baseline.rs   # Baseline management
│           │   └── trend.rs      # Trend charts
│           ├── event.rs          # Event handling
│           ├── input.rs          # Keyboard input
│           └── theme.rs          # Color themes
```

### Dependencies (coax-tui/Cargo.toml)

```toml
[package]
name = "coax-tui"
version.workspace = true
edition.workspace = true

[dependencies]
# Internal
coax-scanner = { workspace = true }

# TUI framework
ratatui = "0.26"
crossterm = "0.27"

# State management
tokio = { workspace = true }

# Utilities
serde = { workspace = true }
serde_json = { workspace = true }
chrono = "0.4"
unicode-width = "0.1"

# Clipboard (for copy secret)
arboard = "3.3"
```

---

## Feature Priorities

### P0: Core Features (Must Have)

| Feature | Effort | Priority |
|---------|--------|----------|
| Main dashboard view | 2 days | P0 |
| Finding list view | 2 days | P0 |
| Finding detail view | 2 days | P0 |
| Keyboard navigation | 1 day | P0 |
| Basic filtering | 1 day | P0 |
| Exit/help | 0.5 days | P0 |

**Total P0 Effort:** 8.5 days (~2 weeks)

---

### P1: Enhanced Features (Should Have)

| Feature | Effort | Priority |
|---------|--------|----------|
| Advanced filtering | 1 day | P1 |
| Sorting | 1 day | P1 |
| Search functionality | 1 day | P1 |
| Settings panel | 1 day | P1 |
| Baseline management | 2 days | P1 |
| Export (JSON/SARIF) | 1 day | P1 |
| Watch mode | 2 days | P1 |

**Total P1 Effort:** 9 days (~2 weeks)

---

### P2: Advanced Features (Nice to Have)

| Feature | Effort | Priority |
|---------|--------|----------|
| Trend charts | 2 days | P2 |
| LLM integration panel | 2 days | P2 |
| Multiple themes | 1 day | P2 |
| Mouse support | 1 day | P2 |
| Custom keybindings | 1 day | P2 |
| Plugin system | 3 days | P2 |

**Total P2 Effort:** 10 days (~2 weeks)

---

## Estimated Effort Summary

| Phase | Features | Effort | Timeline |
|-------|----------|--------|----------|
| **P0** | Core dashboard, list, detail, navigation | 8.5 days | 2 weeks |
| **P1** | Filtering, search, settings, baseline | 9 days | 2 weeks |
| **P2** | Charts, LLM, themes, mouse | 10 days | 2 weeks |
| **Total** | All features | 27.5 days | 6 weeks |

---

## Implementation Milestones

### Week 1-2: P0 Core Features

- [ ] Set up coax-tui crate with Ratatui
- [ ] Implement main dashboard layout
- [ ] Implement finding list view
- [ ] Implement finding detail view
- [ ] Add keyboard navigation (↑↓←→, Enter, q)
- [ ] Add basic filtering (severity)
- [ ] Add help screen

**Deliverable:** Functional TUI with basic navigation

---

### Week 3-4: P1 Enhanced Features

- [ ] Implement advanced filtering (pattern, file, status)
- [ ] Add sorting (severity, file, line, date)
- [ ] Add search functionality (fuzzy search)
- [ ] Implement settings panel
- [ ] Implement baseline management view
- [ ] Add export functionality (JSON, SARIF)
- [ ] Implement watch mode (auto-rescan)

**Deliverable:** Production-ready TUI with full feature set

---

### Week 5-6: P2 Advanced Features

- [ ] Implement trend charts (ASCII graphs)
- [ ] Add LLM integration panel
- [ ] Add multiple color themes
- [ ] Add mouse support
- [ ] Add custom keybindings configuration
- [ ] Performance optimization
- [ ] Documentation and examples

**Deliverable:** Polished TUI with advanced features

---

## Success Criteria

### End of P0 (Week 2)

- [ ] TUI launches without errors
- [ ] Dashboard shows scan results
- [ ] Can navigate findings with keyboard
- [ ] Can view finding details
- [ ] Can filter by severity
- [ ] Can exit cleanly

### End of P1 (Week 4)

- [ ] All P0 criteria met
- [ ] Advanced filtering works
- [ ] Search finds findings
- [ ] Settings can be changed
- [ ] Baseline can be updated
- [ ] Export generates valid files
- [ ] Watch mode auto-rescans

### End of P2 (Week 6)

- [ ] All P1 criteria met
- [ ] Trend charts display correctly
- [ ] Multiple themes available
- [ ] Mouse clicks work
- [ ] Performance: 60 FPS rendering
- [ ] Memory usage <50MB

---

## Risks and Mitigations

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Ratatui learning curve | Medium | Low | Good documentation, start with examples |
| Performance issues | High | Low | Rust performance, benchmark early |
| Terminal compatibility | Medium | Medium | Test on common terminals (iTerm, Windows Terminal) |
| Feature creep | High | High | Strict prioritization, defer P2 features |
| Integration complexity | Medium | Low | Clean API between scanner and TUI |

---

## Conclusion

**Recommended Approach:**

1. **Framework:** Ratatui (Rust native, excellent docs, active community)
2. **Phased Implementation:** P0 → P1 → P2 over 6 weeks
3. **Priority Features:** Dashboard, finding list/detail, navigation, filtering
4. **Stretch Goals:** Trend charts, LLM panel, themes

**Next Steps:**

1. Create `coax-tui` crate
2. Add Ratatui dependency
3. Implement basic dashboard layout
4. Iterate based on user feedback

---

## References

- **Ratatui Docs:** https://ratatui.rs/
- **Ratatui Examples:** https://github.com/ratatui-org/ratatui/tree/main/examples
- **Ratatui Cookbook:** https://ratatui.rs/cookbook/
- **TUI Design Inspiration:** https://github.com/ratatui-org/ratatui/blob/main/COMMUNITY-PROJECTS.md

---

*Design completed: 2026-03-15*
