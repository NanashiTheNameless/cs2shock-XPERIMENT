# Release and Deployment Guide

## Automated Builds

This project uses GitHub Actions for automated building and releasing. There are two main workflows:

### CI Workflow (`.github/workflows/ci.yml`)

Runs on every push to `main` or `dev` branches, and on pull requests.

**What it does:**
- Runs `cargo fmt` to check code formatting
- Runs `cargo clippy` to check for common mistakes
- Builds the project on Linux, Windows, and macOS
- Runs all tests
- Creates release builds to verify they compile

**Triggered by:**
- Push to `main` or `dev` branches
- Pull requests to `main` or `dev` branches

### Release Workflow (`.github/workflows/release.yml`)

Creates official releases with binaries for all supported platforms.

**What it does:**
- Builds release binaries for:
  - Windows x64
  - Linux x64
  - macOS x64 (Intel)
  - macOS ARM64 (Apple Silicon)
- Creates a GitHub release
- Uploads all binaries to the release
- Includes `README.md`, `config.json`, and `gamestate_integration_cs2shock.cfg` in each archive

**Triggered by:**
- Pushing a tag starting with `v` (e.g., `v1.1.0`)
- Manual workflow dispatch from GitHub Actions UI

## Creating a Release

### 1. Update Version Number

Update the version in `Cargo.toml`:

```toml
[package]
name = "cs2shock"
version = "1.2.0"  # <- Update this
edition = "2021"
```

### 2. Commit Changes

```bash
git add Cargo.toml
git commit -m "Bump version to 1.2.0"
git push origin main
```

### 3. Create and Push Tag

```bash
# Create the tag
git tag v1.2.0

# Push the tag to GitHub
git push origin v1.2.0
```

### 4. Watch the Build

1. Go to the [Actions tab](https://github.com/VolcanoCookies/cs2shock/actions) on GitHub
2. You'll see the "Release" workflow running
3. Wait for all builds to complete (usually 5-10 minutes)
4. Once done, check the [Releases page](https://github.com/VolcanoCookies/cs2shock/releases)

## Manual Release (Workflow Dispatch)

You can also trigger a release manually without creating a tag:

1. Go to the [Actions tab](https://github.com/VolcanoCookies/cs2shock/actions)
2. Click on "Release" workflow
3. Click "Run workflow"
4. Select the branch
5. Click "Run workflow" button

This will create a development release with a timestamp.

## Local Testing

Before creating a release, test the build locally:

```bash
# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test

# Build release
cargo build --release
```

## Platform-Specific Notes

### Linux
The CI builds on Ubuntu and requires GTK3 and X11 libraries. These are installed automatically in the workflow.

### macOS
- Builds for both Intel (x64) and Apple Silicon (ARM64)
- Cross-compilation is handled automatically by the workflow

### Windows
- Builds using MSVC toolchain
- Creates a `.zip` archive instead of `.tar.gz`

## Troubleshooting

### Build fails on one platform
- Check the Actions log for specific errors
- Ensure all dependencies are properly listed in `Cargo.toml`
- Test locally if you have access to that platform

### Release not created
- Make sure the tag starts with `v`
- Check GitHub Actions permissions in repository settings
- Verify GITHUB_TOKEN has write access

### Binary doesn't work
- Test the release build locally first: `cargo build --release`
- Check that all runtime dependencies are documented
- Verify the binary is properly linked for the target platform
