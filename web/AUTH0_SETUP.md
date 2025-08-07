# 🔐 Auth0 Setup Guide for Agent Explorer

This guide will help you set up Auth0 authentication with email allowlisting for your Agent Explorer.

## 📋 Prerequisites

1. **Auth0 Account**: Sign up at [auth0.com](https://auth0.com) if you don't have one
2. **Deployed Vercel App**: Your app should be deployed (which it is!)

## 🏗️ Auth0 Configuration

### Step 1: Create Auth0 Application

1. **Go to [Auth0 Dashboard](https://manage.auth0.com)**
2. **Applications** → **Create Application**
3. **Name**: `Agent Explorer` (or any name you prefer)
4. **Type**: Select **Single Page Web Applications**
5. **Click Create**

### Step 2: Configure Application Settings

In your new application's **Settings** tab:

#### **Basic Information**
- **Name**: Agent Explorer
- **Description**: Secure AI agent management interface

#### **Application URIs**
- **Allowed Callback URLs**: 
  ```
  https://collaborative-intelligence-o7bw7ep8w-claude-codes-projects.vercel.app,
  http://localhost:3000
  ```

- **Allowed Logout URLs**:
  ```
  https://collaborative-intelligence-o7bw7ep8w-claude-codes-projects.vercel.app,
  http://localhost:3000
  ```

- **Allowed Web Origins**:
  ```
  https://collaborative-intelligence-o7bw7ep8w-claude-codes-projects.vercel.app,
  http://localhost:3000
  ```

#### **Advanced Settings**
- **Grant Types**: Ensure these are checked:
  - ✅ Authorization Code
  - ✅ Refresh Token
  - ✅ Implicit

- **Click Save Changes**

### Step 3: Get Your Credentials

From the **Settings** tab, copy these values:

- **Domain**: `your-domain.auth0.com` 
- **Client ID**: `your-client-id-here`

## 🔧 Local Development Setup

### Step 1: Update Environment Variables

Edit your `.env` file:

```bash
# Replace with your actual Auth0 values
REACT_APP_AUTH0_DOMAIN=your-domain.auth0.com
REACT_APP_AUTH0_CLIENT_ID=your-client-id-here

# Keep existing values
GENERATE_SOURCEMAP=false
FAST_REFRESH=false
WDS_SOCKET_PORT=0
ESLINT_NO_DEV_ERRORS=true
TSC_COMPILE_ON_ERROR=true
DISABLE_ESLINT_PLUGIN=true
REACT_APP_WEBPACK_DEV_SERVER_OVERLAY=false
```

### Step 2: Configure Allowed Emails

Edit `src/components/AccessControl.tsx` and update the email allowlist:

```typescript
const ALLOWED_EMAILS = [
  'your-email@gmail.com',          // Your email
  'colleague@company.com',         // Add more emails as needed
  'admin@yourdomain.com',         // Anyone else who should have access
];
```

### Step 3: Test Locally

```bash
# Start development server
npm start

# Visit http://localhost:3000
# You should see the login screen
```

## 🚀 Production Deployment

### Step 1: Set Vercel Environment Variables

```bash
# Set Auth0 environment variables in Vercel
vercel env add REACT_APP_AUTH0_DOMAIN
# Enter your Auth0 domain when prompted

vercel env add REACT_APP_AUTH0_CLIENT_ID  
# Enter your Auth0 client ID when prompted
```

**Or via Vercel Dashboard:**
1. Go to [vercel.com](https://vercel.com)
2. Select your `collaborative-intelligence` project
3. **Settings** → **Environment Variables**
4. Add:
   - `REACT_APP_AUTH0_DOMAIN` = `your-domain.auth0.com`
   - `REACT_APP_AUTH0_CLIENT_ID` = `your-client-id`

### Step 2: Deploy with Authentication

```bash
# Deploy the updated app
vercel --prod
```

## ✅ What You Get

### 🔒 **Secure Access Control**
- **Login Required**: Users must sign in to access the app
- **Email Allowlist**: Only approved email addresses can access
- **Beautiful UI**: Professional login/logout screens
- **User Profile**: Shows who's logged in with logout option

### 🎨 **Login Experience**
- **Secure Login Screen**: Professional Auth0 authentication
- **Access Denied Screen**: Clear messaging for unauthorized users
- **User Profile Dropdown**: Shows logged-in user with logout option

### 🛡️ **Security Features**
- **Industry Standard**: Auth0 handles all authentication security
- **Token Management**: Secure token storage and refresh
- **Email Verification**: Users must have verified email addresses
- **Single Sign-On**: Works with Google, Microsoft, etc. (if configured)

## 🔧 Customization

### Adding More Email Addresses
Edit `src/components/AccessControl.tsx`:
```typescript
const ALLOWED_EMAILS = [
  'new-user@example.com',  // Add new emails here
];
```

### Enabling Social Logins
In Auth0 Dashboard:
1. **Authentication** → **Social** 
2. Enable Google, GitHub, Microsoft, etc.
3. Users can sign in with their social accounts

### Customizing Login Screen
The login UI is in `src/components/AccessControl.tsx` - modify the JSX for custom styling.

## 🚨 Important Notes

1. **Keep Credentials Secret**: Never commit real Auth0 credentials to git
2. **Environment Variables**: Use Vercel's environment variable system for production
3. **HTTPS Only**: Auth0 requires HTTPS in production (Vercel provides this automatically)
4. **Email Case Sensitivity**: Email addresses in the allowlist are case-sensitive

## 🆘 Troubleshooting

### "Domain/Client ID not found"
- Check your `.env` file has the correct Auth0 values
- Restart your development server after changing environment variables

### "Callback URL mismatch"
- Ensure your Vercel URL is added to Auth0 Callback URLs
- Check for typos in the URLs

### "Access Denied" for your own email
- Verify your email is in the `ALLOWED_EMAILS` array
- Check email spelling and case sensitivity
- Make sure your Auth0 account has a verified email

---

**Ready to secure your Agent Explorer!** 🔐✨