use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use heck::{ToSnakeCase, ToUpperCamelCase};
use inquire::{
    validator::{StringValidator, Validation},
    CustomUserError,
};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    Case, Language, Loader, Project, Replacement, ReplacementInsertion, Replacements,
    RuntimeSettings,
};

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
    author: Option<String>,
    #[arg(long)]
    repo_url: Option<String>,
    #[arg(long)]
    issues_url: Option<String>,
    #[arg(long)]
    homepage: Option<String>,
    #[arg(long)]
    config_path: Option<PathBuf>,
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
    let mut no_persistence = cli.no_persistence;
    let mut config: Config = if no_persistence {
        let mut config = Config::default();
        config.no_persistence = true;
        config
    } else {
        let config_path = cli
            .config_path
            .clone()
            .or_else(|| {
                dirs::config_dir()
                    .or_else(|| std::env::current_dir().ok())
                    .map(|it| it.join("mpcreator"))
            })
            .unwrap()
            .join("config.json");
        if config_path.exists() {
            let config_string = std::fs::read_to_string(config_path)?;
            let config: Config = serde_json::from_str(&config_string)?;
            if config.no_persistence {
                no_persistence = true;
                let mut config = Config::default();
                config.no_persistence = true;
                config
            } else {
                config
            }
        } else {
            Config::default()
        }
    };
    let no_persistence = no_persistence;

    let loader = cli.loader.or(config.preffered_loader.clone()).map_or_else(
        || inquire::Select::new("Select loader", vec![Loader::Quilt]).prompt(),
        |it| Ok(it),
    )?;
    let lang = cli.lang.or(config.preffered_lang.clone()).map_or_else(
        || inquire::Select::new("Select language", vec![Language::Java, Language::Kotlin]).prompt(),
        |it| Ok(it),
    )?;
    let projects_dir: PathBuf = match cli.projects_dir.or(config.projects_dir.clone()) {
        Some(it) => it,
        None => {
            let text: PathBuf = inquire::Text::new("Projects directory").prompt()?.into();
            config.projects_dir = Some(text.clone());
            text
        }
    };
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
    let author = cli.author.map_or_else(
        || {
            let mut prompt = inquire::Text::new("Author");
            if let Some(author) = &config.default_author {
                prompt = prompt.with_default(author);
            }
            prompt.prompt()
        },
        |it| Ok(it),
    )?;

    let mut project = Project {
        title: title.clone(),
        id,
        main_class_name,
        maven_group: String::new(),
        author,
        repo_url: None,
        homepage_url: None,
        issues_url: None,
        loader,
        lang,
        path: projects_dir.join(&title),
    };

    let maven_group = cli.maven_group.map_or_else(
        || {
            let text = inquire::Text::new("Maven Group")
                .with_validator(RegexValidator(Regex::new(GROUP_REGEX).unwrap()));

            match &config.group_format {
                None => text.with_placeholder("e.g. com.example").prompt(),
                Some(format) => text.with_default(&format.format(&project)).prompt(),
            }
        },
        |it| Ok(it),
    )?;
    if config.group_format.is_none() {
        config.group_format = Some(StringTemplate::try_detect(&maven_group, &project));
    }
    project.maven_group = maven_group.clone();

    let repo_url = cli.repo_url.map_or_else(
        || {
            inquire::Text::new("Repository URL")
                .with_default(
                    &config
                        .repo_url_format
                        .as_ref()
                        .map_or(String::new(), |it| it.format(&project)),
                )
                .prompt()
        },
        |it| Ok(it),
    )?;
    project.repo_url = if repo_url.is_empty() {
        None
    } else {
        if config.repo_url_format.is_none() {
            config.repo_url_format = Some(StringTemplate::try_detect(&repo_url, &project));
        }
        Some(repo_url)
    };

    let issues_url = cli.issues_url.map_or_else(
        || {
            inquire::Text::new("Issues URL")
                .with_default(
                    &config
                        .issues_url_format
                        .as_ref()
                        .map_or(String::new(), |it| it.format(&project)),
                )
                .prompt()
        },
        |it| Ok(it),
    )?;
    project.issues_url = if issues_url.is_empty() {
        None
    } else {
        if config.issues_url_format.is_none() {
            config.issues_url_format = Some(StringTemplate::try_detect(&issues_url, &project));
        }
        Some(issues_url)
    };

    let homepage_url = cli.homepage.map_or_else(
        || {
            inquire::Text::new("Home Page URL")
                .with_default(
                    &config
                        .homepage_url_format
                        .as_ref()
                        .map_or(String::new(), |it| it.format(&project)),
                )
                .prompt()
        },
        |it| Ok(it),
    )?;
    project.homepage_url = if homepage_url.is_empty() {
        None
    } else {
        if config.homepage_url_format.is_none() {
            config.homepage_url_format = Some(StringTemplate::try_detect(&homepage_url, &project));
        }
        Some(homepage_url)
    };

    if !config.no_persistence {
        let config_path = cli
            .config_path
            .or_else(|| {
                dirs::config_dir()
                    .or_else(|| std::env::current_dir().ok())
                    .map(|it| it.join("mpcreator"))
            })
            .unwrap()
            .join("config.json");
        std::fs::create_dir_all(config_path.parent().unwrap())?;
        std::fs::write(config_path, serde_json::to_string_pretty(&config)?)?;
    }

    let verbose = cli.verbose;

    return Ok((
        RuntimeSettings {
            projects_dir: projects_dir.clone(),
            verbose,
            no_persistence,
            replacements: config.replacements,
        },
        project,
    ));
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    projects_dir: Option<PathBuf>,
    no_persistence: bool,
    default_author: Option<String>,
    group_format: Option<StringTemplate>,
    homepage_url_format: Option<StringTemplate>,
    repo_url_format: Option<StringTemplate>,
    issues_url_format: Option<StringTemplate>,
    #[serde(default = "default_replacements")]
    replacements: HashMap<Loader, HashMap<Language, Replacements>>,
    preffered_lang: Option<Language>,
    preffered_loader: Option<Loader>,
}

