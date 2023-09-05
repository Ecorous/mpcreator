use clap::Parser;
use git2::Repository;
use std::{
    fmt::{format, Display},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
enum Loader {
    Quilt = 1,
}

impl From<String> for Loader {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "quilt" | "q" => Loader::Quilt,
            _ => panic!("error: cannot convert String {} to Loader", value),
        }
    }
}

impl From<&str> for Loader {
    fn from(value: &str) -> Self {
        Loader::from(String::from(value))
    }
}

#[derive(Debug, Clone)]
enum Lang {
    Java,
    Kotlin,
}
impl From<String> for Lang {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "java" | "j" => Lang::Java,
            "kotlin" | "k" => Lang::Kotlin,
            _ => panic!("error: cannot convert String {} to Lang", value),
        }
    }
}

impl From<&str> for Lang {
    fn from(value: &str) -> Self {
        Lang::from(String::from(value))
    }
}

impl PartialEq for Lang {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl PartialEq for Loader {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Display for Loader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from(match self {
                Loader::Quilt => "Quilt",
            })
        )
    }
}

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from(match self {
                Lang::Java => "Java",
                Lang::Kotlin => "Kotlin",
            })
        )
    }
}

impl Lang {
    fn to_src_string(self) -> String {
        if self == Lang::Java {
            return String::from("java");
        } else {
            return String::from("kotlin");
        }
    }

    fn file_extension(self) -> String {
        if self == Lang::Java {
            return String::from("java");
        } else {
            return String::from("kt");
        }
    }
}

const QUILT_JAVA_GIT_TEMPLATE: &str = "https://github.com/QuiltMC/quilt-template-mod.git";
const QUILT_KOTLIN_GIT_TEMPLATE: &str = "https://github.com/QuiltMC/quilt-kotlin-template-mod.git";

#[derive(Parser, Debug)]
struct Command {
    #[arg(long)]
    loader: Option<Loader>,
    #[arg(long)]
    lang: Option<Lang>,
    #[arg(long)]
    projects_dir: Option<PathBuf>,
    #[arg(long)]
    title: Option<String>,
    #[arg(long)]
    id: Option<String>,
    #[arg(long)]
    maven_group: Option<String>,
    #[arg(long, short = 'v')]
    verbose: Option<bool>,
}

fn clone(loader: Loader, lang: Lang, projects_path: PathBuf, id: String) -> PathBuf {
    let url;
    if loader == Loader::Quilt && lang == Lang::Java {
        url = QUILT_JAVA_GIT_TEMPLATE;
    } else if loader == Loader::Quilt && lang == Lang::Kotlin {
        url = QUILT_KOTLIN_GIT_TEMPLATE;
    } else {
        panic!("Invalid combination of Loader + Lang")
    }
    let final_path = projects_path.join(id);
    if final_path.try_exists().unwrap() && final_path.read_dir().unwrap().count() != 0 {
        panic!(
            "\"{}\" exists and is not empty.",
            final_path.to_str().unwrap()
        )
    }
    if !final_path.try_exists().unwrap() {
        std::fs::create_dir_all(final_path.clone()).unwrap();
    }
    git2::Repository::clone(url, final_path.clone());
    final_path.clone()
}

