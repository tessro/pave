# Enforcement

## Purpose

This document explains pave's enforcement mechanisms for documentation quality and how to configure them for different levels of strictness.

**Non-goals:** Not a CI/CD setup guide (see site documentation) or CI provider troubleshooting.

## Interface

Pave provides four enforcement commands that combine for progressively stricter requirements:

| Command | Purpose | Speed |
|---------|---------|-------|
| `pave check` | Validates document structure and rules | Fast |
| `pave verify` | Executes verification commands | Varies |
| `pave changed` | Detects impacted but not-updated docs | Fast |
| `pave coverage` | Measures code-to-doc coverage | Fast |

### Enforcement Levels

**Level 1 - Structure:** `pave check` - Ensures docs have required sections.

**Level 2 - Verification:** `pave check && pave verify` - Adds command execution.

**Level 3 - Change Detection:** `pave check && pave verify && pave changed --strict` - Fails if changed code has docs that weren't updated.

**Level 4 - Coverage:** Add `pave coverage --threshold 80` - Fails if coverage drops below threshold.

### Command Flags

**`pave check`**: `--format text|json|github`, `--strict`, `--gradual`, `--changed`, `--base <ref>`

**`pave verify`**: `--format`, `--timeout <secs>`, `--keep-going`, `--report <path>`

**`pave changed`**: `--base <ref>`, `--strict`, `--format`

**`pave coverage`**: `--threshold <N>`, `--include <pattern>`, `--exclude <pattern>`, `--format`

## Configuration

All rules are in `.pave.toml` under `[rules]`.

### Basic Validation

```toml
[rules]
max_lines = 300                       # Maximum lines per document (default: 300)
require_verification = true           # Require ## Verification section (default: true)
require_examples = true               # Require ## Examples section (default: true)
require_verification_commands = true  # Require commands in Verification (default: true)
```

### Output Matching

```toml
[rules]
strict_output_matching = false  # true: output mismatches fail; false: warnings (default)
skip_output_matching = false    # true: don't check output at all (default: false)
```

### Type-Specific Rules

```toml
[rules.type_specific]
runbooks = false    # Require When to Use, Steps, Rollback
adrs = false        # Require Status, Context, Decision, Consequences
components = false  # Require Interface OR Configuration section
```

### Path Validation

```toml
[rules]
validate_paths = false   # Validate glob syntax in ## Paths
warn_empty_paths = false # Warn if patterns match no files
```

### Gradual Mode

For incremental adoption:

```toml
[rules]
gradual = true                # Errors become warnings, check exits 0
gradual_until = "2025-06-01"  # Auto-disable after this date (YYYY-MM-DD)
```

CLI flags `--strict` and `--gradual` override the config. After `gradual_until` passes, gradual mode automatically disables.

### Code-to-Doc Mapping

```toml
[mapping]
exclude = ["target/", "node_modules/", "*.generated.rs"]
```

Patterns excluded from coverage and change detection. Built-in: `target/`, `node_modules/`, `dist/`, `__pycache__/`, `.git/`.

## Examples

### Minimal Config

```toml
[pave]
version = "0.1"

[docs]
root = "docs"
```

Run: `pave check`

### Strict Config

```toml
[pave]
version = "0.1"

[docs]
root = "docs"

[rules]
strict_output_matching = true
validate_paths = true

[rules.type_specific]
runbooks = true
adrs = true
components = true
```

Run:
```bash
pave check --strict && pave verify --keep-going && pave changed --strict && pave coverage --threshold 80
```

### Gradual Adoption Config

```toml
[pave]
version = "0.1"

[docs]
root = "docs"

[rules]
gradual = true
gradual_until = "2026-06-01"
```

CI workflow:
```yaml
- run: pave check --gradual --format github   # Report all, don't fail
- run: pave check --changed --strict          # Enforce on new/changed only
```

### Recommended CI Workflow

```yaml
name: Documentation
on: [push, pull_request]

jobs:
  docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Install pave
        run: cargo install pave

      - name: Check structure
        run: pave check --strict --format github

      - name: Verify commands
        run: pave verify --keep-going --format github

      - name: Check impacted docs
        run: pave changed --strict
        if: github.event_name == 'pull_request'

      - name: Check coverage
        run: pave coverage --threshold 70
```

## Verification

```bash
pave check docs/components/enforcement.md
```

```bash
pave check --gradual && echo "Gradual mode exits 0"
```

## Gotchas

- `--changed` requires git history: use `fetch-depth: 0` in checkout
- `pave changed` uses `## Paths` sections: docs without Paths won't trigger change detection
- Gradual mode affects `check` only, not `verify`
- Type-specific rules require directory conventions (`runbooks/`, `adrs/`, `components/`)

## Decisions

**Why multiple levels?** Teams have different needs. Layered levels allow incremental adoption.

**Why gradual mode with deadlines?** Teams need time to fix existing docs. Deadlines ensure gradual mode doesn't become permanent.

**Why strict output matching off by default?** Output is brittle (timestamps, paths). Warnings let teams fix issues without blocking CI.

**Why separate check and verify?** `check` is fast for every PR. `verify` may be slow or require specific environments.

## Paths

- `src/commands/check.rs`
- `src/commands/verify.rs`
- `src/commands/changed.rs`
- `src/commands/coverage.rs`
- `src/config.rs`
