use anyhow::{bail, Context, Result};
use mpcreator::{change, input, Language, Loader};
use std::path::PathBuf;

const QUILT_JAVA_GIT_TEMPLATE: &str = "https://github.com/QuiltMC/quilt-template-mod.git";
const QUILT_KOTLIN_GIT_TEMPLATE: &str = "https://github.com/QuiltMC/quilt-kotlin-template-mod.git";

fn clone(loader: &Loader, lang: &Language, final_path: &PathBuf) -> Result<()> {
    let url;
    if *loader == Loader::Quilt && *lang == Language::Java {
        url = QUILT_JAVA_GIT_TEMPLATE;
    } else if *loader == Loader::Quilt && *lang == Language::Kotlin {
        url = QUILT_KOTLIN_GIT_TEMPLATE;
    } else {
        bail!("Invalid combination of Loader + Lang");
    }
    if final_path.try_exists()? && final_path.read_dir()?.count() != 0 {
        bail!(
            "\"{}\" exists and is not empty.",
            final_path.to_string_lossy()
        )
    }
    if !final_path.try_exists()? {
        std::fs::create_dir_all(&final_path)?;
    }
    git2::Repository::clone(url, &final_path)?;
    Ok(())
}

fn main() -> Result<()> {
    let (settings, project) = input::input()?;

    if settings.verbose {
        println!("Loader: {}", project.loader);
        println!("Lang: {}", project.lang);
    }
    clone(&project.loader, &project.lang, &project.path).context("Error while cloning template")?;
    if settings.verbose {
        println!("Cloned template to: {}", project.path.display());
        println!("Removing template's git metadata...");
    }

    change::change(&settings, &project)?;
    if settings.verbose {
        println!("REMOVE: com.example")
    }
    //std::fs::rmdir();
    println!("done");
    Ok(())
}
