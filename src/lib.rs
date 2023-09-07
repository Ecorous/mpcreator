use std::{
    collections::HashMap,
    fmt::Display,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use clap::ValueEnum;
use heck::{ToKebabCase, ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use serde::{Deserialize, Serialize};

pub mod change;
pub mod input;

#[derive(Debug, Clone, PartialEq, Eq, ValueEnum, Serialize, Deserialize, Hash)]
pub enum Loader {
    Quilt = 1,
}

#[derive(Debug, Clone, PartialEq, Eq, ValueEnum, Serialize, Deserialize, Hash)]
pub enum Language {
    Java,
    Kotlin,
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

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from(match self {
                Language::Java => "Java",
                Language::Kotlin => "Kotlin",
            })
        )
    }
}

impl Language {
    pub fn to_src_string(&self) -> String {
        if *self == Language::Java {
            return String::from("java");
        } else {
            return String::from("kotlin");
        }
    }

    pub fn file_extension(&self) -> String {
        if *self == Language::Java {
            return String::from("java");
        } else {
            return String::from("kt");
        }
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    pub title: String,
    pub id: String,
    pub main_class_name: String,
    pub maven_group: String,
    pub author: String,
    pub repo_url: Option<String>,
    pub issues_url: Option<String>,
    pub homepage_url: Option<String>,
    pub loader: Loader,
    pub lang: Language,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeSettings {
    pub projects_dir: PathBuf,
    pub verbose: bool,
    pub no_persistence: bool,
    pub replacements: HashMap<Loader, HashMap<Language, Replacements>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplacementFile {
    All {
        #[serde(with = "serde_regex")]
        matching: Option<regex::Regex>,
        #[serde(with = "serde_regex")]
        except_matching: Option<regex::Regex>,
    },
    Only {
        path: String,
    },
}

impl ReplacementFile {
    fn matches(&self, check: &Path) -> Result<bool> {
        match self {
            ReplacementFile::All {
                matching,
                except_matching,
            } => Ok(matching.as_ref().map_or(Ok(true), |it| -> Result<bool> {
                Ok(it.is_match(check.to_str().context("Unable convert Path to String")?))
            })? && !except_matching
                .as_ref()
                .map_or(Ok(false), |it| -> Result<bool> {
                    Ok(it.is_match(check.to_str().context("Unable convert Path to String")?))
                })?),
            ReplacementFile::Only { path } => Ok(check.ends_with(path)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Replacement {
    pub file: ReplacementFile,
    pub replace: String,
    pub with: ReplacementInsertion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplacementInsertion {
    Literal(String),
    Name(Case),
    Id(Case),
    Author(Case),
    RepoUrl(Case),
    IssuesUrl(Case),
    HomepageUrl(Case),
    Group(Case),
    MainClass(Case),
    Loader(Case),
    Lang(Case),
}

impl ReplacementInsertion {
    pub fn format(&self, project: &Project) -> String {
        match self {
            Self::Literal(string) => string.to_string(),
            Self::Name(case) => case.format(&project.title),
            Self::Id(case) => case.format(&project.id),
            Self::Author(case) => case.format(&project.author),
            Self::RepoUrl(case) => {
                case.format(&project.repo_url.as_ref().unwrap_or(&String::new()))
            }
            Self::IssuesUrl(case) => {
                case.format(&project.issues_url.as_ref().unwrap_or(&String::new()))
            }
            Self::HomepageUrl(case) => {
                case.format(&project.homepage_url.as_ref().unwrap_or(&String::new()))
            }
            Self::MainClass(case) => case.format(&project.main_class_name),
            Self::Loader(case) => case.format(&project.loader.to_string()),
            Self::Lang(case) => case.format(&project.lang.to_string()),
            Self::Group(case) => case.format(&project.maven_group),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Replacements(pub Vec<Replacement>);

impl Replacements {
    pub fn apply(&self, path: PathBuf, project: &Project, verbose: bool) -> Result<()> {
        for el in walkdir::WalkDir::new(path) {
            let el = el?;
            if el.path().is_file() {
                let file = std::fs::read_to_string(el.path());
                if let Err(e) = file {
                    eprintln!("Error while reading {:?} to string: {}", el.path(), e);
                    continue;
                }
                let mut file = file.unwrap();
                for replacement in &self.0 {
                    if replacement.file.matches(el.path())? {
                        file =
                            file.replace(&replacement.replace, &replacement.with.format(project));
                        if verbose {
                            println!(
                                "{}: {} -> {}",
                                el.path().display(),
                                replacement.replace,
                                replacement.with.format(project)
                            );
                        }
                    }
                }
                std::fs::write(el.path(), file)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Case {
    None,
    SnakeCase,
    UpperCamelCase,
    LowerCamelCase,
    KebapCase,
}

impl Case {
    fn format(&self, string: &str) -> String {
        match self {
            Self::None => string.to_string(),
            Self::SnakeCase => string.to_snake_case(),
            Self::UpperCamelCase => string.to_upper_camel_case(),
            Self::LowerCamelCase => string.to_lower_camel_case(),
            Self::KebapCase => string.to_kebab_case(),
        }
    }
}
