# Repository Relocation Methods

## CRITICAL WARNING

**Repository relocation requires extreme care to preserve git history and integrity.**

A serious error occurred during the migration of this repository from being embedded within Points to a sibling directory, resulting in the **complete loss of git history**. This document contains guidance to prevent such issues in the future.

## Safe Relocation Methods

### Method 1: Git Clone with New Remote (Recommended)

This approach safely preserves all history while changing location:

```bash
# 1. Clone the repository to the new location
git clone /path/to/original/repo /path/to/new/location

# 2. Verify the repository integrity in the new location
cd /path/to/new/location
git log  # Should show complete history

# 3. Update the remote to point to the correct origin (if needed)
git remote set-url origin new-remote-url

# 4. Push to ensure remote is updated
git push
```

### Method 2: Git Bundle for Offline Transfer

For situations requiring offline transfer between systems:

```bash
# On the source system
cd /path/to/original/repo
git bundle create repo-transfer.bundle --all

# Copy the bundle file to the target system

# On the target system
mkdir /path/to/new/repo
cd /path/to/new/repo
git clone repo-transfer.bundle .
git remote remove origin  # Remove the bundle as a remote
git remote add origin real-repository-url  # Add the actual remote
```

### Method 3: Create Sibling Repository with Shared History

For creating a sibling repository relationship:

```bash
# 1. Create the new repository in the sibling location
mkdir -p /path/to/sibling/location
cd /path/to/sibling/location
git init

# 2. Add the original repository as a remote
git remote add original /path/to/original/repo

# 3. Fetch all branches and history
git fetch original

# 4. Create local branches tracking the original
git checkout -b main original/main

# 5. Set up the correct remote for the new repository location
git remote remove original
git remote add origin new-repository-url
```

## Verification Checklist

After any repository relocation, verify:

1. **Complete history**: Run `git log` to ensure all commits are present
2. **Branch integrity**: Check `git branch -a` to verify all branches transferred
3. **Remote configuration**: Verify `git remote -v` shows correct remotes
4. **Working directory state**: Run `git status` to check for clean working directory
5. **Push/pull functionality**: Test `git pull` and `git push` to verify remote connectivity

## Recovery Options for Lost History

If history is lost, options include:

1. **Repository restoration**: Clone from a backup or the original remote
2. **Ghost commits**: Manually recreate commits in sequence using patch files
3. **Repository forensics**: Use git recovery tools to find dangling objects

## Required Process for All Repository Operations

1. **Pre-operation notification**: Notify Repository Topologist before any structural changes
2. **Approval**: Wait for explicit approval before proceeding
3. **Backup**: Create a backup copy before making changes
4. **Verification**: Verify integrity after operations
5. **Post-operation notification**: Report results to Repository Topologist

## Institutional Knowledge

This document was created following a critical error where standard file operations were used instead of git-aware commands, resulting in loss of repository history. Always treat git repositories as special entities requiring specific tools for manipulation, not as regular directories.

---

*Document created on: April 23, 2025*