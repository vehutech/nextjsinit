# nextjsinit

A blazingly fast, intelligent CLI tool for scaffolding Next.js projects with automatic GitHub repo creation and optional Vercel deployment. Written in Rust 🦀

## 🚀 Features

- **Smart detection** - Automatically detects existing components (Next.js, Git, GitHub, Vercel)
- **Git-GitHub validation** - Verifies local Git is connected to GitHub before deployment
- **Push verification** - Checks for unpushed commits and offers to push automatically
- **Interactive mode** - Choose specific operations when no project name is provided
- **Zero-config setup** - Create a Next.js project with one command
- **Automatic GitHub integration** - Creates and pushes to a new GitHub repository
- **Public/Private repo selection** - Choose repository visibility interactively
- **Optional Vercel deployment** - Deploy to production with URL extraction
- **Organized workspace** - All projects stored in `~/dev/nextjs/`
- **Idempotent operations** - Safely skip already-completed steps
- **Input validation** - Validates all user responses before proceeding
- **Robust Next.js detection** - Supports `.js`, `.mjs`, and `.ts` config files
- **Zero runtime dependencies** - Single compiled binary, no interpreter needed

## 📋 Prerequisites

Before using `nextjsinit`, ensure you have the following installed:

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

**Note:** The `nextjsinit` binary itself has no runtime dependencies once compiled!

## 🔧 Installation

### Prerequisites for Building

