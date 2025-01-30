#!/bin/bash

# Check if project name is provided
if [ -z "$1" ]; then
    echo "Please provide a project name"
    echo "Usage: ./install.sh <project-name>"
    exit 1
fi

PROJECT_NAME=$1

# Create Next.js app with Bun
bunx create-next-app@latest $PROJECT_NAME \
  --ts \
  --tailwind \
  --src-dir \
  --app \
  --turbopack \
  --import-alias="" \
  --use-bun

# Copy template files
echo "Copying template files..."
cp -r template/.github $PROJECT_NAME/
cp template/next.config.mjs $PROJECT_NAME/

echo "✅ Project setup complete! 🚀"
echo "To get started:"
echo "  cd $PROJECT_NAME"
echo "  bun dev"
