use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use heck::{ToSnakeCase, ToUpperCamelCase};
use inquire::{validator::{StringValidator, Validation}, CustomUserError};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{Language, Loader, Project, RuntimeSettings};

const MOD_ID_REGEX: &str = r"^[a-z][a-z0-9_]*$";
const JAVA_CLASS_NAME_REGEX: &str = r"^[a-zA-Z_$][a-zA-Z0-9_$]*$";
const GROUP_REGEX: &str = r"^([a-zA-Z_$][a-zA-Z0-9_$]*\.)*[a-zA-Z_$][a-zA-Z0-9_$]*$";

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
    #[arg(long, value_parser=parse_mod_id)]
    id: Option<String>,
    #[arg(long, value_parser=parse_class_name)]
    main_class_name: Option<String>,
    #[arg(long, value_parser=parse_group)]
    maven_group: Option<String>,
    #[arg(long, short)]
    verbose: bool,
    /// Will not try to use the config file and will not write to it.
    #[arg(long, short)]
    no_persistence: bool,
}

pub fn input() -> Result<(RuntimeSettings, Project)> {
    let cli = Command::parse();
    let no_persistence = cli.no_persistence;

    let loader = cli.loader.map_or_else(
        || inquire::Select::new("Select loader", vec![Loader::Quilt]).prompt(),
        |it| Ok(it),
    )?;
    let lang = cli.lang.map_or_else(
        || inquire::Select::new("Select language", vec![Language::Java, Language::Kotlin]).prompt(),
        |it| Ok(it),
    )?;
    let projects_dir: PathBuf = cli.projects_dir.map_or_else(
        || -> Result<PathBuf> { Ok(inquire::Text::new("Projects directory").prompt()?.into()) },
        |it| Ok(it),
    )?;
    let title = cli.title.map_or_else(
        || inquire::Text::new("Mod Title (e.g. Example Mod)").prompt(),
        |it| Ok(it),
    )?;
    let id = cli.id.map_or_else(
        || {
            inquire::Text::new("Mod ID")
                .with_default(&title.to_snake_case())
                .with_validator(RegexValidator(Regex::new(MOD_ID_REGEX).unwrap()))
                .prompt()
        },
        |it| Ok(it),
    )?;
    let main_class_name = cli.main_class_name.map_or_else(
        || {
            inquire::Text::new("Main Class Name")
                .with_default(&title.to_upper_camel_case())
                .with_validator(RegexValidator(Regex::new(JAVA_CLASS_NAME_REGEX).unwrap()))
                .prompt()
        },
        |it| Ok(it),
    )?;
    let maven_group = cli.maven_group.map_or_else(
        || {
            inquire::Text::new("Maven Group")
                .with_placeholder("e.g. com.example")
                .with_validator(RegexValidator(Regex::new(GROUP_REGEX).unwrap()))
                .prompt()
        },
        |it| Ok(it),
    )?;
    let verbose = cli.verbose;

    return Ok((
        RuntimeSettings {
            projects_dir: projects_dir.clone(),
            verbose,
            no_persistence,
        },
        Project {
            title: title.clone(),
            id,
            main_class_name,
            maven_group,
            loader,
            lang,
            path: projects_dir.join(title),
        },
    ));
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    projects_dir: PathBuf,
    no_persistence: bool,
    base_groups: Vec<String>,
    preffered_lang: Option<Language>,
    preffered_loader: Option<Loader>,
}

#[derive(Clone)]
struct RegexValidator (Regex);

impl StringValidator for RegexValidator {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        if self.0.is_match(input) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid(inquire::validator::ErrorMessage::Default))
        }
    }
}

fn parse_mod_id(id: &str) -> Result<String, String> {
    if Regex::new(MOD_ID_REGEX).unwrap().is_match(&id) {
        Ok(id.to_string())
    } else {
        Err("Invalid mod id".to_string())
    }
}

fn parse_class_name(name: &str) -> Result<String, String> {
    if Regex::new(JAVA_CLASS_NAME_REGEX).unwrap().is_match(&name) {
        Ok(name.to_string())
    } else {
        Err("Invalid class name".to_string())
    }
}


fn parse_group(name: &str) -> Result<String, String> {
    if Regex::new(GROUP_REGEX).unwrap().is_match(&name) {
        Ok(name.to_string())
    } else {
        Err("Invalid class name".to_string())
    }
}