fn default_replacements() -> HashMap<Loader, HashMap<Language, Replacements>> {
    let mut map = HashMap::new();
    let mut loaders_map = HashMap::new();
    loaders_map.insert(
        Language::Java,
        Replacements(vec![Replacement {
            file: crate::ReplacementFile::All {
                matching: None,
                except_matching: Some(Regex::new(r"(\.jar)|(gradlew)|(gradlew.bat)|(README.md)|(.editorconfig)|(.gitignore)|(gradle-wrapper.properties)|(\.png)|(LICENSE-TEMPLATE.md)|(.gitattributes)$").unwrap()),
            },
            replace: "com.example".to_string(),
            with: ReplacementInsertion::Group(Case::None),
        },
        Replacement {
            file: crate::ReplacementFile::All {
                matching: None,
                except_matching: Some(Regex::new(r"(\.jar)|(gradlew)|(gradlew.bat)|(README.md)|(.editorconfig)|(.gitignore)|(gradle-wrapper.properties)|(\.png)|(LICENSE-TEMPLATE.md)|(.gitattributes)$").unwrap()),
            },
            replace: "Example Mod".to_string(),
            with: ReplacementInsertion::Name(Case::None),
        },
        Replacement {
            file: crate::ReplacementFile::All {
                matching: None,
                except_matching: Some(Regex::new(r"(\.jar)|(gradlew)|(gradlew.bat)|(README.md)|(.editorconfig)|(.gitignore)|(gradle-wrapper.properties)|(\.png)|(LICENSE-TEMPLATE.md)|(.gitattributes)$").unwrap()),
            },
            replace: "example_mod".to_string(),
            with: ReplacementInsertion::Id(Case::None),
        },
        Replacement {
            file: crate::ReplacementFile::All {
                matching: None,
                except_matching: Some(Regex::new(r"(\.jar)|(gradlew)|(gradlew.bat)|(README.md)|(.editorconfig)|(.gitignore)|(gradle-wrapper.properties)|(\.png)|(LICENSE-TEMPLATE.md)|(.gitattributes)$").unwrap()),
            },
            replace: "ExampleMod".to_string(),
            with: ReplacementInsertion::MainClass(Case::None),
        },
        Replacement {
            file: crate::ReplacementFile::All {
                matching: None,
                except_matching: Some(Regex::new(r"(\.jar)|(gradlew)|(gradlew.bat)|(README.md)|(.editorconfig)|(.gitignore)|(gradle-wrapper.properties)|(\.png)|(LICENSE-TEMPLATE.md)|(.gitattributes)$").unwrap()),
            },
            replace: "exampleMod".to_string(),
            with: ReplacementInsertion::MainClass(Case::LowerCamelCase),
        },
        Replacement {
            file: crate::ReplacementFile::ONLY { path: "quilt.mod.json".to_string() },
            replace: "Your name here".to_string(),
            with: ReplacementInsertion::Author(Case::None),
        },
        Replacement {
            file: crate::ReplacementFile::ONLY { path: "quilt.mod.json".to_string() },
            replace: "https://example.com/".to_string(),
            with: ReplacementInsertion::HomepageUrl(Case::None),
        },
        Replacement {
            file: crate::ReplacementFile::ONLY { path: "quilt.mod.json".to_string() },
            replace: "https://github.com/QuiltMC/quilt-template-mod/issues".to_string(),
            with: ReplacementInsertion::IssuesUrl(Case::None),
        },
        Replacement {
            file: crate::ReplacementFile::ONLY { path: "quilt.mod.json".to_string() },
            replace: "https://github.com/QuiltMC/quilt-template-mod".to_string(),
            with: ReplacementInsertion::RepoUrl(Case::None),
        },
        ]),
    );
    loaders_map.insert(
        Language::Kotlin,
        Replacements(vec![Replacement {
            file: crate::ReplacementFile::All {
                matching: None,
                except_matching: Some(Regex::new(r"(\.jar)|(gradlew)|(gradlew.bat)|(README.md)|(.editorconfig)|(.gitignore)|(gradle-wrapper.properties)|(\.png)|(LICENSE-TEMPLATE.md)|(.gitattributes)$").unwrap()),
            },
            replace: "com.example".to_string(),
            with: ReplacementInsertion::Group(Case::None),
        },
        Replacement {
            file: crate::ReplacementFile::All {
                matching: None,
                except_matching: Some(Regex::new(r"(\.jar)|(gradlew)|(gradlew.bat)|(README.md)|(.editorconfig)|(.gitignore)|(gradle-wrapper.properties)|(\.png)|(LICENSE-TEMPLATE.md)|(.gitattributes)$").unwrap()),
            },
            replace: "Example Mod".to_string(),
            with: ReplacementInsertion::Name(Case::None),
        },
        Replacement {
            file: crate::ReplacementFile::All {
                matching: None,
                except_matching: Some(Regex::new(r"(\.jar)|(gradlew)|(gradlew.bat)|(README.md)|(.editorconfig)|(.gitignore)|(gradle-wrapper.properties)|(\.png)|(LICENSE-TEMPLATE.md)|(.gitattributes)$").unwrap()),
            },
            replace: "example_mod".to_string(),
            with: ReplacementInsertion::Id(Case::None),
        },
        Replacement {
            file: crate::ReplacementFile::All {
                matching: None,
                except_matching: Some(Regex::new(r"(\.jar)|(gradlew)|(gradlew.bat)|(README.md)|(.editorconfig)|(.gitignore)|(gradle-wrapper.properties)|(\.png)|(LICENSE-TEMPLATE.md)|(.gitattributes)$").unwrap()),
            },
            replace: "ExampleMod".to_string(),
            with: ReplacementInsertion::MainClass(Case::None),
        },
        Replacement {
            file: crate::ReplacementFile::All {
                matching: None,
                except_matching: Some(Regex::new(r"(\.jar)|(gradlew)|(gradlew.bat)|(README.md)|(.editorconfig)|(.gitignore)|(gradle-wrapper.properties)|(\.png)|(LICENSE-TEMPLATE.md)|(.gitattributes)$").unwrap()),
            },
            replace: "exampleMod".to_string(),
            with: ReplacementInsertion::MainClass(Case::LowerCamelCase),
        },
        Replacement {
            file: crate::ReplacementFile::ONLY { path: "quilt.mod.json".to_string() },
            replace: "Mod Name".to_string(),
            with: ReplacementInsertion::Name(Case::None),
        },
        Replacement {
            file: crate::ReplacementFile::ONLY { path: "quilt.mod.json".to_string() },
            replace: "Your name here".to_string(),
            with: ReplacementInsertion::Author(Case::None),
        },
        Replacement {
            file: crate::ReplacementFile::ONLY { path: "quilt.mod.json".to_string() },
            replace: "https://example.com/".to_string(),
            with: ReplacementInsertion::HomepageUrl(Case::None),
        },
        Replacement {
            file: crate::ReplacementFile::ONLY { path: "quilt.mod.json".to_string() },
            replace: "https://github.com/QuiltMC/quilt-kotlin-template-mod/issues".to_string(),
            with: ReplacementInsertion::IssuesUrl(Case::None),
        },
        Replacement {
            file: crate::ReplacementFile::ONLY { path: "quilt.mod.json".to_string() },
            replace: "https://github.com/QuiltMC/quilt-kotlin-template-mod".to_string(),
            with: ReplacementInsertion::RepoUrl(Case::None),
        },
        Replacement {
            file: crate::ReplacementFile::All {
                matching: None,
                except_matching: Some(Regex::new(r"(\.jar)|(gradlew)|(gradlew.bat)|(README.md)|(.editorconfig)|(.gitignore)|(gradle-wrapper.properties)|(\.png)|(LICENSE-TEMPLATE.md)|(.gitattributes)$").unwrap()),
            },
            replace: "quilt-kotlin-template-mod".to_string(),
            with: ReplacementInsertion::Id(Case::None),
        },
        ]),
    );
    map.insert(Loader::Quilt, loaders_map);
    map
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringTemplate(Vec<Part>);

impl StringTemplate {
    const CASES: [Case; 5] = [
        Case::None,
        Case::SnakeCase,
        Case::UpperCamelCase,
        Case::LowerCamelCase,
        Case::KebapCase,
    ];
    pub fn format(&self, project: &Project) -> String {
        self.0
            .iter()
            .map(|it| it.format(project))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn try_detect(url: &str, project: &Project) -> StringTemplate {
        let mut res = vec![Part::Literal(url.to_string())];
        for case in Self::CASES {
            res = res
                .into_iter()
                .map(|part| {
                    if let Part::Literal(string) = &part {
                        string
                            .split(&case.format(&project.title))
                            .map(|it| Part::Literal(it.to_string()))
                            .fold(vec![], |mut vec, it| {
                                if !vec.is_empty() {
                                    vec.push(Part::Name(case));
                                }
                                vec.push(it);
                                vec
                            })
                    } else {
                        vec![part]
                    }
                })
                .flatten()
                .collect();
        }
        for case in Self::CASES {
            res = res
                .into_iter()
                .map(|part| {
                    if let Part::Literal(string) = &part {
                        string
                            .split(&case.format(&project.author))
                            .map(|it| Part::Literal(it.to_string()))
                            .fold(vec![], |mut vec, it| {
                                if !vec.is_empty() {
                                    vec.push(Part::Author(case));
                                }
                                vec.push(it);
                                vec
                            })
                    } else {
                        vec![part]
                    }
                })
                .flatten()
                .collect();
        }
        StringTemplate(res)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Part {
    Name(Case),
    Author(Case),
    Literal(String),
}

impl Part {
    pub fn format(&self, project: &Project) -> String {
        match self {
            Self::Name(case) => case.format(&project.title),
            Self::Author(case) => case.format(&project.author),
            Self::Literal(string) => string.clone(),
        }
    }
}

#[derive(Clone)]
struct RegexValidator(Regex);

impl StringValidator for RegexValidator {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        if self.0.is_match(input) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid(
                inquire::validator::ErrorMessage::Default,
            ))
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
