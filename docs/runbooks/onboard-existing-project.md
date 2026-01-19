# Runbook: Onboard Existing Project to Paver

## When to Use

Use this runbook when helping a user adopt paver in an existing codebase. This applies when:
- A user asks for help adopting paver in their project
- You're evaluating a project's documentation for PAVED compliance
- You're migrating legacy documentation to PAVED format

## Preconditions

- paver CLI installed and accessible
- Access to the target project's repository
- Understanding of the project's documentation structure
- Read access to existing documentation files

## Steps

### 1. Run initial assessment

Scan the project to understand its documentation landscape:

```bash
paver adopt
```

Review the output for:
- Detected documentation paths
- Document counts by type
- Recommended document type mappings
- Missing sections that PAVED requires

### 2. Generate suggested configuration

Get configuration recommendations:

```bash
paver adopt --suggest-config
```

Review the suggested settings and note any adjustments needed for the project's specific needs.

### 3. Preview initialization

Dry-run the init to see what would be created:

```bash
paver adopt --dry-run
```

Confirm the proposed changes make sense before proceeding.

### 4. Initialize paver

Create the configuration file:

```bash
paver init
```

### 5. Configure gradual mode

Edit `.paver.toml` to enable gradual adoption:

```toml
[rules]
gradual = true
gradual_until = "YYYY-MM-DD"  # Set to 2-3 months from now
```

This prevents CI failures while documentation is being migrated.

### 6. Identify high-priority documents

Determine which documents to convert first. Prioritize by:
1. Frequency of use (most-read docs first)
2. Criticality (onboarding, core services)
3. Freshness (recently updated docs are more accurate)

List candidates:
```bash
ls docs/ | head -10
```

### 7. Convert priority documents

For each high-priority document:

1. Determine the appropriate PAVED type:
   - Service/module documentation → Component
   - Operational procedures → Runbook
   - Design decisions → ADR

2. Create a new PAVED document:
   ```bash
   paver new component <name>
   # or
   paver new runbook <name>
   # or
   paver new adr <name>
   ```

3. Migrate content from the legacy document into PAVED sections

4. Add Verification commands that prove accuracy

5. Add Examples with copy-paste code

### 8. Validate converted documents

Check that converted docs pass validation:

```bash
paver check docs/components/<name>.md
```

In gradual mode, errors appear as warnings. Address them before disabling gradual mode.

### 9. Set up CI integration

Help the user add paver to their CI pipeline. Provide the appropriate configuration for their CI system (GitHub Actions, GitLab CI, etc.).

### 10. Install git hooks

Set up pre-commit hooks:

```bash
paver hooks install
```

For verification in hooks:

```bash
paver hooks install --verify
```

### 11. Document progress tracking

Show the user how to monitor adoption progress:

```bash
paver check
paver coverage
```

### 12. Plan gradual mode exit

Establish criteria for disabling gradual mode:
- All priority documents converted
- CI passing consistently
- Team comfortable with workflow

Set a target date and add it to `gradual_until`.

## Rollback

If adoption causes problems:

1. Remove the git hooks:
   ```bash
   paver hooks uninstall
   ```

2. Remove CI integration (revert workflow file changes)

3. Optionally remove `.paver.toml`:
   ```bash
   rm .paver.toml
   ```

The original documentation remains intact throughout the process.

## Verification

Confirm paver is correctly configured:

```bash
paver config list
```

Confirm validation runs:

```bash
paver check
```

Confirm hooks are installed:

```bash
ls -la .git/hooks/pre-commit
```

## Escalation

If issues arise during onboarding:

1. Check the user-facing guide at `site/docs/onboarding-existing-projects.md` for common patterns
2. Review project-specific documentation needs
3. Open an issue on the paver repository with:
   - Project structure summary
   - Error messages or unexpected behavior
   - Steps attempted

## Examples

### Onboarding a Python project

```bash
# Assess existing docs
paver adopt

# Initialize with defaults
paver init

# Enable gradual mode
# Edit .paver.toml: gradual = true

# Convert README to component
paver new component my-python-lib

# Validate
paver check
```

### Onboarding a monorepo

For monorepos, configure the docs root appropriately:

```bash
paver init --docs-root packages/my-service/docs
```

Or set up multiple paver configurations:

```bash
# In each package directory
cd packages/service-a && paver init
cd packages/service-b && paver init
```

### Handling mixed documentation styles

When a project has multiple doc formats (Markdown, RST, AsciiDoc):

1. Focus paver on Markdown files initially
2. Exclude other formats in `.paver.toml`:
   ```toml
   [mapping]
   exclude = ["**/*.rst", "**/*.adoc"]
   ```
3. Convert other formats to Markdown over time

## Paths

- `src/commands/adopt.rs` - Adopt command implementation
- `src/commands/init.rs` - Init command implementation
- `site/docs/onboarding-existing-projects.md` - User-facing guide
