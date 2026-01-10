use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, exit};

struct ProjectPaths {
    base_dir: PathBuf,
    project_path: PathBuf,
    project_name: String,
}

struct ProjectStatus {
    has_nextjs: bool,
    has_git: bool,
    has_github: bool,
    has_vercel: bool,
}

fn main() {
    let home = env::var("HOME").expect("HOME environment variable not set");
    let base_dir = PathBuf::from(home).join("dev/nextjs");
    
    // Ensure base directory exists
    fs::create_dir_all(&base_dir).ok();
    
    // Check for required tools
    if !command_exists("gh") {
        eprintln!("❌ GitHub CLI (gh) not found. Install it first!");
        eprintln!("   macOS: brew install gh");
        eprintln!("   Ubuntu: sudo apt install gh");
        exit(1);
    }
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        // No project name provided - enter interactive mode
        handle_no_args(&base_dir);
    } else {
        // Project name provided - full workflow
        let project_name = &args[1];
        let project_path = base_dir.join(project_name);
        
        let paths = ProjectPaths {
            base_dir,
            project_path: project_path.clone(),
            project_name: project_name.to_string(),
        };
        
        run_full_workflow(paths);
    }
}

fn handle_no_args(base_dir: &Path) {
    // Try to detect current project
    if let Ok(current_dir) = env::current_dir() {
        if current_dir.starts_with(base_dir) {
            if let Some(project_name) = current_dir.file_name() {
                let project_name = project_name.to_string_lossy().to_string();
                println!("🔍 Detected current project: {}", project_name);
                
                let paths = ProjectPaths {
                    base_dir: base_dir.to_path_buf(),
                    project_path: current_dir.clone(),
                    project_name: project_name.clone(),
                };
                
                interactive_mode(paths);
                return;
            }
        }
    }
    
    // Not in a project directory
    println!("🤔 No project name provided and not in a Next.js project directory.\n");
    print!("Enter project name: ");
    io::stdout().flush().unwrap();
    
    let mut project_name = String::new();
    io::stdin().read_line(&mut project_name).unwrap();
    let project_name = project_name.trim().to_string();
    
    if project_name.is_empty() {
        eprintln!("❌ Project name cannot be empty.");
        exit(1);
    }
    
    let project_path = base_dir.join(&project_name);
    
    if project_path.exists() {
        println!("📁 Project directory already exists: {}", project_path.display());
    } else {
        println!("📁 Creating new project directory...");
        fs::create_dir_all(&project_path).unwrap();
    }
    
    let paths = ProjectPaths {
        base_dir: base_dir.to_path_buf(),
        project_path,
        project_name,
    };
    
    interactive_mode(paths);
}

fn interactive_mode(paths: ProjectPaths) {
    println!("\n🎯 Interactive Mode");
    println!("📁 Current location: {}", paths.project_path.display());
    
    let status = check_project_status(&paths);
    
    println!("\n📊 Current Status:");
    println!("  Next.js Project: {}", if status.has_nextjs { "✅" } else { "❌" });
    println!("  Git Initialized: {}", if status.has_git { "✅" } else { "❌" });
    println!("  GitHub Repo: {}", if status.has_github { "✅" } else { "❌" });
    println!("  Vercel Deployed: {}", if status.has_vercel { "✅" } else { "❌" });
    
    println!("\nWhat would you like to do?");
    println!("1. Full setup (Next.js + Git + GitHub + Vercel)");
    println!("2. Next.js project bootstrapping only");
    println!("3. Git initialization only");
    println!("4. GitHub repository creation only");
    println!("5. Vercel deployment only");
    println!("6. Exit");
    
    let choice = get_menu_choice(1, 6);
    
    match choice {
        1 => {
            if !status.has_nextjs { create_nextjs_project(&paths); }
            if !status.has_git { init_git(&paths); }
            if !status.has_github { create_github_repo(&paths); }
            if command_exists("vercel") && get_yes_no("\nDeploy to Vercel?") {
                deploy_to_vercel(&paths);
            }
        }
        2 => {
            if status.has_nextjs {
                println!("⚠️  Next.js project already exists!");
            } else {
                create_nextjs_project(&paths);
            }
        }
        3 => {
            if status.has_git {
                println!("⚠️  Git is already initialized!");
            } else {
                init_git(&paths);
            }
        }
        4 => {
            if status.has_github {
                println!("⚠️  GitHub repository already exists!");
            } else {
                if !status.has_git {
                    println!("⚠️  Git not initialized. Initializing first...");
                    init_git(&paths);
                }
                create_github_repo(&paths);
            }
        }
        5 => {
            if !command_exists("vercel") {
                println!("❌ Vercel CLI not found. Install it with: npm install -g vercel");
                return;
            }
            if status.has_vercel {
                if get_yes_no("⚠️  Project might already be deployed. Deploy again?") {
                    deploy_to_vercel(&paths);
                }
            } else {
                deploy_to_vercel(&paths);
            }
        }
        6 => {
            println!("👋 Goodbye!");
            exit(0);
        }
        _ => unreachable!(),
    }
}

