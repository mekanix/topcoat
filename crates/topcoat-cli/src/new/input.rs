use clap::Args;
use topcoat_font::fontsource::Family;

use super::{
    choice::{Css, Database, DatabaseBackend, Font, Icons, Interaction, Routing},
    options::{DatabaseSetup, FontSetup, IconSetup, ProjectOptions},
};

/// The Iconify set used when none is named.
pub const DEFAULT_ICON_SET: &str = "lucide";
/// The Fontsource family used when none is named.
const DEFAULT_FONT_FAMILY: &str = "inter";
/// The Fontsource family used with Topcoat UI when none is named, matching the UI theme.
const UI_FONT_FAMILY: &str = "geist";

/// Command-line flags for the application choices.
#[derive(Args)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "each bool is a command-line switch"
)]
pub struct ChoiceArgs {
    /// Use the recommended choice for every option not given as a flag
    #[arg(long, conflicts_with = "minimal")]
    recommended: bool,
    /// Use the smallest setup for every option not given as a flag
    #[arg(long)]
    minimal: bool,
    /// How request paths are assigned to handlers
    #[arg(long, value_name = "STYLE")]
    routing: Option<Routing>,
    /// Database integration
    #[arg(long, value_name = "INTEGRATION")]
    database: Option<Database>,
    /// Database backend [default: sqlite]
    #[arg(long, value_name = "BACKEND")]
    database_backend: Option<DatabaseBackend>,
    /// Browser interaction approach
    #[arg(long, value_name = "APPROACH")]
    interaction: Option<Interaction>,
    /// CSS framework for pages and components [default: tailwind, or plain CSS with --minimal]
    #[arg(long, value_name = "FRAMEWORK", conflicts_with = "no_css")]
    css: Option<Css>,
    /// Use a plain CSS stylesheet without a CSS framework
    #[arg(long)]
    no_css: bool,
    /// Icon source
    #[arg(long, value_name = "SOURCE")]
    icons: Option<Icons>,
    /// Iconify set to stage [default: lucide]
    #[arg(long, value_name = "SET")]
    icon_set: Option<String>,
    /// Font source
    #[arg(long, value_name = "SOURCE")]
    font: Option<Font>,
    /// Fontsource family ID [default: inter, or geist with Tailwind UI]
    #[arg(long, value_name = "FAMILY")]
    font_family: Option<String>,
    /// Use components from the selected CSS framework
    #[arg(long, conflicts_with = "no_ui")]
    ui: bool,
    /// Do not install Topcoat UI components
    #[arg(long)]
    no_ui: bool,
    /// Include the todo example
    #[arg(long, conflicts_with = "no_example")]
    example: bool,
    /// Do not include the todo example
    #[arg(long)]
    no_example: bool,
}

impl ChoiceArgs {
    /// The preset selected by `--recommended` or `--minimal`, if any.
    pub fn preset(&self) -> Option<Preset> {
        if self.recommended {
            Some(Preset::Recommended)
        } else if self.minimal {
            Some(Preset::Minimal)
        } else {
            None
        }
    }
}

/// A complete set of top-level choices selected by a single flag.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preset {
    Recommended,
    Minimal,
}

impl Preset {
    /// The preset's value for every top-level choice.
    pub fn choices(self) -> PresetChoices {
        match self {
            Self::Recommended => PresetChoices {
                routing: Routing::Module,
                database: Database::Toasty,
                interaction: Interaction::Topcoat,
                icons: Icons::Iconify,
                font: Font::Fontsource,
                ui: true,
                example: true,
            },
            Self::Minimal => PresetChoices {
                routing: Routing::Module,
                database: Database::None,
                interaction: Interaction::None,
                icons: Icons::None,
                font: Font::None,
                ui: false,
                example: false,
            },
        }
    }
}

/// A value for every top-level choice.
pub struct PresetChoices {
    pub routing: Routing,
    pub database: Database,
    pub interaction: Interaction,
    pub icons: Icons,
    pub font: Font,
    pub ui: bool,
    pub example: bool,
}

/// Where a choice's value came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Origin {
    /// An explicit command-line flag.
    Flag,
    /// A preset's value for a choice not given as a flag.
    Preset(Preset),
    /// An answer to a wizard question.
    Prompt,
}

/// A choice's value and where it came from.
#[derive(Clone, Copy, Debug)]
pub struct Sourced<T> {
    pub value: T,
    pub origin: Origin,
}

impl<T> Sourced<T> {
    pub fn new(value: T, origin: Origin) -> Self {
        Self { value, origin }
    }
}

