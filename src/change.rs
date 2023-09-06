use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use heck::{ToUpperCamelCase, ToLowerCamelCase};

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
        project.title.to_upper_camel_case(),
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
            project.title.to_upper_camel_case()
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
    class_file = project.replace_class_name(class_file);
    class_file = project.replace_group(class_file);
    class_file = project.replace_mod_id(class_file);
    class_file = project.replace_mod_name(class_file);
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
    mixin_file = project.replace_group(mixin_file);
    mixin_file = project.replace_mod_id(mixin_file);
    mixin_file = project.replace_mod_name(mixin_file);
    mixin_file = project.replace_class_name(mixin_file);
    mixin_file = project.replace_mod_prefix(mixin_file);
    std::fs::write(mixin_path, mixin_file)?;
    Ok(())
}

fn handle_resources(project: &Project, settings: &RuntimeSettings, paths: &PathData) -> Result<()> {
    let resources_path: PathBuf = paths.src_main.join("resources");

    change_mod_json(project, settings, &resources_path).context("Error while handling mod json")?;
    move_assets(project, settings, &resources_path).context("Error while moving assets")?;
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
        "Mod Name",
        &project.title,
    );
    let mod_json = project.replace_group(mod_json);
    let mod_json = project.replace_mod_id(mod_json);
    let mod_json = project.replace_mod_name(mod_json);
    std::fs::write(resources_path.join("quilt.mod.json"), mod_json)?;
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
    let mixin_json = project.replace_group(mixin_json);
    let mixin_json = project.replace_mod_id(mixin_json);

    std::fs::write(
        resources_path.join(format!("{}.mixins.json", project.id)),
        mixin_json,
    )?;
    Ok(())
}

impl Project {
    fn replace_group(&self, string: String) -> String {
        string.replace("com.example", &self.maven_group)
    }
    
    fn replace_mod_name(&self, string: String) -> String {
        string.replace("Example Mod", &self.title)
    }

    fn replace_mod_id(&self, string: String) -> String {
        string.replace("example_mod", &self.id)
    }

    fn replace_class_name(&self, string: String) -> String {
        string.replace("ExampleMod", &self.title.to_upper_camel_case())
    }

    fn replace_mod_prefix(&self, string: String) -> String {
        string.replace("exampleMod", &self.id.to_lower_camel_case())
    }
}