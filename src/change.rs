use std::path::{Path, PathBuf};

use anyhow::{Result, Context};

use crate::{RuntimeSettings, Project};

struct PathData {
    src_main: PathBuf,
    maven_g_path: String,
    lang_new_maven_path: PathBuf,
}

pub fn change(settings: &RuntimeSettings, project: &Project) -> Result<()> {

    remove_git_metadata(&project.path, settings).context("Error while removing git metadata")?;
    change_gradle_properties(&project, settings).context("Error while changing gradle.properties")?;

    let src_main = project.path.join("src").join("main");
    let maven_g_path = project.maven_group.replace(".", "/");
    let lang_new_maven_path = src_main
        .join(project.lang.to_src_string())
        .join(&maven_g_path)
        .join(&project.id);

    let path_data: PathData  = PathData { src_main, maven_g_path, lang_new_maven_path };

    move_source_code(&project, &settings, &path_data).context("Error while moving source code")?;
    handle_main_class(&project, &settings, &path_data)?;

    Ok(())
}

fn remove_git_metadata(project_path: &Path, settings: &RuntimeSettings) -> Result<()> {
    if settings.verbose {
        println!("Removing template's git metadata...");
    }
    std::fs::remove_dir_all(project_path.join(".git"))?;
    Ok(())
}

fn change_gradle_properties(project: &Project, settings: &RuntimeSettings) -> Result<()> {
    // gradle.properties
    if settings.verbose {
        println!("CHANGE: gradle.properties")
    };
    let mut gradle_properties = std::fs::read_to_string(project.path.join("gradle.properties"))?;
    gradle_properties = gradle_properties.replace(
        "maven_group = com.example",
        &format!("maven_group = {}", project.maven_group),
    );
    gradle_properties = gradle_properties.replace(
        "archives_base_name = example_mod",
        &format!("archives_base_name = {}", project.id),
    );
    std::fs::write(project.path.join("gradle.properties"), gradle_properties)?;
    Ok(())
}

fn move_source_code(project: &Project, settings: &RuntimeSettings, paths: &PathData) -> Result<()> {
    // change maven group dir
    if settings.verbose {
        println!("MOVE: com.example.example_mod -> {}", project.maven_group)
    }
    std::fs::create_dir_all(&paths.lang_new_maven_path)?;
    std::fs::rename(
        paths.src_main
            .join(project.lang.to_src_string())
            .join("com")
            .join("example")
            .join("example_mod"),
        &paths.lang_new_maven_path,
    )?;
    Ok(())
}

fn handle_main_class(project: &Project, settings: &RuntimeSettings, paths: &PathData) -> Result<()> {
    let main_class_file = paths.lang_new_maven_path.join(format!(
        "{}.{}",
        project.title,
        project.lang.file_extension()
    ));
    rename_main_class(project, settings, paths, &main_class_file).context("Error while renaming main class")?;
    change_main_class(project, settings, paths, &main_class_file).context("Error while changing main class")?;

    Ok(())
}


fn rename_main_class(project: &Project, settings: &RuntimeSettings, paths: &PathData, main_class_file: &PathBuf) -> Result<()> {
    if settings.verbose {
        println!(
            "MOVE: ExampleMod.{0} -> {1}.{0}",
            project.lang.file_extension(),
            project.title
        )
    }
    std::fs::rename(
        paths.lang_new_maven_path.join(format!(
            "ExampleMod.{}",
            project.lang.file_extension()
        )),
        &main_class_file,
    )?;
    Ok(())
}

fn change_main_class(project: &Project, settings: &RuntimeSettings, paths: &PathData, main_class_file: &PathBuf) -> Result<()> {
    if settings.verbose {
        println!(
            "CHANGE: ExampleMod.{}",
            project.lang.file_extension()
        )
    }
    let mut class_file = std::fs::read_to_string(&main_class_file)?;
    class_file = class_file.replace("ExampleMod", &project.title);
    class_file = class_file.replace(
        "package com.example.example_mod",
        &format!(
            "package {}.{}",
            project.maven_group,
            project.id
        ),
    );
    class_file = class_file.replace(
        "LoggerFactory.getLogger(\"Example Mod\")",
        &format!("LoggerFactory.getLogger(\"{}\")", project.title),
    );
    std::fs::write(&main_class_file, class_file)?;
    Ok(())
}