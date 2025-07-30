#!/bin/bash

# Deploy script for GitHub Pages
echo "🚀 Building and deploying to GitHub Pages..."

# Build the application
echo "📦 Building application..."
just build

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Build completed successfully!"
    echo "📁 Files ready for deployment in docs/ directory"
    echo ""
    echo "🔗 To deploy to GitHub Pages:"
    echo "1. Push your changes to the main branch"
    echo "2. The GitHub Actions workflow will automatically deploy to gh-pages branch"
    echo "3. Your site will be available at: https://yourusername.github.io/labubu-assets"
    echo ""
    echo "📝 Manual deployment (if needed):"
    echo "git add docs/"
    echo "git commit -m 'Deploy to GitHub Pages'"
    echo "git push origin main"
else
    echo "❌ Build failed!"
    exit 1
fi 