/// Application choices collected so far. A `None` field is still open.
pub struct Input {
    pub routing: Option<Sourced<Routing>>,
    pub database: Option<Sourced<Database>>,
    pub database_backend: Option<DatabaseBackend>,
    pub interaction: Option<Sourced<Interaction>>,
    pub css: Sourced<Css>,
    pub icons: Option<Sourced<Icons>>,
    pub icon_set: Option<String>,
    pub font: Option<Sourced<Font>>,
    pub font_family: Option<String>,
    pub ui: Option<Sourced<bool>>,
    pub example: Option<Sourced<bool>>,
}

impl Input {
    /// Collects the choices made by `args`, then fills open choices from the preset.
    pub fn new(args: ChoiceArgs) -> Self {
        fn flag<T>(value: T) -> Sourced<T> {
            Sourced::new(value, Origin::Flag)
        }

        let preset = args.preset();
        let css = if args.no_css {
            Css::None
        } else {
            args.css
                .unwrap_or(if preset == Some(Preset::Minimal) && !args.ui {
                    Css::None
                } else {
                    Css::Tailwind
                })
        };
        let mut input = Self {
            routing: args.routing.map(flag),
            database: args.database.map(flag),
            database_backend: args.database_backend,
            interaction: args.interaction.map(flag),
            css: flag(css),
            icons: args.icons.map(flag),
            icon_set: args.icon_set,
            font: args.font.map(flag),
            font_family: args.font_family,
            ui: switch(args.ui, args.no_ui).map(flag),
            example: switch(args.example, args.no_example).map(flag),
        };
        if let Some(preset) = preset {
            input.fill(preset);
        }
        input
    }

    /// Fills open top-level choices from `preset`.
    fn fill(&mut self, preset: Preset) {
        let origin = Origin::Preset(preset);
        let choices = preset.choices();
        self.routing
            .get_or_insert(Sourced::new(choices.routing, origin));
        self.database
            .get_or_insert(Sourced::new(choices.database, origin));
        self.interaction
            .get_or_insert(Sourced::new(choices.interaction, origin));
        self.icons
            .get_or_insert(Sourced::new(choices.icons, origin));
        self.font.get_or_insert(Sourced::new(choices.font, origin));
        self.ui.get_or_insert(Sourced::new(choices.ui, origin));
        self.example
            .get_or_insert(Sourced::new(choices.example, origin));
    }

    /// Records the answer to a wizard question.
    pub fn answer(&mut self, answer: Answer) {
        fn prompt<T>(value: T) -> Sourced<T> {
            Sourced::new(value, Origin::Prompt)
        }

        match answer {
            Answer::Routing(value) => self.routing = Some(prompt(value)),
            Answer::Database(value) => self.database = Some(prompt(value)),
            Answer::Interaction(value) => self.interaction = Some(prompt(value)),
            Answer::Icons(value) => self.icons = Some(prompt(value)),
            Answer::Font(value) => self.font = Some(prompt(value)),
            Answer::Ui(value) => self.ui = Some(prompt(value)),
            Answer::Example(value) => self.example = Some(prompt(value)),
        }
    }

