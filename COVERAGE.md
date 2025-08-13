# Test Coverage

This project uses [cargo-tarpaulin](https://github.com/xd009642/tarpaulin) for code coverage analysis and reports coverage to [Coveralls.io](https://coveralls.io/).

## Coverage Workflows

### Detailed Coverage (`coverage-detailed.yml`)
- **Triggers**: Tagged push to `main` branch
- **Purpose**: Comprehensive coverage analysis with multiple output formats
- **Outputs**:
  - HTML report deployed to GitHub Pages
  - Coverage badge (requires `COVERAGE_GIST_ID` secret)
  - Coveralls integration
  - Workflow artifacts

## Local Coverage Testing

To run coverage locally:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run coverage for jirust-cli only
cargo tarpaulin \
  --engine llvm \
  --all-features \
  --packages jirust-cli \
  --exclude-files "target/*" \
  --exclude-files "*/tests/*" \
  --exclude-files "jira_v3_openapi/*" \
  --out Html

# View the report
open tarpaulin-report.html
```

## Configuration

### Required Secrets

- `COVERALLS_REPO_TOKEN`: Token for uploading to Coveralls.io (already configured)

### Optional Secrets (for detailed coverage)

- `COVERAGE_GIST_ID`: GitHub Gist ID for coverage badge (optional)

### Coverage Settings

The coverage analysis:
- **Focuses on**: `jirust-cli` package only
- **Excludes**: Auto-generated `jira_v3_openapi` code, test files, build artifacts
- **Engine**: LLVM for accurate coverage measurement
- **Timeout**: 120 seconds to handle longer test runs

## Coverage Targets

- **Target**: 80%+ line coverage
- **Current**: View latest results on [Coveralls.io](https://coveralls.io/github/ilpanich/jirust-cli)

## Troubleshooting

### Common Issues

1. **Coverage too low**: Add more unit tests, especially for error paths
2. **Timeout errors**: Increase timeout in workflow or optimize slow tests
3. **Build failures**: Ensure all dependencies are correctly specified

### Local Debugging

```bash
# Verbose output for debugging
cargo tarpaulin --engine llvm --all-features --packages jirust-cli --verbose

# Check which files are being analyzed
cargo tarpaulin --engine llvm --all-features --packages jirust-cli --print-summary
```