- **Rust & Cargo** - Install from [rustup.rs](https://rustup.rs)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Method 1: Install from Source

```bash
# Clone the repository
git clone https://github.com/vehutech/nextjsinit.git
cd nextjsinit

# Build and install (automatically installs to ~/.cargo/bin)
cargo install --path .

# Make sure ~/.cargo/bin is in your PATH
# Add to ~/.zshrc or ~/.bashrc:
# export PATH="$HOME/.cargo/bin:$PATH"
```

### Method 2: Build and Copy Manually

```bash
# Clone the repository
git clone https://github.com/vehutech/nextjsinit.git
cd nextjsinit

# Build release binary
cargo build --release

# Copy to system path
sudo cp target/release/nextjsinit /usr/local/bin/

# Verify installation
which nextjsinit
```

### Method 3: Download Pre-built Binary (Coming Soon)

```bash
# Download from GitHub releases
curl -L https://github.com/vehutech/nextjsinit/releases/latest/download/nextjsinit-macos -o nextjsinit
chmod +x nextjsinit
sudo mv nextjsinit /usr/local/bin/
```

### Renaming from Previous Version

If you have the Lua version of `newnextjs` or `nextjsinit` installed:

```bash
# Remove old version
sudo rm /usr/local/bin/newnextjs
sudo rm /usr/local/bin/nextjsinit  # if Lua version exists

# Install new Rust version as shown above
```

## 📖 Usage

### Basic Command

```bash
nextjsinit <project-name>
```

### Interactive Mode (No Arguments)

```bash
nextjsinit
```

When run without arguments, `nextjsinit` enters interactive mode:

**If you're in a Next.js project directory:**
```
🔍 Detected current project: my-app

🎯 Interactive Mode
📁 Current location: /Users/you/dev/nextjs/my-app

📊 Current Status:
  Next.js Project: ✅
  Git Initialized: ✅
  GitHub Repo: ✅
  Git → GitHub Connected: ✅
  All Changes Pushed: ⚠️
  Vercel Deployed: ❌

What would you like to do?
1. Full setup (Next.js + Git + GitHub + Vercel)
2. Next.js project bootstrapping only
3. Git initialization only
4. GitHub repository creation only
5. Vercel deployment only
6. Exit

Enter your choice (1-6):
```

**If you're not in a project directory:**
```
🤔 No project name provided and not in a Next.js project directory.

Enter project name: my-new-app
```

### Example Workflows

#### 1. Create New Project (Full Setup)

```bash
nextjsinit my-awesome-app
```

**Output:**
```
🚀 Initializing project: my-awesome-app
📁 Location: /Users/you/dev/nextjs/my-awesome-app

📁 Creating project directory...

🚀 Creating Next.js project: my-awesome-app
[Next.js setup output...]
✅ Next.js project created successfully!

📦 Initializing Git repository...
✅ Git initialized successfully!

🐙 Creating GitHub repository...
Should the GitHub repository be public? (y/n): y
✅ GitHub repository created and pushed successfully!

Would you like to deploy this project to Vercel now? (y/n): y

🌍 Deploying to Vercel...
✅ Successfully deployed to Vercel!
🔗 URL: https://my-awesome-app-xyz123.vercel.app

✅ Project 'my-awesome-app' setup complete!
```

#### 2. Add GitHub to Existing Project

```bash
cd ~/dev/nextjs/existing-project
nextjsinit
# Choose option 4: GitHub repository creation only
```

#### 3. Deploy Existing Project to Vercel

```bash
cd ~/dev/nextjs/my-project
nextjsinit
# Choose option 5: Vercel deployment only
```

**Smart validation:**
```
❌ Git is not initialized. Please initialize Git first (option 3).

# After fixing Git:
❌ GitHub repository doesn't exist. Please create it first (option 4).

# After fixing GitHub:
❌ Local Git is not connected to GitHub.
   Run: git remote add origin https://github.com/YOUR_USERNAME/my-project.git

# After connecting:
⚠️  Warning: You have unpushed changes.
Push changes to GitHub before deploying? (y/n): y

📤 Pushing changes to GitHub...
✅ Changes pushed successfully!

🌍 Deploying to Vercel...
✅ Successfully deployed to Vercel!
🔗 URL: https://my-project-xyz123.vercel.app
```

#### 4. Skip Existing Components

```bash
nextjsinit existing-project
```

**Output:**
```
📁 Project directory already exists.
⚠️  Next.js project already exists, skipping creation.
⚠️  Git already initialized, skipping.
⚠️  GitHub repository already exists, skipping creation.

Would you like to deploy this project to Vercel now? (y/n): n
🛑 Skipping Vercel deployment.

✅ Project 'existing-project' setup complete!
```

## 🎯 Smart Features

### 1. Automatic Detection

`nextjsinit` automatically detects:
- Existing project directories
- Next.js project configuration (`.js`, `.mjs`, `.ts` variants)
- Git initialization status
- GitHub repository existence
- Git remote connection to GitHub
- Unpushed commits
- Vercel deployment status

### 2. Pre-deployment Validation

Before Vercel deployment, `nextjsinit` verifies:
1. ✅ Vercel CLI is installed
2. ✅ Git is initialized
3. ✅ GitHub repository exists
4. ✅ Local Git is connected to the correct GitHub repo
5. ✅ All changes are pushed (or offers to push automatically)

This prevents deployment failures and ensures code consistency.

### 3. Input Validation

All user inputs are validated:
- Yes/no prompts only accept `y/Y/n/N`
- Menu choices must be valid numbers
- Invalid inputs trigger helpful error messages

### 4. Idempotent Operations

Running `nextjsinit` multiple times safely skips completed steps:
- Won't recreate existing directories
- Won't reinitialize Git if already done
- Won't create duplicate GitHub repositories
- Warns before redeploying to Vercel

### 5. Context Awareness

When run without arguments, `nextjsinit`:
- Detects if you're in a project directory
- Shows current project status with connection details
- Offers relevant operations based on current state

## ⚙️ Configuration

### Change Base Directory

Edit line 4 in the script:

```lua
local baseDir = os.getenv("HOME") .. "/dev/nextjs"
-- Change to your preferred location:
local baseDir = os.getenv("HOME") .. "/projects"
```

### Customize Next.js Options

Modify the `createNextJsProject` function (around line 95):

```lua
local cmd = string.format("cd \"%s\" && npx create-next-app@latest ./ --typescript --tailwind --app", projectPath)
```

## 🗂️ Project Structure

After running `nextjsinit my-app`:

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

### "command not found: nextjsinit"

Ensure the script is in your `$PATH`:

```bash
echo $PATH
# Move script to a directory in PATH
sudo mv nextjsinit /usr/local/bin/
chmod +x /usr/local/bin/nextjsinit
```

### "gh: command not found"

Install GitHub CLI:

```bash
brew install gh  # macOS
sudo apt install gh  # Ubuntu/Debian
gh auth login
```

### "vercel: command not found"

```bash
npm install -g vercel
vercel login
```

### Permission Denied

```bash
chmod +x /usr/local/bin/nextjsinit
```

### GitHub Authentication Failed

```bash
gh auth status
gh auth login --web
```

### "Invalid input" Loop

Make sure you're entering exactly `y` or `n` (lowercase or uppercase) for yes/no prompts, and valid numbers for menu choices.

### Vercel Deployment Fails

Check the pre-deployment validation:
```bash
# Verify Git is connected to GitHub
git remote -v

# Should show:
# origin  https://github.com/username/project.git (fetch)
# origin  https://github.com/username/project.git (push)

# Check for unpushed commits
git status

# Push if needed
git push origin main
```

### "Local Git is not connected to GitHub"

```bash
# Add GitHub remote manually
git remote add origin https://github.com/YOUR_USERNAME/PROJECT_NAME.git

# Verify
git remote -v

# Push
git push -u origin main
```

## 🎮 Interactive Mode Options

| Option | Description | When to Use |
|--------|-------------|-------------|
| **1. Full setup** | Creates Next.js + Git + GitHub + Vercel | Starting a brand new project |
| **2. Next.js only** | Bootstraps Next.js project | You want to handle Git/GitHub manually |
| **3. Git only** | Initializes Git repository | Adding version control to existing project |
| **4. GitHub only** | Creates and pushes to GitHub | Publishing existing local project |
| **5. Vercel only** | Deploys to Vercel | Deploying already-configured project |
| **6. Exit** | Quits the program | Changed your mind |

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

## 📝 License

MIT License - see [LICENSE](LICENSE) file for details

## 🙏 Acknowledgments

- [Next.js](https://nextjs.org/) - The React framework
- [Vercel](https://vercel.com/) - Deployment platform
- [GitHub CLI](https://cli.github.com/) - GitHub automation

## 📮 Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/nextjsinit/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/nextjsinit/discussions)

## 🔮 Roadmap

- [x] Smart component detection
- [x] Interactive mode
- [x] Public/private repo selection
- [x] Input validation
- [x] Vercel URL extraction
- [x] Git-GitHub connection validation
- [x] Unpushed commits detection
- [x] Automatic push before deployment
- [x] Robust Next.js config detection (.js/.mjs/.ts)
- [ ] Template support (blog, e-commerce, dashboard)
- [ ] Command-line flags (`--skip-github`, `--skip-vercel`, etc.)
- [ ] Integration with other platforms (Netlify, Railway)
- [ ] Configuration file support (`.nextjsinitrc`)
- [ ] Database initialization (Prisma, Supabase)
- [ ] Automated testing setup
- [ ] CI/CD pipeline templates
- [ ] Pre-commit hooks configuration
- [ ] Environment variable management

---
