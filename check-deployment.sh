#!/bin/bash

# Check deployment status script
echo "🔍 Checking deployment status..."

# Check if docs directory exists and has content
if [ -d "docs" ] && [ "$(ls -A docs)" ]; then
    echo "✅ docs/ directory exists with content"
    echo "📁 Contents of docs/:"
    ls -la docs/
    echo ""
    echo "📄 index.html exists: $(test -f docs/index.html && echo 'Yes' || echo 'No')"
    echo "📁 assets/ exists: $(test -d docs/assets && echo 'Yes' || echo 'No')"
    echo "📁 wasm/ exists: $(test -d docs/wasm && echo 'Yes' || echo 'No')"
    echo ""
    echo "🚀 Ready for deployment!"
    echo ""
    echo "📋 Next steps:"
    echo "1. git add ."
    echo "2. git commit -m 'Add GitHub Pages deployment'"
    echo "3. git push origin main"
    echo ""
    echo "🔗 After pushing, your site will be available at:"
    echo "   https://yourusername.github.io/labubu-assets"
else
    echo "❌ docs/ directory is missing or empty"
    echo "💡 Run 'just build' to generate the deployment files"
fi 