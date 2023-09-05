use std::{fmt::Display, path::PathBuf};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

pub mod input;
pub mod change;

#[derive(Debug, Clone, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
pub enum Loader {
    Quilt = 1,
}

#[derive(Debug, Clone, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
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
    pub maven_group: String,
    pub loader: Loader,
    pub lang: Language,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeSettings {
    pub projects_dir: PathBuf,
    pub verbose: bool,
    pub no_persistence: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    projects_dir: PathBuf,
    no_persistence: bool,
    base_groups: Vec<String>,
    preffered_lang: Option<Language>,
    preffered_loader: Option<Loader>,
}