fn main() {
    let cli = Command::parse();
    let loader;
    let lang;
    let projects_dir: PathBuf;
    let title: String;
    let id: String;
    let maven_group: String;
    let verbose: bool;
    if let Some(loader_) = cli.loader {
        loader = loader_;
    } else {
        loader = inquire::Select::new("Select loader", vec![Loader::Quilt])
            .prompt()
            .unwrap();
    }
    if let Some(lang_) = cli.lang {
        lang = lang_
    } else {
        lang = inquire::Select::new("Select lang", vec![Lang::Java, Lang::Kotlin])
            .prompt()
            .unwrap();
    }
    if let Some(projects_dir_) = cli.projects_dir {
        projects_dir = projects_dir_
    } else {
        projects_dir = inquire::Text::new(
            "Projects Directory (e.g. /home/user/Projects or C:\\Users\\User\\Projetcts)",
        )
        .prompt()
        .unwrap()
        .into();
    }
    if let Some(title_) = cli.title {
        title = title_
    } else {
        title = inquire::Text::new("Mod Title (e.g. ExampleMod)")
            .prompt()
            .unwrap();
    }
    if let Some(id_) = cli.id {
        id = id_;
    } else {
        id = inquire::Text::new("Mod ID (e.g. example_mod)")
            .prompt()
            .unwrap();
    }
    if let Some(maven_group_) = cli.maven_group {
        maven_group = maven_group_
    } else {
        maven_group =
            inquire::Text::new("Maven Group (usually a reversed domain name, e.g. com.example)")
                .prompt()
                .unwrap()
    }
    verbose = if let Some(verbose_) = cli.verbose {
        verbose_
    } else {
        false
    };

    if verbose {
        println!("Loader: {loader}");
        println!("Lang: {lang}");
    }
    let path = clone(
        loader.clone(),
        lang.clone(),
        projects_dir.clone(),
        id.clone(),
    );
    if verbose {
        println!("Cloned template to: {}", path.display());
        println!("Removing template's git metadata...");
    }
    std::fs::remove_dir_all(path.join(".git")).unwrap();

    // gradle.properties
    if verbose {
        println!("CHANGE: gradle.properties")
    };
    let mut gradle_properties = std::fs::read_to_string(path.join("gradle.properties")).unwrap();
    gradle_properties = gradle_properties.replace(
        "maven_group = com.example",
        &format!("maven_group = {}", maven_group),
    );
    gradle_properties = gradle_properties.replace(
        "archives_base_name = example_mod",
        &format!("archives_base_name = {}", maven_group),
    );
    std::fs::write(path.join("gradle.properties"), gradle_properties).unwrap();

    // change maven group dir
    if verbose {
        println!("MOVE: com.example.example_mod -> {}", maven_group)
    }
    let src_main = path.join("src").join("main");
    let maven_g_path = maven_group.replace(".", "/");
    let lang_new_maven_path = src_main
        .join(lang.clone().to_src_string())
        .join(maven_g_path.clone())
        .join(id.clone());
    std::fs::create_dir_all(lang_new_maven_path.clone()).unwrap();
    std::fs::rename(
        src_main
            .join(lang.clone().to_src_string())
            .join("com")
            .join("example")
            .join("example_mod"),
        lang_new_maven_path.clone(),
    )
    .unwrap();

    let main_class_file = lang_new_maven_path.clone().join(format!(
        "{}.{}",
        title.clone(),
        lang.clone().file_extension()
    ));
    std::fs::write(main_class_file.clone(), "").unwrap();

    if verbose {
        println!(
            "MOVE: ExampleMod.{0} -> {1}.{0}",
            lang.clone().file_extension(),
            title.clone()
        )
    }
    std::fs::rename(
        lang_new_maven_path
            .clone()
            .join(format!("ExampleMod.{}", lang.clone().file_extension())),
        main_class_file.clone(),
    )
    .unwrap();
    if verbose {
        println!("CHANGE: ExampleMod.{}", lang.clone().file_extension())
    }
    let mut class_file = std::fs::read_to_string(main_class_file.clone()).unwrap();
    class_file = class_file.replace("ExampleMod", &title.clone());
    class_file = class_file.replace(
        "package com.example.example_mod",
        &format!("package {}.{}", maven_group.clone(), id.clone()),
    );
    class_file = class_file.replace(
        "LoggerFactory.getLogger(\"Example Mod\")",
        &format!("LoggerFactory.getLogger(\"{}\")", title),
    );
    std::fs::write(main_class_file.clone(), class_file).unwrap();
    std::fs::create_dir_all(
        src_main
            .join("java")
            .join(maven_g_path.clone())
            .join(id.clone())
            .join("mixin"),
    )
    .unwrap();
    if lang == Lang::Kotlin {
        if verbose {
            println!(
                "MOVE: java/com.example.example_mod.mixin -> java/{}.{}.mixin",
                maven_group.clone(),
                id.clone()
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
                .join(maven_g_path.clone())
                .join(id.clone())
                .join("mixin"),
        )
        .unwrap();
    }
    if verbose {
        println!("CHANGE: mixin.TitleScreenMixin.java")
    }
    let mut mixin_file = std::fs::read_to_string(
        src_main
            .join("java")
            .join(maven_g_path.clone())
            .join(id.clone())
            .join("mixin")
            .join("TitleScreenMixin.java"),
    )
    .unwrap();
    mixin_file = mixin_file.replace(
        "package com.example.example_mod.mixin",
        &format!("package {}.{}.mixin", maven_group.clone(), id.clone()),
    );
    mixin_file = mixin_file.replace(
        "import com.example.example_mod.ExampleMod",
        &format!(
            "import {}.{}.{}",
            maven_group.clone(),
            id.clone(),
            title.clone()
        ),
    );
    if lang.clone() == Lang::Java {
        mixin_file = mixin_file.replace("ExampleMod.LOGGER", &format!("{}.LOGGER", title.clone()));
    } else {
        mixin_file = mixin_file.replace(
            "ExampleMod.INSTANCE",
            &format!("{}.INSTANCE", title.clone()),
        );
    }

    mixin_file = mixin_file.replace("exampleMod$", &format!("{}$", id.clone()));
    std::fs::write(
        src_main
            .join("java")
            .join(maven_g_path.clone())
            .join(id.clone())
            .join("mixin")
            .join("TitleScreenMixin.java"),
        mixin_file,
    )
    .unwrap();
    if verbose {
        println!("CHANGE: quilt.mod.json")
    }
    let mut mod_json =
        std::fs::read_to_string(src_main.join("resources").join("quilt.mod.json")).unwrap();
    mod_json = mod_json.replace(
        "\"name\": \"Mod Name\"",
        &format!("\"name\": \"{}\"", title.clone()),
    );
    mod_json = mod_json.replace(
        "\"group\": \"com.example\"",
        &format!("\"group\": \"{}\"", maven_group.clone()),
    );
    mod_json = mod_json.replace(
        "com.example.example_mod.ExampleMod",
        &format!("{}.{}.{}", maven_group.clone(), id.clone(), title.clone()),
    );
    mod_json = mod_json.replace("example_mod", &id.clone());
    std::fs::write(src_main.join("resources").join("quilt.mod.json"), mod_json).unwrap();
    if verbose {
        println!(
            "MOVE: example_mod.mixins.json -> {}.mixins.json",
            id.clone()
        )
    }
    std::fs::rename(
        src_main.join("resources").join("example_mod.mixins.json"),
        src_main
            .join("resources")
            .join(format!("{}.mixins.json", id.clone())),
    )
    .unwrap();
    let mut mixin_json = std::fs::read_to_string(
        src_main
            .join("resources")
            .join(format!("{}.mixins.json", id.clone())),
    )
    .unwrap();
    mixin_json = mixin_json.replace(
        "com.example.example_mod.mixin",
        &format!("{}.{}.mixin", maven_group, id),
    );
    std::fs::write(
        src_main
            .join("resources")
            .join(format!("{}.mixins.json", id)),
        mixin_json,
    )
    .unwrap();
    if verbose {
        println!("REMOVE: com.example")
    }
    //std::fs::rmdir();
    println!("done")
}
