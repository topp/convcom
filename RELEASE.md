# Release Process

This document describes how to create releases for ConvCom with automated binary builds.

## Prerequisites

- Push access to the repository
- GitHub Actions enabled on the repository
- All changes merged into the main branch

## Creating a Release

### 1. Update Version (if needed)

Update the version in `Cargo.toml`:

```toml
[package]
name = "convcom"
version = "1.0.0"  # Update this
edition = "2021"
```

### 2. Create and Push Tag

```bash
# Make sure you're on the main branch with latest changes
git checkout main
git pull origin main

# Create a version tag (must start with 'v')
git tag v1.0.0

# Push the tag to trigger release workflow
git push origin v1.0.0
```

### 3. Automated Build Process

Once you push the tag, GitHub Actions will automatically:

1. **Build binaries** for all supported platforms:
   - Linux (x86_64)
   - Windows (x86_64)
   - macOS Intel (x86_64)
   - macOS Apple Silicon (aarch64)

2. **Create a GitHub Release** with:
   - Release notes
   - Installation instructions
   - All platform binaries attached

3. **Upload artifacts** as compressed files:
   - `convcom-linux-x86_64.tar.gz`
   - `convcom-windows-x86_64.exe.zip`
   - `convcom-macos-x86_64.tar.gz`
   - `convcom-macos-aarch64.tar.gz`

### 4. Verify Release

1. Go to the [Releases page](https://github.com/topp/convcom/releases)
2. Check that the new release appears with all binaries
3. Test download and installation on different platforms

## Release Workflow Details

The release process is handled by `.github/workflows/release.yml`:

- **Trigger**: Pushing tags matching `v*.*.*` pattern
- **Builds**: Cross-compilation for 4 platforms
- **Artifacts**: Compressed binaries for easy distribution
- **Release**: Automatic GitHub release with formatted description

## Versioning

We follow [Semantic Versioning](https://semver.org/):

- `v1.0.0` - Major release (breaking changes)
- `v1.1.0` - Minor release (new features, backward compatible)
- `v1.1.1` - Patch release (bug fixes)

## Manual Release (Emergency)

If automated release fails, you can create releases manually:

1. Build binaries locally:
   ```bash
   # Linux
   cargo build --release --target x86_64-unknown-linux-gnu
   
   # Windows (on Windows or with cross-compilation)
   cargo build --release --target x86_64-pc-windows-msvc
   
   # macOS (on macOS)
   cargo build --release --target x86_64-apple-darwin
   cargo build --release --target aarch64-apple-darwin
   ```

2. Compress binaries:
   ```bash
   tar -czvf convcom-linux-x86_64.tar.gz target/x86_64-unknown-linux-gnu/release/convcom
   # ... repeat for other platforms
   ```

3. Create release manually on GitHub with the compressed files

## Troubleshooting

**Build fails for a specific platform:**
- Check if the target is properly added in the workflow
- Verify cross-compilation dependencies

**Release not triggered:**
- Ensure tag follows `v*.*.*` pattern exactly
- Check that GitHub Actions is enabled for the repository

**Binary doesn't work:**
- Verify the build completed successfully
- Check for missing runtime dependencies on target platform