    /// The Fontsource family used when none is named: the Tailwind UI theme's font if
    /// Topcoat UI is selected.
    pub fn default_font_family(&self) -> &'static str {
        if self.css.value == Css::Tailwind && self.ui.is_some_and(|ui| ui.value) {
            UI_FONT_FAMILY
        } else {
            DEFAULT_FONT_FAMILY
        }
    }

    /// The open top-level choices, in the order the wizard asks them.
    pub fn questions(&self) -> Vec<Question> {
        [
            (self.routing.is_none(), Question::Routing),
            (self.database.is_none(), Question::Database),
            (self.interaction.is_none(), Question::Interaction),
            (self.ui.is_none(), Question::Ui),
            (self.icons.is_none(), Question::Icons),
            (self.font.is_none(), Question::Font),
            (self.example.is_none(), Question::Example),
        ]
        .into_iter()
        .filter_map(|(open, question)| open.then_some(question))
        .collect()
    }

    /// Validates the complete set of choices and fills defaults for unset settings.
    ///
    /// Returns an error naming the missing flags if a top-level choice is still open,
    /// or explaining how to resolve choices that cannot be combined.
    pub fn resolve(&self) -> Result<Resolution, String> {
        let (
            Some(routing),
            Some(database),
            Some(interaction),
            Some(icons),
            Some(font),
            Some(ui),
            Some(example),
        ) = (
            self.routing,
            self.database,
            self.interaction,
            self.icons,
            self.font,
            self.ui,
            self.example,
        )
        else {
            let flags: Vec<&str> = self.questions().into_iter().map(Question::flag).collect();
            return Err(format!(
                "missing choices: pass {}, or use --recommended or --minimal",
                flags.join(", ")
            ));
        };

        if ui.value && self.css.value == Css::None {
            return Err(
                "Topcoat UI requires a CSS framework; also pass --no-ui or --css tailwind or --css chota"
                    .to_string(),
            );
        }

        let database_setup = match (database.value, self.database_backend) {
            (Database::Toasty, backend) => DatabaseSetup::Toasty {
                backend: backend.unwrap_or_default(),
            },
            (Database::None, Some(_)) => {
                return Err("--database-backend requires --database toasty".to_string());
            }
            (Database::None, None) => DatabaseSetup::None,
        };

        let icon_setup = match (icons.value, self.icon_set.as_deref()) {
            (Icons::Iconify, set) => IconSetup::Iconify {
                set: icon_set_id(set.unwrap_or(DEFAULT_ICON_SET))?,
            },
            (Icons::Custom | Icons::None, Some(_)) => {
                return Err("--icon-set requires --icons iconify".to_string());
            }
            (Icons::Custom, None) => IconSetup::Custom,
            (Icons::None, None) => IconSetup::None,
        };

        let font_setup = match (font.value, self.font_family.as_deref()) {
            (Font::Fontsource, family) => {
                let family = family.unwrap_or(self.default_font_family());
                FontSetup::Fontsource {
                    family: font_family_id(family)?,
                }
            }
            (Font::None, Some(_)) => {
                return Err("--font-family requires --font fontsource".to_string());
            }
            (Font::None, None) => FontSetup::None,
        };

        let options = ProjectOptions {
            routing: routing.value,
            database: database_setup,
            interaction: interaction.value,
            css: self.css.value,
            icons: icon_setup,
            font: font_setup,
            ui: ui.value,
            example: example.value,
        };

        let mut notes = Vec::new();
        if options.ui && options.css == Css::Tailwind && options.font == FontSetup::None {
            notes.push(
                "The Tailwind UI theme expects the Geist font and falls back to the system font"
                    .to_string(),
            );
        }

        Ok(Resolution { options, notes })
    }
}

/// An open top-level choice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Question {
    Routing,
    Database,
    Interaction,
    Ui,
    Icons,
    Font,
    Example,
}

impl Question {
    /// The flags that answer the question.
    pub fn flag(self) -> &'static str {
        match self {
            Self::Routing => "--routing",
            Self::Database => "--database",
            Self::Interaction => "--interaction",
            Self::Icons => "--icons",
            Self::Font => "--font",
            Self::Ui => "--ui or --no-ui",
            Self::Example => "--example or --no-example",
        }
    }
}

/// An answer to a wizard question.
#[derive(Clone, Copy, Debug)]
pub enum Answer {
    Routing(Routing),
    Database(Database),
    Interaction(Interaction),
    Icons(Icons),
    Font(Font),
    Ui(bool),
    Example(bool),
}

/// Validated options and notes about choices made on the user's behalf.
pub struct Resolution {
    pub options: ProjectOptions,
    pub notes: Vec<String>,
}

/// Combines a pair of `--x` and `--no-x` flags. Clap rejects passing both.
fn switch(enabled: bool, disabled: bool) -> Option<bool> {
    match (enabled, disabled) {
        (true, _) => Some(true),
        (_, true) => Some(false),
        _ => None,
    }
}

/// Checks that `value` has the form of an Iconify set ID such as `lucide`: lowercase
/// ASCII letters, digits, and hyphens. Whether the set exists is only known once it is
/// downloaded.
pub fn icon_set_id(value: &str) -> Result<String, String> {
    let valid = !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-');
    if valid {
        Ok(value.to_string())
    } else {
        Err(format!(
            "invalid icon set `{value}`: use lowercase letters, digits, and hyphens"
        ))
    }
}

