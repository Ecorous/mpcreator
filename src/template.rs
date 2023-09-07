use std::collections::HashMap;

use regex::Regex;

use crate::{Loader, Language, Replacements, Replacement, Case, ReplacementInsertion, input::TemplateData};

const QUILT_JAVA_GIT_TEMPLATE: &str = "https://github.com/QuiltMC/quilt-template-mod.git";
const QUILT_KOTLIN_GIT_TEMPLATE: &str = "https://github.com/QuiltMC/quilt-kotlin-template-mod.git";


pub(crate) fn build_default_template_data() -> HashMap<Loader, HashMap<Language, TemplateData>> {
    let mut map = HashMap::new();
    let mut loaders_map = HashMap::new();
    loaders_map.insert(
        Language::Java,
        build_java_template_data()
    );
    loaders_map.insert(
        Language::Kotlin,
        build_kotlin_template_data()
    );
    map.insert(Loader::Quilt, loaders_map);
    map
}

fn build_java_template_data() -> TemplateData {
    let replacements = 
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
        file: crate::ReplacementFile::Only { path: "quilt.mod.json".to_string() },
        replace: "Your name here".to_string(),
        with: ReplacementInsertion::Author(Case::None),
    },
    Replacement {
        file: crate::ReplacementFile::Only { path: "quilt.mod.json".to_string() },
        replace: "https://example.com/".to_string(),
        with: ReplacementInsertion::HomepageUrl(Case::None),
    },
    Replacement {
        file: crate::ReplacementFile::Only { path: "quilt.mod.json".to_string() },
        replace: "https://github.com/QuiltMC/quilt-template-mod/issues".to_string(),
        with: ReplacementInsertion::IssuesUrl(Case::None),
    },
    Replacement {
        file: crate::ReplacementFile::Only { path: "quilt.mod.json".to_string() },
        replace: "https://github.com/QuiltMC/quilt-template-mod".to_string(),
        with: ReplacementInsertion::RepoUrl(Case::None),
    },
    ]);
    TemplateData {
        repository_url: QUILT_JAVA_GIT_TEMPLATE.to_string(),
        replacements,
    }
}

fn build_kotlin_template_data() -> TemplateData {
    let replacements = Replacements(vec![Replacement {
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
        file: crate::ReplacementFile::Only { path: "quilt.mod.json".to_string() },
        replace: "Mod Name".to_string(),
        with: ReplacementInsertion::Name(Case::None),
    },
    Replacement {
        file: crate::ReplacementFile::Only { path: "quilt.mod.json".to_string() },
        replace: "Your name here".to_string(),
        with: ReplacementInsertion::Author(Case::None),
    },
    Replacement {
        file: crate::ReplacementFile::Only { path: "quilt.mod.json".to_string() },
        replace: "https://example.com/".to_string(),
        with: ReplacementInsertion::HomepageUrl(Case::None),
    },
    Replacement {
        file: crate::ReplacementFile::Only { path: "quilt.mod.json".to_string() },
        replace: "https://github.com/QuiltMC/quilt-kotlin-template-mod/issues".to_string(),
        with: ReplacementInsertion::IssuesUrl(Case::None),
    },
    Replacement {
        file: crate::ReplacementFile::Only { path: "quilt.mod.json".to_string() },
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
    ]);
    TemplateData {
        repository_url: QUILT_KOTLIN_GIT_TEMPLATE.to_string(),
        replacements,
    }
}