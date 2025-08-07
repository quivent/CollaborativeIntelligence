# 🔐 Secure Your Agent Explorer with Auth0

Your Agent Explorer now includes **Auth0 authentication** with email allowlisting! Here's how to set it up.

## 🚀 Quick Deploy with Authentication

### Option 1: Deploy Now (Auth0 setup later)
```bash
# Deploy current version - will show Auth0 errors until configured
vercel --prod

# Then follow AUTH0_SETUP.md to configure authentication
```

### Option 2: Setup Auth0 First (Recommended)
1. **Follow [AUTH0_SETUP.md](AUTH0_SETUP.md)** to create Auth0 application
2. **Set environment variables** in Vercel dashboard
3. **Deploy** with authentication working

## ⚡ Environment Variables for Vercel

Set these in your Vercel project dashboard:

```bash
REACT_APP_AUTH0_DOMAIN=your-domain.auth0.com
REACT_APP_AUTH0_CLIENT_ID=your-client-id-here
```

**Via Vercel CLI:**
```bash
vercel env add REACT_APP_AUTH0_DOMAIN
# Enter: your-domain.auth0.com

vercel env add REACT_APP_AUTH0_CLIENT_ID  
# Enter: your-client-id-here
```

**Via Vercel Dashboard:**
1. Go to [vercel.com/dashboard](https://vercel.com/dashboard)
2. Select `collaborative-intelligence` project
3. **Settings** → **Environment Variables**
4. Add both variables

## 📧 Configure Email Access

Edit `src/components/AccessControl.tsx`:

```typescript
const ALLOWED_EMAILS = [
  'your-email@gmail.com',       // Replace with your email
  'colleague@company.com',      // Add team members
  'admin@yourdomain.com',      // Add anyone else
];
```

## 🔄 Deploy with Authentication

```bash
# Deploy the updated app with Auth0
vercel --prod
```

## ✅ What Happens Next

### 🔒 **With Auth0 Configured:**
- **Login Screen**: Users see professional Auth0 login
- **Email Check**: Only allowed emails can access
- **User Profile**: Shows logged-in user in top-right
- **Secure Access**: Full protection for your agent data

### ❌ **Without Auth0 Configured:**
- **Error Screen**: Users see Auth0 configuration errors
- **Still Secure**: App won't work until properly configured
- **Easy Fix**: Just add environment variables and redeploy

## 🛠️ Test Your Setup

1. **Visit your deployed URL**
2. **Login with an allowed email** → Should work perfectly
3. **Login with unauthorized email** → Should show "Access Denied"
4. **Test logout** → Click user profile → Sign Out

## 🆘 Need Help?

- **Full Setup Guide**: [AUTH0_SETUP.md](AUTH0_SETUP.md)
- **Remove Auth0**: Edit `App.tsx` to remove authentication wrapper
- **Troubleshooting**: Check Vercel logs for environment variable issues

---

**Your Agent Explorer is now enterprise-ready with secure authentication!** 🔐✨