/// Checks that `value` is the ID of a family in the Fontsource catalog, such as `inter`.
pub fn font_family_id(value: &str) -> Result<String, String> {
    if Family::by_id(value).is_some() {
        Ok(value.to_string())
    } else {
        Err(format!(
            "unknown Fontsource family `{value}`; use a family ID listed at \
             https://fontsource.org, such as `{DEFAULT_FONT_FAMILY}`"
        ))
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    #[derive(Parser)]
    struct Cli {
        #[command(flatten)]
        choices: ChoiceArgs,
    }

    fn input<S: AsRef<str>>(args: &[S]) -> Input {
        let args = std::iter::once("topcoat-new").chain(args.iter().map(AsRef::as_ref));
        Input::new(Cli::try_parse_from(args).unwrap().choices)
    }

    fn resolve<S: AsRef<str>>(args: &[S]) -> Result<ProjectOptions, String> {
        input(args).resolve().map(|resolution| resolution.options)
    }

    #[test]
    fn presets_answer_every_question() {
        for preset in ["--recommended", "--minimal"] {
            assert_eq!(input(&[preset]).questions(), []);
            assert!(resolve(&[preset]).is_ok());
        }
    }

    #[test]
    fn flags_override_preset_choices() {
        let options = resolve(&[
            "--recommended",
            "--database",
            "none",
            "--interaction",
            "htmx",
        ])
        .unwrap();
        assert_eq!(options.database, DatabaseSetup::None);
        assert_eq!(options.interaction, Interaction::Htmx);
    }

    #[test]
    fn css_defaults_depend_on_the_preset() {
        assert_eq!(resolve(&["--recommended"]).unwrap().css, Css::Tailwind);
        assert_eq!(resolve(&["--minimal"]).unwrap().css, Css::None);
    }

    #[test]
    fn css_and_ui_choices_are_independent() {
        let options = resolve(&["--recommended", "--css", "chota"]).unwrap();
        assert_eq!(options.css, Css::Chota);
        assert!(options.ui);

        let options = resolve(&["--minimal", "--ui"]).unwrap();
        assert_eq!(options.css, Css::Tailwind);
        assert!(options.ui);
        assert_eq!(options.icons, IconSetup::None);
        assert_eq!(options.font, FontSetup::None);
    }

    #[test]
    fn no_css_selects_plain_css() {
        let options = resolve(&["--recommended", "--no-css", "--no-ui"]).unwrap();
        assert_eq!(options.css, Css::None);
        assert!(resolve(&["--recommended", "--no-css"]).is_err());
        assert!(resolve(&["--minimal", "--ui", "--no-css"]).is_err());
    }

    #[test]
    fn ui_font_default_follows_the_css_framework() {
        let tailwind = resolve(&["--minimal", "--ui", "--font", "fontsource"]).unwrap();
        assert_eq!(
            tailwind.font,
            FontSetup::Fontsource {
                family: UI_FONT_FAMILY.to_string()
            }
        );

        let chota = resolve(&[
            "--minimal",
            "--ui",
            "--css",
            "chota",
            "--font",
            "fontsource",
        ])
        .unwrap();
        assert_eq!(
            chota.font,
            FontSetup::Fontsource {
                family: DEFAULT_FONT_FAMILY.to_string()
            }
        );
    }

    #[test]
    fn answers_resolve_like_the_equivalent_flags() {
        let mut answered = input::<&str>(&[]);
        for answer in [
            Answer::Routing(Routing::Discover),
            Answer::Database(Database::Toasty),
            Answer::Interaction(Interaction::Htmx),
            Answer::Ui(false),
            Answer::Icons(Icons::Custom),
            Answer::Font(Font::Fontsource),
            Answer::Example(false),
        ] {
            answered.answer(answer);
        }
        assert_eq!(answered.questions(), []);

        let flags = resolve(&[
            "--routing",
            "discover",
            "--database",
            "toasty",
            "--interaction",
            "htmx",
            "--icons",
            "custom",
            "--font",
            "fontsource",
            "--no-ui",
            "--no-example",
        ])
        .unwrap();
        assert_eq!(answered.resolve().unwrap().options, flags);
    }

    #[test]
    fn missing_choices_name_their_flags() {
        let error = resolve(&["--database", "none", "--no-ui"]).unwrap_err();
        assert!(error.contains("--interaction"));
        assert!(!error.contains("--database"));
    }

    #[test]
    fn settings_require_their_integration() {
        for setting in [
            ["--database-backend", "sqlite"],
            ["--icon-set", "tabler"],
            ["--font-family", "inter"],
        ] {
            let mut args = vec!["--minimal"];
            args.extend(setting);
            assert!(resolve(&args).is_err(), "{args:?}");
        }
    }

    #[test]
    fn settings_reject_ids_that_are_not_catalog_ids() {
        assert!(resolve(&["--recommended", "--icon-set", "../lucide"]).is_err());
        assert!(resolve(&["--recommended", "--font-family", "\"inter\""]).is_err());
    }

    #[test]
    fn font_families_must_exist_in_the_fontsource_catalog() {
        assert!(resolve(&["--recommended", "--font-family", "roboto"]).is_ok());
        assert!(resolve(&["--recommended", "--font-family", "no-such-font"]).is_err());
    }

    #[test]
    fn only_the_chosen_icon_set_is_staged() {
        let options = resolve(&["--recommended", "--icon-set", "tabler"]).unwrap();
        assert_eq!(options.icon_sets(), ["tabler"]);
    }
}
