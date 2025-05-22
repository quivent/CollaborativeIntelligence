# Tauri Configuration Fix Guide

## Problem

You're encountering this error with `tauri.conf.json`:

```
Error `tauri.conf.json` error on `build`: Additional properties are not allowed ('devProtocol' was unexpected)
```

This occurs because the `devProtocol` property in your `tauri.conf.json` file is not recognized by your current Tauri version. This property either:
1. Was added recently and is not supported in your Tauri version
2. Was renamed or moved to a different location in the configuration structure
3. Is a typo or unused configuration option

## Solution

You need to modify your `tauri.conf.json` file to remove or correct this invalid property. Here's how to fix it:

### Step 1: Locate your tauri.conf.json file

This file is typically located in the `src-tauri` directory of your project:
```
your-project/
├── src-tauri/
│   ├── tauri.conf.json  <-- This is the target file
```

### Step 2: Remove or fix the invalid property

Open the `tauri.conf.json` file and look for the `devProtocol` property. It's likely somewhere in the configuration structure, possibly under a section like `build`, `dev`, or at the root level.

**Option A: Remove the property completely**

Find and delete the `devProtocol` line. For example:

```json
{
  "build": {
    "devProtocol": "http",  <-- Remove this line
    "distDir": "../dist"
  }
}
```

**Option B: Replace with the correct property (if applicable)**

If you need the functionality provided by `devProtocol`, it may have been renamed. Common alternatives in newer Tauri versions include:

```json
{
  "build": {
    "devPath": "http://localhost:3000"  // Use this instead
  }
}
```

### Step 3: Check Tauri version compatibility

This error could indicate a mismatch between your configuration file and Tauri version. Check your Tauri version with:

```bash
npm list @tauri-apps/cli
```

Then consult the [Tauri documentation](https://tauri.app/v1/api/config/) for that specific version to ensure your configuration is valid.

## For Tauri versions 1.x and newer

In Tauri 1.x, the development server configuration typically looks like:

```json
{
  "build": {
    "devPath": "http://localhost:3000", 
    "distDir": "../dist"
  }
}
```

## For older Tauri versions (pre-1.0)

In older Tauri versions, the configuration might have been:

```json
{
  "build": {
    "devPath": "../dist",
    "beforeDevCommand": "npm run dev"
  }
}
```

## Fix Template

If you're unsure about the exact structure, here's a basic template for the `build` section that should work with recent Tauri versions:

```json
{
  "build": {
    "devPath": "http://localhost:3000",
    "distDir": "../dist",
    "beforeBuildCommand": "npm run build",
    "beforeDevCommand": "npm run dev"
  }
}
```

After making these changes, try running `npm run tauri dev` again. The error should be resolved.

## Additional Tips

1. If you're migrating between Tauri versions, review the [migration guides](https://tauri.app/v1/guides/migration/migrate-from-v1/) for any breaking changes.

2. Use the Tauri CLI to validate your configuration:
   ```bash
   npx @tauri-apps/cli config
   ```

3. Consider using the Tauri config guide to regenerate a valid configuration file:
   ```bash
   npx @tauri-apps/cli init
   ```
   This will ask questions and generate a valid configuration for your current Tauri version.