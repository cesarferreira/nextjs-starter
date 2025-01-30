#!/bin/bash

# Check if project name is provided
if [ -z "$1" ]; then
    echo "Please provide a project name"
    echo "Usage: ./install.sh <project-name>"
    exit 1
fi

PROJECT_NAME=$1
REPO_URL="https://raw.githubusercontent.com/cesarferreira/nextjs-starter/main"

# Create Next.js app with Bun
bunx create-next-app@latest $PROJECT_NAME \
  --ts \
  --tailwind \
  --src-dir \
  --app \
  --turbopack \
  --import-alias="" \
  --use-bun

# Download and copy template files
echo "Downloading template files..."
mkdir -p $PROJECT_NAME/.github/workflows
curl -sSL "$REPO_URL/template/.github/workflows/deploy.yml" -o "$PROJECT_NAME/.github/workflows/deploy.yml"
curl -sSL "$REPO_URL/template/next.config.mjs" -o "$PROJECT_NAME/next.config.mjs"

echo "✅ Project setup complete! 🚀"
echo "To get started:"
echo "  cd $PROJECT_NAME"
echo "  bun dev"