fn run_full_workflow(paths: ProjectPaths) {
    println!("\n🚀 Initializing project: {}", paths.project_name);
    println!("📁 Location: {}\n", paths.project_path.display());
    
    // Check if directory exists
    if paths.project_path.exists() {
        println!("📁 Project directory already exists.");
    } else {
        println!("📁 Creating project directory...");
        fs::create_dir_all(&paths.project_path).unwrap();
    }
    
    let status = check_project_status(&paths);
    
    // Next.js setup
    if status.has_nextjs {
        println!("⚠️  Next.js project already exists, skipping creation.");
    } else if !create_nextjs_project(&paths) {
        exit(1);
    }
    
    // Git initialization
    if status.has_git {
        println!("⚠️  Git already initialized, skipping.");
    } else if !init_git(&paths) {
        exit(1);
    }
    
    // GitHub repository
    if status.has_github {
        println!("⚠️  GitHub repository already exists, skipping creation.");
    } else if !create_github_repo(&paths) {
        println!("⚠️  Continuing without GitHub repository...");
    }
    
    // Vercel deployment
    if command_exists("vercel") {
        if get_yes_no("\nWould you like to deploy this project to Vercel now?") {
            deploy_to_vercel(&paths);
        } else {
            println!("\n🛑 Skipping Vercel deployment.\n");
        }
    } else {
        println!("\n⚠️  Vercel CLI not found. Skipping deployment.");
        println!("   Install with: npm install -g vercel\n");
    }
    
    println!("✅ Project '{}' setup complete!\n", paths.project_name);
}

fn check_project_status(paths: &ProjectPaths) -> ProjectStatus {
    ProjectStatus {
        has_nextjs: check_nextjs_project(&paths.project_path),
        has_git: check_git_init(&paths.project_path),
        has_github: check_github_repo(&paths.project_name),
        has_vercel: check_vercel_deployment(&paths.project_path),
    }
}

fn check_nextjs_project(path: &Path) -> bool {
    // Must have package.json
    if !path.join("package.json").exists() {
        return false;
    }
    
    // Check for any Next.js config file variant
    let has_config = path.join("next.config.js").exists()
        || path.join("next.config.mjs").exists()
        || path.join("next.config.ts").exists();
    
    if has_config {
        return true;
    }
    
    // Fallback: check package.json for "next" dependency
    if let Ok(contents) = fs::read_to_string(path.join("package.json")) {
        return contents.contains("\"next\"");
    }
    
    false
}

fn check_git_init(path: &Path) -> bool {
    path.join(".git").exists()
}

fn check_github_repo(project_name: &str) -> bool {
    let output = Command::new("gh")
        .args(&["repo", "view", project_name])
        .output();
    
    match output {
        Ok(result) => result.status.success(),
        Err(_) => false,
    }
}

