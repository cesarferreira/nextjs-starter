# Nexto - Next.js Project Generator

A powerful CLI tool to create Next.js projects with opinionated best practices and configurations.

## Features

- ⚡️ **Next.js 14** with App Router
- 🎯 **TypeScript** for type safety
- 🎨 **Tailwind CSS** for styling
- 📦 **Bun** for faster package management and development
- 🚀 **Turbopack** for lightning-fast builds
- 🔄 **GitHub Actions** for CI/CD
- 📁 **Organized Project Structure** with src directory
- ⚙️ **Optimized Next.js configuration**
- 🔒 **Security best practices**

## Prerequisites

Before you begin, ensure you have the following installed:
- [Bun](https://bun.sh) (Required for package management and development)
- [Rust](https://rustup.rs) (Required for installing the CLI tool)

## Installation

Install the CLI tool globally using:

```bash
cargo install --git https://github.com/cesarferreira/nextjs-starter.git
```

## Usage

Create a new Next.js project anywhere on your system:

```bash
nexto my-project
```

Replace `my-project` with your desired project name (only letters, numbers, hyphens, and underscores are allowed).

## Project Structure

```
my-project/
├── src/
│   ├── app/           # App Router pages and layouts
│   ├── components/    # Reusable UI components
│   ├── lib/          # Utility functions and shared logic
│   └── styles/       # Global styles and Tailwind config
├── public/           # Static assets
├── .github/          # GitHub Actions workflows
└── next.config.mjs   # Next.js configuration
```

## Development

After creating your project:

```bash
cd my-project
bun dev
```

Then open [http://localhost:3000](http://localhost:3000) in your browser.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - feel free to use this template for any project.

## Support

If you find this tool helpful, please consider giving it a star ⭐️ on GitHub. 