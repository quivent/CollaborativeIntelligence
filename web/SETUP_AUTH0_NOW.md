# 🔐 Set Up Auth0 Authentication - Complete Manual Guide

**Everything you need to do to secure your Agent Explorer with Auth0 authentication.**

## 📋 What You Need

1. **Auth0 Account** (free)
2. **10 minutes of your time**
3. **Your email address** (to allow yourself access)

---

## 🚀 Step 1: Create Auth0 Account & Application

### **1.1 Sign Up for Auth0**
1. Go to [auth0.com](https://auth0.com)
2. Click **"Sign Up Free"**
3. Create your account (use your email)
4. Choose a **tenant domain** (e.g., `your-name.auth0.com`)

### **1.2 Create Application**
1. In Auth0 Dashboard, click **"Applications"** → **"Create Application"**
2. **Name**: `Agent Explorer`
3. **Type**: Select **"Single Page Web Applications"**
4. Click **"Create"**

### **1.3 Configure Application Settings**
In the **Settings** tab of your new application:

**Application URIs** section - paste these exactly:
```
Allowed Callback URLs:
https://collaborative-intelligence-o7bw7ep8w-claude-codes-projects.vercel.app, http://localhost:3000

Allowed Logout URLs:
https://collaborative-intelligence-o7bw7ep8w-claude-codes-projects.vercel.app, http://localhost:3000

Allowed Web Origins:
https://collaborative-intelligence-o7bw7ep8w-claude-codes-projects.vercel.app, http://localhost:3000
```

**Advanced Settings** → **Grant Types** - ensure these are checked:
- ✅ Authorization Code
- ✅ Refresh Token  
- ✅ Implicit

**Click "Save Changes"**

---

## 🔑 Step 2: Get Your Credentials

From the **Settings** tab, copy these two values:

- **Domain**: `your-domain.auth0.com`
- **Client ID**: `abc123def456...` (long string)

**Keep these open - you'll need them in the next steps.**

---

## 🌐 Step 3: Set Environment Variables in Vercel

### **Option A: Vercel Dashboard (Easiest)**
1. Go to [vercel.com/dashboard](https://vercel.com/dashboard)
2. Click your **"collaborative-intelligence"** project
3. Go to **"Settings"** → **"Environment Variables"**
4. Click **"Add"** and create:

```bash
Name: REACT_APP_AUTH0_DOMAIN
Value: your-domain.auth0.com
Environment: Production
```

5. Click **"Add"** again and create:

```bash
Name: REACT_APP_AUTH0_CLIENT_ID  
Value: your-client-id-here
Environment: Production
```

### **Option B: Vercel CLI**
```bash
vercel env add REACT_APP_AUTH0_DOMAIN
# When prompted, enter: your-domain.auth0.com

vercel env add REACT_APP_AUTH0_CLIENT_ID
# When prompted, enter: your-client-id
```

---

## 📧 Step 4: Allow Your Email Address

Edit the file `src/components/AccessControl.tsx`:

**Find this line (around line 8):**
```typescript
const ALLOWED_EMAILS = [
  'your-email@example.com', // Replace with your actual email
  'another-email@example.com', // Add more emails as needed
];
```

**Replace with your actual emails:**
```typescript
const ALLOWED_EMAILS = [
  'your-actual-email@gmail.com',    // Your email
  'colleague@company.com',          // Anyone else who needs access
];
```

**Save the file.**

---

## 🚀 Step 5: Deploy

```bash
vercel --prod
```

**Wait for deployment to complete (~2 minutes)**

---

## ✅ Step 6: Test Your Setup

1. **Visit your deployed URL**: https://collaborative-intelligence-o7bw7ep8w-claude-codes-projects.vercel.app

2. **You should see a login screen** with "Agent Explorer" branding

3. **Click "Sign In to Continue"**

4. **Auth0 will prompt you to sign up/sign in**

5. **Use an email from your ALLOWED_EMAILS list**

6. **If successful**: You'll see the Agent Explorer interface with your user profile in the top-right

7. **If "Access Denied"**: Check that your email is exactly in the ALLOWED_EMAILS array

---

## 🎉 You're Done!

Your Agent Explorer now has:
- ✅ **Secure login required**
- ✅ **Only your approved emails can access**
- ✅ **Professional authentication UI**
- ✅ **User profile with logout**

---

## 🆘 Troubleshooting

### **"Configuration Error" or blank page**
- Check that environment variables are set correctly in Vercel
- Redeploy: `vercel --prod`

### **"Access Denied" for your own email**
- Check spelling in `ALLOWED_EMAILS` array
- Email addresses are case-sensitive
- Make sure you saved the file and redeployed

### **"Callback URL Mismatch" error**
- Verify the URLs in Auth0 settings match exactly:
  - `https://collaborative-intelligence-o7bw7ep8w-claude-codes-projects.vercel.app`
  - `http://localhost:3000`

### **Login page doesn't appear**
- Check Auth0 Domain and Client ID in Vercel environment variables
- Make sure you're using the correct Auth0 application (SPA type)

---

## 🔧 Adding More Users Later

To give access to additional people:

1. **Edit `src/components/AccessControl.tsx`**
2. **Add their email to the ALLOWED_EMAILS array**
3. **Deploy**: `vercel --prod`
4. **They can now sign in with their email**

---

## 🔐 Remove Authentication Later

To make the app public again:

1. **Edit `src/App.tsx`**
2. **Remove the `<Auth0ProviderWrapper>` and `<AccessControl>` wrappers**
3. **Deploy**: `vercel --prod`

---

**That's it! Your Agent Explorer is now secured with professional authentication.** 🔒✨

**Total time: ~10 minutes**