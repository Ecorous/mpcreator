use anyhow::{bail, Context, Result};
use mpcreator::{change, input, Project};

fn clone(project: &Project) -> Result<()> {
    if project.path.try_exists()? && project.path.read_dir()?.count() != 0 {
        bail!(
            "\"{}\" exists and is not empty.",
            project.path.to_string_lossy()
        )
    }
    if !project.path.try_exists()? {
        std::fs::create_dir_all(&project.path)?;
    }
    git2::Repository::clone(&project.template_data.repository_url, &project.path)?;
    Ok(())
}

fn main() -> Result<()> {
    let (settings, project) = input::input()?;

    if settings.verbose {
        println!("Loader: {}", project.loader);
        println!("Lang: {}", project.lang);
    }
    clone(&project).context("Error while cloning template")?;
    if settings.verbose {
        println!("Cloned template to: {}", project.path.display());
    }

    change::change(&settings, &project)?;
    if settings.verbose {
        println!("REMOVE: com.example")
    }
    //std::fs::rmdir();
    println!("done");
    Ok(())
}
