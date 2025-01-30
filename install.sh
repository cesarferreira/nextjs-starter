#!/bin/bash

# Check if project name is provided
if [ -z "$1" ]; then
    echo "Please provide a project name"
    echo "Usage: ./install.sh <project-name>"
    exit 1
fi

PROJECT_NAME=$1
REPO_URL="https://raw.githubusercontent.com/cesarferreira/nextjs-starter/main"

# Set environment variables for non-interactive mode
export NEXT_TELEMETRY_DISABLED=1

echo "Creating Next.js app..."

# Create temporary answers file
echo -e "no\nno" > /tmp/answers.txt

# Create Next.js app with Bun (non-interactive)
cat /tmp/answers.txt | bunx --bun create-next-app@latest $PROJECT_NAME \
  --typescript \
  --tailwind \
  --src-dir \
  --app \
  --use-bun

# Check if the project was created successfully
if [ ! -d "$PROJECT_NAME" ]; then
    echo "❌ Failed to create Next.js project"
    exit 1
fi

# Clean up
rm /tmp/answers.txt

echo "Downloading template files..."
mkdir -p $PROJECT_NAME/.github/workflows
curl -sSL "$REPO_URL/template/.github/workflows/deploy.yml" -o "$PROJECT_NAME/.github/workflows/deploy.yml"
curl -sSL "$REPO_URL/template/next.config.mjs" -o "$PROJECT_NAME/next.config.mjs"

if [ $? -eq 0 ]; then
    echo "✅ Project setup complete! 🚀"
    echo "To get started:"
    echo "  cd $PROJECT_NAME"
    echo "  bun dev"
else
    echo "❌ Failed to download template files"
    exit 1
fi
