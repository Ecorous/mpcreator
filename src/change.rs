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
    change_gradle_properties(&project, settings)
        .context("Error while changing gradle.properties")?;

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

    move_source_code(&project, &settings, &path_data).context("Error while moving source code")?;
    handle_main_class(&project, &settings, &path_data)?;
    handle_mixin(&project, &settings, &path_data)?;
    handle_resources(&project, &settings, &path_data)?;
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

fn handle_main_class(
    project: &Project,
    settings: &RuntimeSettings,
    paths: &PathData,
) -> Result<()> {
    let main_class_file = paths.lang_new_maven_path.join(format!(
        "{}.{}",
        project.title,
        project.lang.file_extension()
    ));
    rename_main_class(project, settings, paths, &main_class_file)
        .context("Error while renaming main class")?;
    change_main_class(project, settings, &main_class_file)
        .context("Error while changing main class")?;

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
            project.title
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

fn change_main_class(
    project: &Project,
    settings: &RuntimeSettings,
    main_class_file: &PathBuf,
) -> Result<()> {
    if settings.verbose {
        println!("CHANGE: ExampleMod.{}", project.lang.file_extension())
    }
    let mut class_file = std::fs::read_to_string(&main_class_file)?;
    class_file = class_file.replace("ExampleMod", &project.title);
    class_file = class_file.replace(
        "package com.example.example_mod",
        &format!("package {}.{}", project.maven_group, project.id),
    );
    class_file = class_file.replace(
        "LoggerFactory.getLogger(\"Example Mod\")",
        &format!("LoggerFactory.getLogger(\"{}\")", project.title),
    );
    std::fs::write(&main_class_file, class_file)?;
    Ok(())
}

fn handle_mixin(project: &Project, settings: &RuntimeSettings, paths: &PathData) -> Result<()> {
    std::fs::create_dir_all(
        paths
            .src_main
            .join("java")
            .join(&paths.maven_g_path)
            .join(&project.id)
            .join("mixin"),
    )?;

    handle_kotlin_mixin(project, settings, paths).context("Error while handling kotlin mixin")?;
    change_mixin(project, settings, paths).context("Error while changing mixin")?;

    Ok(())
}

fn handle_kotlin_mixin(
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

fn change_mixin(project: &Project, settings: &RuntimeSettings, paths: &PathData) -> Result<()> {
    fn rename_mixin_package(mixin_file: String, project: &Project) -> String {
        mixin_file.replace(
            "package com.example.example_mod.mixin",
            &format!("package {}.{}.mixin", project.maven_group, project.id),
        )
    }

    fn rename_main_class_import(mixin_file: String, project: &Project) -> String {
        mixin_file.replace(
            "import com.example.example_mod.ExampleMod",
            &format!(
                "import {}.{}.{}",
                project.maven_group, project.id, project.title
            ),
        )
    }

    fn rename_instance_use(mixin_file: String, project: &Project) -> String {
        if project.lang == Language::Java {
            mixin_file.replace("ExampleMod.LOGGER", &format!("{}.LOGGER", project.title))
        } else {
            mixin_file.replace(
                "ExampleMod.INSTANCE",
                &format!("{}.INSTANCE", project.title),
            )
        }
    }

    if settings.verbose {
        println!("CHANGE: mixin.TitleScreenMixin.java")
    }
    let mixin_path = paths
        .src_main
        .join("java")
        .join(&paths.maven_g_path)
        .join(&project.id)
        .join("mixin")
        .join("TitleScreenMixin.java");
    let mut mixin_file = std::fs::read_to_string(&mixin_path)?;
    mixin_file = rename_mixin_package(mixin_file, project);
    mixin_file = rename_main_class_import(mixin_file, project);
    mixin_file = rename_instance_use(mixin_file, project);
    mixin_file = mixin_file.replace("exampleMod$", &format!("{}$", project.id));
    std::fs::write(mixin_path, mixin_file)?;
    Ok(())
}

fn handle_resources(project: &Project, settings: &RuntimeSettings, paths: &PathData) -> Result<()> {
    let resources_path: PathBuf = paths.src_main.join("resources");

    change_mod_json(project, settings, &resources_path).context("Error while handling mod json")?;
    handle_mixin_json(project, settings, &resources_path)
        .context("Error while handling mixin json")?;

    Ok(())
}

fn change_mod_json(
    project: &Project,
    settings: &RuntimeSettings,
    resources_path: &PathBuf,
) -> Result<()> {
    if settings.verbose {
        println!("CHANGE: quilt.mod.json")
    }
    let mod_json = std::fs::read_to_string(resources_path.join("quilt.mod.json"))?;
    let mod_json = mod_json.replace(
        "\"name\": \"Mod Name\"",
        &format!("\"name\": \"{}\"", project.title),
    );
    let mod_json = mod_json.replace(
        "\"group\": \"com.example\"",
        &format!("\"group\": \"{}\"", project.maven_group),
    );
    let mod_json = mod_json.replace(
        "com.example.example_mod.ExampleMod",
        &format!("{}.{}.{}", project.maven_group, project.id, project.title),
    );
    let mod_json = mod_json.replace("example_mod", &project.id);
    std::fs::write(resources_path.join("quilt.mod.json"), mod_json)?;
    Ok(())
}

fn handle_mixin_json(
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

    if settings.verbose {
        println!("CHANGE: {}.mixins.json", project.id)
    }
    let mixin_json =
        std::fs::read_to_string(resources_path.join(format!("{}.mixins.json", project.id)))?;
    let mixin_json = mixin_json.replace(
        "com.example.example_mod.mixin",
        &format!("{}.{}.mixin", project.maven_group, project.id),
    );

    std::fs::write(
        resources_path.join(format!("{}.mixins.json", project.id)),
        mixin_json,
    )?;
    Ok(())
}