fn check_vercel_deployment(path: &Path) -> bool {
    let output = Command::new("vercel")
        .args(&["ls"])
        .current_dir(path)
        .output();
    
    match output {
        Ok(result) => {
            let stdout = String::from_utf8_lossy(&result.stdout);
            if let Some(project_name) = path.file_name() {
                stdout.contains(&project_name.to_string_lossy().to_string())
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

fn create_nextjs_project(paths: &ProjectPaths) -> bool {
    println!("\n🚀 Creating Next.js project: {}", paths.project_name);
    
    let status = Command::new("npx")
        .args(&["create-next-app@latest", "./"])
        .current_dir(&paths.project_path)
        .status();
    
    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("✅ Next.js project created successfully!");
            true
        }
        _ => {
            println!("❌ Failed to create Next.js project.");
            false
        }
    }
}

fn init_git(paths: &ProjectPaths) -> bool {
    println!("\n📦 Initializing Git repository...");
    
    let commands = vec![
        vec!["git", "init"],
        vec!["git", "add", "."],
        vec!["git", "commit", "-m", "Initial Next.js setup"],
    ];
    
    for cmd in commands {
        let status = Command::new(cmd[0])
            .args(&cmd[1..])
            .current_dir(&paths.project_path)
            .status();
        
        if let Ok(exit_status) = status {
            if !exit_status.success() {
                println!("❌ Failed to initialize Git.");
                return false;
            }
        } else {
            println!("❌ Failed to initialize Git.");
            return false;
        }
    }
    
    println!("✅ Git initialized successfully!");
    true
}

fn create_github_repo(paths: &ProjectPaths) -> bool {
    println!("\n🐙 Creating GitHub repository...");
    
    let is_public = get_yes_no("Should the GitHub repository be public?");
    let visibility = if is_public { "--public" } else { "--private" };
    
    let status = Command::new("gh")
        .args(&["repo", "create", &paths.project_name, visibility, "--source=.", "--remote=origin", "--push"])
        .current_dir(&paths.project_path)
        .status();
    
    match status {
        Ok(exit_status) if exit_status.success() => {
            println!("✅ GitHub repository created and pushed successfully!");
            true
        }
        _ => {
            println!("❌ Failed to create GitHub repository.");
            false
        }
    }
}

fn deploy_to_vercel(paths: &ProjectPaths) -> bool {
    println!("\n🌍 Deploying to Vercel...");
    
    let output = Command::new("vercel")
        .args(&["--yes", "--prod", "--confirm"])
        .current_dir(&paths.project_path)
        .output();
    
    match output {
        Ok(result) => {
            let stdout = String::from_utf8_lossy(&result.stdout);
            print!("{}", stdout);
            
            // Extract URL
            if let Some(url) = extract_vercel_url(&stdout) {
                println!("\n✅ Successfully deployed to Vercel!");
                println!("🔗 URL: {}", url);
            } else {
                println!("\n✅ Deployment completed!");
            }
            true
        }
        Err(_) => {
            println!("❌ Failed to deploy to Vercel.");
            false
        }
    }
}

fn extract_vercel_url(output: &str) -> Option<String> {
    for line in output.lines() {
        if line.contains("https://") && line.contains(".vercel.app") {
            if let Some(start) = line.find("https://") {
                let url_part = &line[start..];
                if let Some(end) = url_part.find(|c: char| c.is_whitespace()) {
                    return Some(url_part[..end].to_string());
                } else {
                    return Some(url_part.to_string());
                }
            }
        }
    }
    None
}

fn command_exists(cmd: &str) -> bool {
    Command::new("command")
        .args(&["-v", cmd])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn get_yes_no(prompt: &str) -> bool {
    loop {
        print!("{} (y/n): ", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();
        
        match input.as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("❌ Invalid input. Please enter 'y' or 'n'."),
        }
    }
}

fn get_menu_choice(min: u32, max: u32) -> u32 {
    loop {
        print!("\nEnter your choice ({}-{}): ", min, max);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse::<u32>() {
            Ok(choice) if choice >= min && choice <= max => return choice,
            _ => println!("❌ Invalid choice. Please enter a number between {} and {}.", min, max),
        }
    }
}