//! Package selection and third-party attribution metadata.

use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub(crate) struct Config {
    pub(crate) grammar_licenses: BTreeMap<String, String>,
    pub(crate) components: Vec<Component>,
    pub(crate) packages: Vec<Package>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Component {
    pub(crate) name: String,
    pub(crate) source: String,
    pub(crate) revision: Option<String>,
    pub(crate) version: Option<String>,
    pub(crate) license: String,
    pub(crate) material: String,
    pub(crate) files: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Package {
    pub(crate) name: String,
    pub(crate) directory: String,
    pub(crate) notices: Option<String>,
    pub(crate) components: Vec<String>,
}
