use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use crate::{Language, Loader, Project, RuntimeSettings};

#[derive(Parser, Debug)]
struct Command {
    #[arg(long)]
    loader: Option<Loader>,
    #[arg(long)]
    lang: Option<Language>,
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
    #[arg(long, short)]
    no_persistence: bool,
}

pub fn input() -> Result<(RuntimeSettings, Project)> {
    let cli = Command::parse();
    let no_persistence = cli.no_persistence;

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
        loader = inquire::Select::new("Select loader", vec![Loader::Quilt]).prompt()?;
    }
    if let Some(lang_) = cli.lang {
        lang = lang_
    } else {
        lang = inquire::Select::new("Select language", vec![Language::Java, Language::Kotlin])
            .prompt()?;
    }
    if let Some(projects_dir_) = cli.projects_dir {
        projects_dir = projects_dir_
    } else {
        projects_dir = inquire::Text::new(
            "Projects Directory (e.g. /home/user/Projects or C:\\Users\\User\\Projetcts)",
        )
        .prompt()?
        .into();
    }
    if let Some(title_) = cli.title {
        title = title_
    } else {
        title = inquire::Text::new("Mod Title (e.g. ExampleMod)").prompt()?;
    }
    if let Some(id_) = cli.id {
        id = id_;
    } else {
        id = inquire::Text::new("Mod ID (e.g. example_mod)").prompt()?;
    }
    if let Some(maven_group_) = cli.maven_group {
        maven_group = maven_group_
    } else {
        maven_group =
            inquire::Text::new("Maven Group (usually a reversed domain name, e.g. com.example)")
                .prompt()?
    }
    verbose = if let Some(verbose_) = cli.verbose {
        verbose_
    } else {
        false
    };

    return Ok((
        RuntimeSettings {
            projects_dir: projects_dir.clone(),
            verbose,
            no_persistence,
        },
        Project {
            title: title.clone(),
            id,
            maven_group,
            loader,
            lang,
            path: projects_dir.join(title),
        },
    ));
}
