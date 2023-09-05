use anyhow::{bail, Context, Result};
use mpcreator::{input, Language, Loader, change};
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

    let src_main = project.path.join("src").join("main");
    let maven_g_path = project.maven_group.replace(".", "/");

    std::fs::create_dir_all(
        src_main
            .join("java")
            .join(&maven_g_path)
            .join(&project.id)
            .join("mixin"),
    )
    .unwrap();
    if project.lang == Language::Kotlin {
        if settings.verbose {
            println!(
                "MOVE: java/com.example.example_mod.mixin -> java/{}.{}.mixin",
                project.maven_group,
                project.id
            )
        }
        std::fs::rename(
            src_main
                .join("java")
                .join("com")
                .join("example")
                .join("example_mod")
                .join("mixin"),
            src_main
                .join("java")
                .join(&maven_g_path)
                .join(&project.id)
                .join("mixin"),
        )
        .unwrap();
    }
    if settings.verbose {
        println!("CHANGE: mixin.TitleScreenMixin.java")
    }
    let mut mixin_file = std::fs::read_to_string(
        src_main
            .join("java")
            .join(&maven_g_path)
            .join(&project.id)
            .join("mixin")
            .join("TitleScreenMixin.java"),
    )
    .unwrap();
    mixin_file = mixin_file.replace(
        "package com.example.example_mod.mixin",
        &format!(
            "package {}.{}.mixin",
            project.maven_group,
            project.id
        ),
    );
    mixin_file = mixin_file.replace(
        "import com.example.example_mod.ExampleMod",
        &format!(
            "import {}.{}.{}",
            project.maven_group,
            project.id,
            project.title
        ),
    );
    if project.lang == Language::Java {
        mixin_file = mixin_file.replace(
            "ExampleMod.LOGGER",
            &format!("{}.LOGGER", project.title),
        );
    } else {
        mixin_file = mixin_file.replace(
            "ExampleMod.INSTANCE",
            &format!("{}.INSTANCE", project.title),
        );
    }

    mixin_file = mixin_file.replace("exampleMod$", &format!("{}$", project.id));
    std::fs::write(
        src_main
            .join("java")
            .join(&maven_g_path)
            .join(&project.id)
            .join("mixin")
            .join("TitleScreenMixin.java"),
        mixin_file,
    )
    .unwrap();


    if settings.verbose {
        println!("CHANGE: quilt.mod.json")
    }
    let mod_json =
        std::fs::read_to_string(src_main.join("resources").join("quilt.mod.json")).unwrap();
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
        &format!(
            "{}.{}.{}",
            project.maven_group,
            project.id,
            project.title
        ),
    );
    let mod_json = mod_json.replace("example_mod", &project.id);
    std::fs::write(src_main.join("resources").join("quilt.mod.json"), mod_json).unwrap();
    if settings.verbose {
        println!(
            "MOVE: example_mod.mixins.json -> {}.mixins.json",
            project.id
        )
    }
    std::fs::rename(
        src_main.join("resources").join("example_mod.mixins.json"),
        src_main
            .join("resources")
            .join(format!("{}.mixins.json", project.id)),
    )
    .unwrap();
    let mixin_json = std::fs::read_to_string(
        src_main
            .join("resources")
            .join(format!("{}.mixins.json", project.id)),
    )
    .unwrap();
    let mixin_json = mixin_json.replace(
        "com.example.example_mod.mixin",
        &format!("{}.{}.mixin", project.maven_group, project.id),
    );
    std::fs::write(
        src_main
            .join("resources")
            .join(format!("{}.mixins.json", project.id)),
        mixin_json,
    )
    .unwrap();
    if settings.verbose {
        println!("REMOVE: com.example")
    }
    //std::fs::rmdir();
    println!("done");
    Ok(())
}
