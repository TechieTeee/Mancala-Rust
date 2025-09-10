# Mancala Game - Vercel Deployment Guide

## 🚀 Quick Deploy to Vercel

Your Mancala game is now configured for Vercel deployment! Here's what was fixed:

### ✅ Issues Resolved

1. **Added `vercel.json`** - Proper configuration for WASM deployment
2. **Updated build script** - Uses pre-built WASM files for deployment
3. **Added `.vercelignore`** - Optimizes deployment by excluding unnecessary files
4. **Configured static file handling** - Proper headers for WASM and static assets

### 📁 Deployment Files Created

- `vercel.json` - Vercel configuration with proper WASM support
- `.vercelignore` - Excludes development files from deployment
- `DEPLOYMENT.md` - This deployment guide

### 🔧 Configuration Details

The `vercel.json` includes:
- Proper MIME types for WASM files (`application/wasm`)
- JavaScript file headers (`application/javascript`)
- Static asset caching
- SPA routing fallback

### 🚀 Deploy Steps

1. **Push to GitHub** (if not already done):
   ```bash
   git add .
   git commit -m "Fix Vercel deployment configuration"
   git push origin main
   ```

2. **Deploy to Vercel**:
   - Go to [vercel.com](https://vercel.com)
   - Import your GitHub repository
   - Vercel will automatically detect the configuration
   - Deploy!

### 🛠️ Local Development

For local development, use:
```bash
npm run dev    # Auto-rebuild on changes
npm run serve  # Start local server
```

### 📝 Notes

- The deployment uses pre-built WASM files from the `pkg/` directory
- Static assets (CSS, images, audio) are properly cached
- The game should work immediately after deployment
- No additional build steps required on Vercel

### 🐛 Troubleshooting

If deployment fails:
1. Check that all files in `pkg/` are committed to git
2. Verify `vercel.json` syntax is correct
3. Ensure `package.json` has the correct build script

Your Mancala game should now deploy successfully to Vercel! 🎮✨