use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::{Language, Project, RuntimeSettings};

struct PathData {
    src_main: PathBuf,
    maven_g_path: String,
    lang_new_maven_path: PathBuf,
}

pub fn change(settings: &RuntimeSettings, project: &Project) -> Result<()> {
    remove_git_metadata(&project.path, settings).context("Error while removing git metadata")?;

    let src_main = project.path.join("src").join("main");
    let maven_g_path = project.maven_group.replace(".", "/");
    let lang_new_maven_path = src_main
        .join(project.lang.to_src_string())
        .join(&maven_g_path)
        .join(&project.id);

    let path_data: PathData = PathData {
        src_main,
        maven_g_path,
        lang_new_maven_path,
    };

    project.template_data.replacements
        .apply(project.path.clone(), &project, settings.verbose)
        .context("while replacing constants based on config file")?;

    move_source_code(&project, &settings, &path_data).context("Error while moving source code")?;

    let main_class_file = path_data.lang_new_maven_path.join(format!(
        "{}.{}",
        project.main_class_name,
        project.lang.file_extension()
    ));
    rename_main_class(project, settings, &path_data, &main_class_file)
        .context("Error while renaming main class")?;

    #[cfg(not(target_os = "windows"))]
    std::fs::create_dir_all(
        path_data
            .src_main
            .join("java")
            .join(&path_data.maven_g_path)
            .join(&project.id)
            .join("mixin"),
    )?;
    move_kotlin_mixin(project, settings, &path_data).context("Error while moving kotlin mixin")?;

    let resources_path: PathBuf = path_data.src_main.join("resources");

    move_assets(project, settings, &resources_path).context("Error while moving assets")?;
    move_mixin_json(project, settings, &resources_path).context("Error while moving mixin json")?;

    Ok(())
}

fn remove_git_metadata(project_path: &Path, settings: &RuntimeSettings) -> Result<()> {
    if settings.verbose {
        println!("Removing template's git metadata...");
    }
    std::fs::remove_dir_all(project_path.join(".git"))?;
    Ok(())
}

fn move_source_code(project: &Project, settings: &RuntimeSettings, paths: &PathData) -> Result<()> {
    // change maven group dir
    if settings.verbose {
        println!("MOVE: com.example.example_mod -> {}", project.maven_group)
    }
    #[cfg(not(target_os = "windows"))]
    std::fs::create_dir_all(&paths.lang_new_maven_path)?;
    std::fs::rename(
        paths
            .src_main
            .join(project.lang.to_src_string())
            .join("com")
            .join("example")
            .join("example_mod"),
        &paths.lang_new_maven_path,
    )?;
    Ok(())
}

fn rename_main_class(
    project: &Project,
    settings: &RuntimeSettings,
    paths: &PathData,
    main_class_file: &PathBuf,
) -> Result<()> {
    if settings.verbose {
        println!(
            "MOVE: ExampleMod.{0} -> {1}.{0}",
            project.lang.file_extension(),
            project.main_class_name
        )
    }
    std::fs::rename(
        paths
            .lang_new_maven_path
            .join(format!("ExampleMod.{}", project.lang.file_extension())),
        &main_class_file,
    )?;
    Ok(())
}

fn move_kotlin_mixin(
    project: &Project,
    settings: &RuntimeSettings,
    paths: &PathData,
) -> Result<()> {
    if project.lang == Language::Kotlin {
        if settings.verbose {
            println!(
                "MOVE: java/com.example.example_mod.mixin -> java/{}.{}.mixin",
                project.maven_group, project.id
            )
        }
        std::fs::rename(
            paths
                .src_main
                .join("java")
                .join("com")
                .join("example")
                .join("example_mod")
                .join("mixin"),
            paths
                .src_main
                .join("java")
                .join(&paths.maven_g_path)
                .join(&project.id)
                .join("mixin"),
        )?;
    }
    Ok(())
}

fn move_assets(
    project: &Project,
    settings: &RuntimeSettings,
    resources_path: &PathBuf,
) -> Result<()> {
    if settings.verbose {
        println!("MOVE: assets/example_mod -> assets/{}", project.id)
    }
    std::fs::rename(
        resources_path.join("assets").join("example_mod"),
        resources_path.join("assets").join(&project.id),
    )?;
    Ok(())
}

fn move_mixin_json(
    project: &Project,
    settings: &RuntimeSettings,
    resources_path: &PathBuf,
) -> Result<()> {
    if settings.verbose {
        println!(
            "MOVE: example_mod.mixins.json -> {}.mixins.json",
            project.id
        )
    }
    std::fs::rename(
        resources_path.join("example_mod.mixins.json"),
        resources_path.join(format!("{}.mixins.json", project.id)),
    )?;
    Ok(())
}
