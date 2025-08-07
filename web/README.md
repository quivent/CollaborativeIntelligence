# Agent Explorer

A beautiful, modern AI agent management interface built with React, TypeScript, and Tailwind CSS. Browse and explore 106+ AI agents with rich metadata, capabilities, and documentation.

![Agent Explorer](https://img.shields.io/badge/version-1.0.0-blue.svg)
![React](https://img.shields.io/badge/React-19.1-blue.svg)
![TypeScript](https://img.shields.io/badge/TypeScript-4.9-blue.svg)
![Tailwind CSS](https://img.shields.io/badge/Tailwind-3.4-blue.svg)

## ✨ Features

### 🎨 **Beautiful Interface**
- **Two View modes**: Directory-style list view and 4x4 grid layout
- **20-color palette**: Soft, eye-friendly colors with consistent agent identification
- **Theme switcher**: Light, Dark, and Color modes with persistent preferences
- **Responsive design**: Works perfectly on all devices

### 🤖 **Rich Agent Data**
- **106 AI Agents** with detailed metadata and descriptions
- **Agent capabilities**: Specialized skills and areas of expertise
- **Usage statistics**: Activity counts, versions, and last-used dates
- **Component documentation**: Memory, Learning, and README files for each agent

### 📖 **Elegant Documentation**
- **Inline markdown rendering**: No popups - content flows naturally
- **Professional styling**: Beautiful typography with theme-aware colors
- **Interactive components**: Click to expand/collapse documentation
- **Loading states**: Smooth animations and feedback

### 🚀 **Modern Tech Stack**
- **React 19** with hooks and modern patterns
- **TypeScript** for type safety and developer experience
- **Tailwind CSS** for utility-first styling
- **React Router** for seamless navigation
- **React Markdown** for beautiful content rendering

## 🚀 Quick Start

### Prerequisites
- Node.js 16+ 
- npm or yarn

### Installation & Development
```bash
# Install dependencies
npm install

# Start development server
npm start
# Opens http://localhost:3000
```

### Production Build
```bash
# Build for production
npm run build

# Serve locally (install serve first: npm install -g serve)
serve -s build
```

## 🌐 Deploy to Vercel

### Option 1: Vercel CLI (Recommended)
```bash
# Install Vercel CLI
npm install -g vercel

# Deploy from this folder
vercel --prod
```

### Option 2: Vercel Dashboard
1. Compress this folder to a ZIP file
2. Go to [vercel.com](https://vercel.com)
3. Click "New Project" → "Import" → Upload ZIP
4. Deploy with default settings

### Option 3: Git Repository
1. Push this folder to a GitHub repository
2. Connect the repo to Vercel
3. Deploy automatically on push

## 📁 Project Structure

```
agent-explorer/
├── public/              # Static assets
│   ├── index.html      # Main HTML template
│   └── ...
├── src/
│   ├── components/     # React components
│   │   ├── DirectoryView.tsx  # List-style agent browser
│   │   ├── GridView.tsx       # Grid-style agent browser
│   │   ├── Navigation.tsx     # Top navigation bar
│   │   └── ThemeSwitcher.tsx  # Theme toggle component
│   ├── contexts/       # React contexts
│   │   └── ThemeContext.tsx   # Theme management
│   ├── services/       # Data services
│   │   └── agentService.ts    # Agent data & API simulation
│   ├── types/          # TypeScript definitions
│   ├── utils/          # Utility functions
│   └── ...
├── package.json        # Project configuration
└── README.md          # This file
```

## 🔐 Authentication (Optional)

The app includes **Auth0 authentication** with email allowlisting for secure access control.

### Quick Setup
**📖 [SETUP_AUTH0_NOW.md](SETUP_AUTH0_NOW.md) - Complete step-by-step guide (10 minutes)**

**Or Automated Setup:**
```bash
# 90% automated - still in development
npm run setup-auth0-full
```

### Features
- ✅ **Secure Login**: Professional Auth0 authentication screens
- ✅ **Email Allowlisting**: Only specific email addresses can access
- ✅ **User Profiles**: Shows who's logged in with logout option
- ✅ **Access Denied**: Clear messaging for unauthorized users

### Disable Authentication
To remove authentication and make the app publicly accessible:
1. **Remove Auth0 wrapper** from `App.tsx`
2. **Remove UserProfile** from `Navigation.tsx`  
3. **Redeploy** without Auth0 environment variables

## 🎨 Customization

### Adding New Agents
Edit `src/services/agentService.ts`:

```typescript
{
  name: 'YourAgent',
  metadata: {
    description: 'Your agent description',
    capabilities: ['capability1', 'capability2'],
    // ... other metadata
  },
  // ... other properties
}
```

### Modifying Colors
Edit `src/utils/colors.ts` to customize the color palette.

### Theme Customization  
Edit `src/index.css` and `src/contexts/ThemeContext.tsx` for theme modifications.

## 🔒 Security Features

- **MetaMask blocking**: Comprehensive crypto wallet popup prevention
- **No external dependencies**: All agent data is self-contained
- **Static deployment**: No backend required, pure client-side app
- **Type safety**: Full TypeScript implementation

## 📊 Agent Data

The application includes **106 AI agents** with:
- Detailed descriptions and capabilities
- Realistic usage statistics and metadata
- Three component types: Memory, Learning, and README
- Mock markdown content generation for demonstration

## 🛠 Development

### Available Scripts
- `npm start` - Development server
- `npm run build` - Production build  
- `npm test` - Run tests
- `npm run eject` - Eject from Create React App

### Technology Choices
- **React 19**: Latest React with concurrent features
- **TypeScript**: Full type safety and IntelliSense
- **Tailwind CSS**: Utility-first CSS framework
- **Create React App**: Zero-config build setup

## 🌟 Key Features in Detail

### Directory View
- Alphabetical agent listing with search
- Expandable agent details with inline documentation
- Component browsing with visual indicators
- Agent overview with statistics and capabilities

### Grid View  
- Visual 4-column agent cards
- Modal agent details with full information
- Inline component documentation
- Hover effects and smooth animations

### Theme System
- **Light Mode**: Clean, professional appearance
- **Dark Mode**: Easy on the eyes for extended use
- **Color Mode**: Vibrant, colorful agent differentiation
- **Persistent**: Remembers your preference across sessions

## 📱 Responsive Design

- **Desktop**: Full-featured experience with all components
- **Tablet**: Optimized layouts for medium screens  
- **Mobile**: Touch-friendly interface with adjusted navigation

## 🚀 Performance

- **Optimized bundle**: Tree-shaking and code splitting
- **Fast loading**: Minimal initial load time
- **Smooth animations**: Hardware-accelerated transitions
- **Efficient rendering**: React optimizations and memoization

## 📄 License

MIT License - feel free to use this project for any purpose.

## 🤝 Contributing

This is a standalone demonstration project. The agent data is mock/simulated for showcase purposes.

---

**Agent Explorer** - Beautiful AI agent management made simple ✨
