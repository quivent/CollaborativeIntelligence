# 🤖 Automated Auth0 Setup

**90% automated setup!** This script handles almost everything for you.

## 🚀 One-Command Setup

```bash
# Option 1: Full automated setup (recommended)
npm run setup-auth0-full

# Option 2: Just the Node.js script
npm run setup-auth0

# Option 3: Direct execution
./setup-auth0.sh
```

## ✅ What Gets Automated

### **Fully Automated (No Manual Work Required):**
- ✅ **Auth0 Application Creation** - Creates SPA with correct settings
- ✅ **Callback URL Configuration** - Sets production and development URLs
- ✅ **Grant Types** - Configures Authorization Code, Refresh Token, Implicit
- ✅ **Local Environment** - Updates `.env` file with credentials
- ✅ **Vercel Environment Variables** - Sets production environment vars
- ✅ **Social Connections** - Enables Google OAuth (if available)

### **Requires 30 Seconds of Manual Work:**
- 📧 **Update email allowlist** in `src/components/AccessControl.tsx`
- 🚀 **Deploy to production** with `vercel --prod`

## 📋 Prerequisites

### **Required (One-Time Setup):**
1. **Auth0 Account** - Free at [auth0.com](https://auth0.com)
2. **Vercel Account** - Already have this ✅
3. **Auth0 CLI** - Script will install automatically

### **Optional but Recommended:**
- **Vercel CLI** - For automatic environment variable setup
  ```bash
  npm install -g vercel
  vercel auth login
  ```

## 🎯 Step-by-Step Process

### **Step 1: Run Automated Setup**
```bash
npm run setup-auth0-full
```

**What happens automatically:**
1. **Checks dependencies** (installs Auth0 CLI if needed)
2. **Prompts for Auth0 login** (opens browser)
3. **Creates Auth0 application** with perfect settings
4. **Updates environment files** with credentials
5. **Sets Vercel environment variables** (if CLI available)
6. **Configures social logins** (Google OAuth)

### **Step 2: Update Email Allowlist (30 seconds)**
Edit `src/components/AccessControl.tsx`:
```typescript
const ALLOWED_EMAILS = [
  'your-email@gmail.com',       // Replace with your email
  'colleague@company.com',      // Add team members
];
```

### **Step 3: Deploy**
```bash
vercel --prod
```

## 🎉 You're Done!

Your Agent Explorer now has:
- ✅ **Professional Auth0 authentication**
- ✅ **Email allowlist security**
- ✅ **Beautiful login/logout UI**
- ✅ **Production environment variables**
- ✅ **Social login support**

## 📊 Automation Coverage

| Task | Status | Time Saved |
|------|--------|------------|
| Create Auth0 account | ⚠️ Manual (one-time) | N/A |
| Create Auth0 application | ✅ Automated | ~10 minutes |
| Configure callback URLs | ✅ Automated | ~5 minutes |
| Set grant types | ✅ Automated | ~3 minutes |
| Get credentials | ✅ Automated | ~2 minutes |
| Update .env file | ✅ Automated | ~2 minutes |
| Set Vercel env vars | ✅ Automated | ~5 minutes |
| Enable social logins | ✅ Automated | ~5 minutes |
| Update email allowlist | ⚠️ Manual | ~30 seconds |
| Deploy application | ⚠️ Manual | ~1 minute |

**Total automation: ~90% (32+ minutes of work automated)**

## 🔧 Manual Fallback

If automation fails, use the detailed manual guide:
- **[AUTH0_SETUP.md](AUTH0_SETUP.md)** - Complete manual setup guide

## 🆘 Troubleshooting

### **"Auth0 CLI not found"**
```bash
# Install Auth0 CLI
brew install auth0/auth0-cli/auth0

# Or follow installation guide
```

### **"Vercel CLI not available"**
```bash
# Install Vercel CLI (optional)
npm install -g vercel
vercel auth login
```

### **"Failed to create application"**
- Ensure you're logged in: `auth0 auth login`
- Check your Auth0 account has sufficient permissions
- Try the manual setup guide

### **"Environment variables not set"**
- Check Vercel dashboard manually
- Or set them via CLI:
  ```bash
  vercel env add REACT_APP_AUTH0_DOMAIN
  vercel env add REACT_APP_AUTH0_CLIENT_ID
  ```

## 🔐 Security Notes

- **Credentials are automatically generated** - No manual copy/paste errors
- **Environment variables are set securely** in Vercel dashboard
- **No credentials stored in code** - Everything uses environment variables
- **Professional Auth0 security** - Industry-standard authentication

---

**Your Agent Explorer will be secured in under 5 minutes!** 🚀✨