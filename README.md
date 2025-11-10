#nextjsinit

A blazingly fast CLI tool for scaffolding Next.js projects with automatic GitHub repo creation and optional Vercel deployment.

## 🚀 Features

- **Zero-config setup** - Create a Next.js project with one command
- **Automatic GitHub integration** - Creates and pushes to a new GitHub repository
- **Optional Vercel deployment** - Deploy to production immediately after setup
- **Organized workspace** - All projects stored in `~/dev/nextjs/`
- **Interactive prompts** - User-friendly deployment workflow

## 📋 Prerequisites

Before using `newnextjs`, ensure you have the following installed:

- **Lua 5.1+** - The script runtime
- **Node.js & npm** - Required for Next.js
- **Git** - Version control
- **GitHub CLI (`gh`)** - For automatic repo creation
  ```bash
  # Install GitHub CLI
  brew install gh  # macOS
  # or
  sudo apt install gh  # Ubuntu/Debian
  
  # Authenticate
  gh auth login
  ```
- **Vercel CLI** (optional) - For deployment
  ```bash
  npm install -g vercel
  vercel login
  ```

## 🔧 Installation

### Method 1: Direct Download

```bash
# Download the script
curl -O https://raw.githubusercontent.com/yourusername/newnextjs/main/newnextjs

# Make it executable
chmod +x newnextjs

# Move to your PATH
sudo mv newnextjs /usr/local/bin/
```

### Method 2: Clone Repository

```bash
git clone https://github.com/yourusername/newnextjs.git
cd newnextjs
chmod +x newnextjs
sudo ln -s $(pwd)/newnextjs /usr/local/bin/newnextjs
```

## 📖 Usage

### Basic Command

```bash
newnextjs <project-name>
```

### Example

```bash
newnextjs my-awesome-app
```

### What Happens

1. **Creates project directory** at `~/dev/nextjs/my-awesome-app`
2. **Scaffolds Next.js** using `create-next-app@latest`
3. **Initializes Git** repository
4. **Creates GitHub repository** (public by default)
5. **Pushes initial commit** to GitHub
6. **Prompts for Vercel deployment** (optional)

### Interactive Flow

```
🚀 Creating new Next.js project: my-awesome-app
📁 Location: /Users/you/dev/nextjs/my-awesome-app

[Next.js setup output...]
[GitHub repo creation output...]

Would you like to deploy this project to Vercel now? (y/n): y

🌍 Deploying to Vercel...

✅ Successfully deployed to Vercel!
✅ Project 'my-awesome-app' created and pushed to GitHub successfully!
```

## ⚙️ Configuration

### Change Base Directory

Edit line 3 in the script:

```lua
local baseDir = os.getenv("HOME") .. "/dev/nextjs"
-- Change to your preferred location:
local baseDir = os.getenv("HOME") .. "/projects"
```

### Private GitHub Repositories

Replace `--public` with `--private` on line 24:

```lua
gh repo create %s --private --source=. --remote=origin --push
```

### Auto-deploy Without Prompt

Replace the interactive deployment section with:

```lua
print("\n🌍 Auto-deploying to Vercel...\n")
local vercelCommand = string.format("cd \"%s\" && vercel --yes --prod --confirm", projectPath)
os.execute(vercelCommand)
```

## 🎯 Advanced Usage

### With Custom Next.js Options

The script uses default `create-next-app` settings. To customize, modify line 17:

```lua
npx create-next-app@latest ./ --typescript --tailwind --app --src-dir
```

### Multiple Next.js Versions

```bash
# For specific version
npx create-next-app@14.1.0 ./
```

### Skip Vercel Deployment Prompt

Press `n` when prompted, or modify the script to auto-skip.

## 🗂️ Project Structure

After running `newnextjs my-app`:

```
~/dev/nextjs/
└── my-app/
    ├── .git/
    ├── .gitignore
    ├── next.config.js
    ├── package.json
    ├── README.md
    ├── app/
    ├── public/
    └── node_modules/
```

## 🐛 Troubleshooting

### "command not found: newnextjs"

Ensure the script is in your `$PATH`:

```bash
echo $PATH
# Move script to a directory in PATH
sudo mv newnextjs /usr/local/bin/
```

### "gh: command not found"

Install GitHub CLI:

```bash
brew install gh  # or appropriate package manager
gh auth login
```

### "vercel: command not found"

```bash
npm install -g vercel
vercel login
```

### Permission Denied

```bash
chmod +x /usr/local/bin/newnextjs
```

### GitHub Authentication Failed

```bash
gh auth status
gh auth login --web
```

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Commit your changes**
   ```bash
   git commit -m "Add amazing feature"
   ```
4. **Push to the branch**
   ```bash
   git push origin feature/amazing-feature
   ```
5. **Open a Pull Request**

### Development Setup

```bash
git clone https://github.com/yourusername/newnextjs.git
cd newnextjs

# Test locally
./newnextjs test-project
```

## 📝 License

MIT License - see [LICENSE](LICENSE) file for details

## 🙏 Acknowledgments

- [Next.js](https://nextjs.org/) - The React framework
- [Vercel](https://vercel.com/) - Deployment platform
- [GitHub CLI](https://cli.github.com/) - GitHub automation

## 📮 Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/newnextjs/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/newnextjs/discussions)
- **Email**: your.email@example.com

## 🔮 Roadmap

- [ ] Add template support (blog, e-commerce, dashboard)
- [ ] Support for private/public repo selection via flag
- [ ] Integration with other deployment platforms (Netlify, Railway)
- [ ] Configuration file support (`.newnextjsrc`)
- [ ] Database initialization options (Prisma, Supabase)
- [ ] TypeScript/JavaScript preference prompt
- [ ] Automated testing setup
- [ ] CI/CD pipeline templates

## 📊 Project Stats

![GitHub stars](https://img.shields.io/github/stars/yourusername/newnextjs?style=social)
![GitHub forks](https://img.shields.io/github/forks/yourusername/newnextjs?style=social)
![GitHub issues](https://img.shields.io/github/issues/yourusername/newnextjs)
![GitHub license](https://img.shields.io/github/license/yourusername/newnextjs)

---

**Made with ❤️ by developers, for developers**
