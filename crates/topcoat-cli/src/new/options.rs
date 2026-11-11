use super::choice::{Css, DatabaseBackend, Interaction, Routing};

/// Validated choices for a new application.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectOptions {
    pub routing: Routing,
    pub database: DatabaseSetup,
    pub interaction: Interaction,
    pub css: Css,
    pub icons: IconSetup,
    pub font: FontSetup,
    pub ui: bool,
    pub example: bool,
}

impl ProjectOptions {
    /// The Iconify sets to stage at build time, without duplicates.
    pub fn icon_sets(&self) -> Vec<&str> {
        let mut sets = Vec::new();
        if let IconSetup::Iconify { set } = &self.icons {
            sets.push(set.as_str());
        }
        sets
    }
}

/// The selected database integration and its settings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DatabaseSetup {
    None,
    Toasty { backend: DatabaseBackend },
}

/// The selected icon source and its settings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IconSetup {
    None,
    Custom,
    Iconify { set: String },
}

/// The selected font source and its settings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FontSetup {
    None,
    Fontsource { family: String },
}
