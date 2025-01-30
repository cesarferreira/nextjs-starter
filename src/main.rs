use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;
use std::path::PathBuf;
use tokio::fs;
use regex::Regex;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::fs::Permissions;
use std::future::Future;
use std::pin::Pin;

const TEMPLATE_REPO: &str = "cesarferreira/nextjs-starter";
const TEMPLATE_BRANCH: &str = "main";

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Name of the project to create
    project_name: String,
}

fn validate_project_name(name: &str) -> Result<()> {
    let valid_name = Regex::new(r"^[a-zA-Z0-9-_]+$")?;
    if !valid_name.is_match(name) {
        anyhow::bail!("Project name can only contain letters, numbers, hyphens, and underscores");
    }
    Ok(())
}

fn check_bun_installation() -> Result<()> {
    let output = Command::new("bun")
        .arg("--version")
        .output()
        .context("Failed to check if Bun is installed. Please install Bun first: https://bun.sh")?;
    
    if !output.status.success() {
        anyhow::bail!("Bun is not installed. Please install it first: https://bun.sh");
    }
    Ok(())
}

async fn download_template_files(target_dir: &PathBuf) -> Result<()> {
    println!("{}", "📥 Downloading template files...".green());
    
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
            .template("{spinner:.green} {msg}")?,
    );
    pb.set_message("Fetching template files...");

    // Create a temporary directory for the template
    let temp_dir = tempfile::tempdir()?;
    
    // Clone the template repository
    let status = Command::new("git")
        .args(["clone", "--depth", "1", "--branch", TEMPLATE_BRANCH, 
              &format!("https://github.com/{}", TEMPLATE_REPO),
              temp_dir.path().to_str().unwrap()])
        .status()
        .context("Failed to clone template repository")?;

    if !status.success() {
        anyhow::bail!("Failed to download template files");
    }

    // Copy template files from the template directory inside the cloned repo
    let template_path = temp_dir.path().join("template");
    if !template_path.exists() {
        anyhow::bail!("Template directory not found in the repository");
    }

    // Copy template files
    fn copy_dir_recursive<'a>(
        from: PathBuf,
        to: PathBuf,
        pb: &'a ProgressBar,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
        Box::pin(async move {
            if !from.exists() {
                return Ok(());
            }

            if !to.exists() {
                fs::create_dir_all(&to).await?;
            }

            let mut entries = fs::read_dir(from).await?;
            while let Some(entry) = entries.next_entry().await? {
                let ft = entry.file_type().await?;
                let from_path = entry.path();
                let to_path = to.join(entry.file_name());
                
                pb.set_message(format!("Copying: {}", entry.file_name().to_string_lossy()));

                if ft.is_dir() {
                    copy_dir_recursive(from_path, to_path, pb).await?;
                } else {
                    fs::copy(&from_path, &to_path).await
                        .with_context(|| format!("Failed to copy {} to {}", 
                            from_path.display(), to_path.display()))?;
                    
                    // Preserve file permissions
                    #[cfg(unix)]
                    {
                        if let Ok(metadata) = std::fs::metadata(&from_path) {
                            let mode = metadata.permissions().mode();
                            tokio::fs::set_permissions(&to_path, Permissions::from_mode(mode)).await?;
                        }
                    }
                }
            }
            Ok(())
        })
    }

    // Copy files from template directory to target directory
    copy_dir_recursive(template_path, target_dir.clone(), &pb)
        .await
        .context("Failed to copy template files")?;

    pb.finish_with_message("Template files downloaded successfully!");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let project_name = cli.project_name;

    // Validate project name
    validate_project_name(&project_name)?;

    // Check if Bun is installed
    check_bun_installation()?;

    // Check if project directory already exists
    let project_path = PathBuf::from(&project_name);
    if project_path.exists() {
        anyhow::bail!("Directory {} already exists", project_name);
    }

    println!("{}", "🚀 Creating Next.js project...".green());

    // Create Next.js project
    let status = Command::new("bunx")
        .arg("create-next-app@latest")
        .arg(&project_name)
        .arg("--ts")
        .arg("--tailwind")
        .arg("--src-dir")
        .arg("--app")
        .arg("--turbopack")
        .arg("--import-alias")
        .arg("@/*")
        .arg("--use-bun")
        .arg("--eslint")
        .arg("-y")
        .status()
        .context("Failed to execute create-next-app")?;

    if !status.success() {
        // Clean up if create-next-app failed
        if project_path.exists() {
            fs::remove_dir_all(&project_path).await
                .context("Failed to clean up project directory after error")?;
        }
        anyhow::bail!("Failed to create Next.js project");
    }

    // Download and copy template files
    download_template_files(&project_path)
        .await
        .context("Failed to setup template files")?;

    println!("\n{}", "✅ Project setup complete!".green().bold());
    println!(
        "\n{} {}",
        "To get started, run:".cyan(),
        format!("cd {} && bun dev", project_name).white()
    );

    Ok(())
} 