# GitHub Pages Deployment Guide

This guide will help you deploy your Dioxus application to GitHub Pages.

## 🚀 Quick Start

1. **Push your changes to GitHub**:
   ```bash
   git add .
   git commit -m "Add GitHub Pages deployment"
   git push origin main
   ```

2. **Enable GitHub Pages**:
   - Go to your repository on GitHub
   - Navigate to Settings → Pages
   - Under "Source", select "Deploy from a branch"
   - Choose "gh-pages" branch
   - Click "Save"

3. **Wait for deployment**:
   - GitHub Actions will automatically build and deploy your app
   - Your site will be available at: `https://yourusername.github.io/labubu-assets`

## 📋 Prerequisites

- Your repository is on GitHub
- You have push access to the repository
- GitHub Actions are enabled (they are by default)

## 🔧 Manual Deployment

If you prefer to deploy manually:

```bash
# Build the application
just build

# Check deployment status
./check-deployment.sh

# Commit and push
git add docs/
git commit -m "Deploy to GitHub Pages"
git push origin main
```

## 🛠️ Build Process

The build process:
1. Uses `dx bundle --platform web --release --out-dir docs`
2. Moves files from `docs/public/` to `docs/`
3. Creates static files ready for GitHub Pages

## 📁 File Structure

After building, your `docs/` directory contains:
```
docs/
├── index.html          # Main HTML file
├── assets/             # Static assets (CSS, images)
├── wasm/               # WebAssembly files
└── CNAME              # Custom domain (optional)
```

## 🔄 Automatic Deployment

The GitHub Actions workflow (`.github/workflows/deploy.yml`) will:
1. Install Rust and Dioxus CLI
2. Build your application
3. Deploy to the `gh-pages` branch
4. Make it available on GitHub Pages

## 🌐 Custom Domain

To use a custom domain:
1. Add your domain in GitHub repository settings
2. Update the `docs/CNAME` file with your domain
3. The workflow will automatically include it in deployment

## 🐛 Troubleshooting

### Build fails
- Check that all dependencies are properly configured
- Ensure Rust toolchain is up to date
- Verify Dioxus CLI is installed

### Deployment doesn't work
- Check GitHub Actions logs for errors
- Ensure the `gh-pages` branch is created
- Verify GitHub Pages is enabled in repository settings

### Site not loading
- Check the GitHub Pages URL is correct
- Verify the `gh-pages` branch contains the built files
- Wait a few minutes for changes to propagate

## 📝 Scripts

- `./deploy.sh` - Build and prepare for deployment
- `./check-deployment.sh` - Check deployment status
- `just build` - Build the application manually

## 🔗 Useful Links

- [GitHub Pages Documentation](https://docs.github.com/en/pages)
- [Dioxus Documentation](https://dioxuslabs.com/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions) 