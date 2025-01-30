#!/bin/bash

bunx create-next-app@latest {{PROJECT_NAME}} \
  --ts \
  --tailwind \
  --src-dir \
  --app \
  --turbopack \
  --import-alias=""
  --use-bun

# copy template
cp -r template/{{PROJECT_NAME}} .
