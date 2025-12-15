# VaultX npm Publishing Guide

How to publish VaultX to npm registry so users can install with `npm install -g vaultx`.

## Prerequisites

### 1. npm Account
Create a free account at https://www.npmjs.com/signup if you don't have one.

### 2. Verify Package Name
Check that `vaultx` is available:
- Visit: https://www.npmjs.com/package/vaultx
- If not taken, you can publish

### 3. Update Author Info
Edit `npm/package.json` and update:
```json
{
  "author": "Your Name <your.email@example.com>",
  "homepage": "https://github.com/yourusername/VaultX",
  "repository": {
    "url": "https://github.com/yourusername/VaultX.git"
  }
}
```

## Publishing Steps

### Step 1: Prepare Binaries

Make sure GitHub Releases have pre-built binaries:

```bash
# Create a git tag (this triggers GitHub Actions)
git tag v0.1.0
git push origin v0.1.0

# Wait for GitHub Actions to build binaries
# Check: https://github.com/yourusername/VaultX/releases
```

The release should contain these binaries:
- `vx-linux-x64`
- `vx-macos-x64`
- `vx-macos-arm64`
- `vx-windows-x64.exe`

### Step 2: Login to npm

```bash
npm login
# Enter your npm username
# Enter your npm password
# Enter your npm email
# Verify 2FA code if enabled
```

You should see:
```
Logged in as YOUR_USERNAME on https://registry.npmjs.org/
```

### Step 3: Update Version

Edit `npm/package.json`:
```json
{
  "version": "0.1.0"
}
```

The version format is `MAJOR.MINOR.PATCH` (semantic versioning).

### Step 4: Publish to npm

From the repository root:
```bash
cd npm
npm publish
```

You should see:
```
npm notice publishing vaultx@0.1.0
+ vaultx@0.1.0
```

### Step 5: Verify Publication

Check that it's live:
```bash
# Check npm registry
npm view vaultx

# Or visit: https://www.npmjs.com/package/vaultx
```

## Testing the Installation

### Test Locally Before Publishing
```bash
# Create a test directory
mkdir ~/vaultx-test && cd ~/vaultx-test

# Install from local package (before publishing)
npm install -g ../VaultX/npm

# Test it works
vx --version
vx --help
```

### Test Global Installation After Publishing
```bash
# Install from npm registry
npm install -g vaultx

# Test commands
vx init test-project
vx add test-project SECRET_KEY
vx list
vx get test-project SECRET_KEY
```

### Test on Different Platforms
Use this Matrix to test:
| Platform | Test Command |
|----------|--------------|
| Linux x64 | `npm install -g vaultx && vx --version` |
| macOS Intel | Run on Intel Mac |
| macOS ARM64 | Run on M1/M2 Mac |
| Windows x64 | `npm install -g vaultx && vx --version` |

## Updating the Package

### For Bug Fixes (Patch Version)
```bash
# Update version: 0.1.0 → 0.1.1
# In npm/package.json: "version": "0.1.1"

cd npm
npm publish
```

### For New Features (Minor Version)
```bash
# Update version: 0.1.0 → 0.2.0
# In npm/package.json: "version": "0.2.0"

cd npm
npm publish
```

### For Major Changes (Major Version)
```bash
# Update version: 0.1.0 → 1.0.0
# In npm/package.json: "version": "1.0.0"

cd npm
npm publish
```

## Automating with GitHub Actions

Create `.github/workflows/publish-npm.yml`:

```yaml
name: Publish to npm

on:
  release:
    types: [published]

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v3
        with:
          node-version: 18
          registry-url: https://registry.npmjs.org/

      - name: Publish to npm
        run: |
          cd npm
          npm publish
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
```

Then set up `NPM_TOKEN` in GitHub Secrets:
1. Create token at https://www.npmjs.com/settings/~/tokens
2. Add to GitHub repository secrets: https://github.com/yourusername/VaultX/settings/secrets
3. Name it: `NPM_TOKEN`

Now publishing is automatic when you create a GitHub Release!

## Troubleshooting

### "Package name is already taken"

Solution: Choose a different name
```json
{
  "name": "@yourusername/vaultx"
}
```

Then publish with scoped package:
```bash
npm publish --access public
```

### "ERR! 403 You do not have permission to publish"

Solutions:
1. Check you're logged in: `npm whoami`
2. Create new npm account or use existing one
3. Make sure email is verified on npmjs.com

### "Binary download fails after publishing"

The install script downloads from `release/download/latest/`

Make sure your GitHub Release:
1. Has the correct binary names
2. Uses the `latest` tag or specific version tag
3. Binaries are not renamed after release

### Version Already Published

Can't publish the same version twice. Increment it:
```bash
# Current: 0.1.0 → Next: 0.1.1
npm version patch  # This updates package.json
npm publish
```

## Maintenance

### Update Dependencies
Periodically update Node dependencies:
```bash
npm outdated
npm update
```

### Monitor Downloads
Track package popularity:
- https://www.npmjs.com/package/vaultx/stats
- Shows weekly downloads, growth, etc.

### User Feedback
Monitor GitHub issues for installation problems:
- https://github.com/yourusername/VaultX/issues

## Best Practices

✅ **DO:**
- Keep package.json metadata up-to-date
- Test on multiple platforms before publishing
- Use semantic versioning
- Include clear README
- Respond to user issues
- Update regularly with fixes/features

❌ **DON'T:**
- Publish without testing
- Use version 0.0.0 in production
- Leave old versions without documentation
- Publish private credentials
- Forget to update install script for new platforms

## Version History Example

```
1.0.0 - Major release, production ready
  ├─ 1.0.1 - Bug fixes
  ├─ 1.0.2 - Security patches
  └─ 1.1.0 - New feature: TTL-based expiration
0.2.0 - Beta release
  └─ 0.2.1 - Beta fixes
0.1.0 - Initial alpha release
```

## Next Steps

1. ✅ Update author information
2. ✅ Create GitHub Release with binaries
3. ✅ Create npm account
4. ✅ Publish: `cd npm && npm publish`
5. ✅ Test: `npm install -g vaultx`
6. ✅ Share with users!

## Support

- **npm Help**: https://docs.npmjs.com/
- **GitHub Releases**: https://docs.github.com/repositories/releasing-projects-on-github/managing-releases-in-a-repository
- **Semantic Versioning**: https://semver.org/

