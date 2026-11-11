use topcoat_core::{context::Cx, error::Result};
use topcoat_view::{Attributes, Child, Class, StaticClass, View};
use topcoat_view_macro::{class, component, view};

/// The visual style of a [`button`].
///
/// [`Default`] is `ButtonVariant::Primary`, used when no variant is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ButtonVariant {
    /// The primary-filled button for the main action.
    #[default]
    Primary,
    /// A muted button for secondary actions.
    Secondary,
    /// An outlined button for less prominent actions.
    Outline,
    /// A text-only button for toolbars and inline actions.
    Ghost,
    /// A destructive-filled button for actions such as deleting data.
    Destructive,
}

impl ButtonVariant {
    fn classes(self) -> StaticClass {
        match self {
            Self::Primary => class!("primary"),
            Self::Secondary => class!("secondary"),
            Self::Outline => class!("outline"),
            Self::Ghost => class!("clear"),
            Self::Destructive => class!("error"),
        }
    }
}

const BASE: StaticClass = class!("button");

/// Builds the full class list for a button of the given `variant`.
///
/// Use it to give button styling to another element, such as a link:
///
/// ```
/// # let cx = topcoat::context::Cx::new();
/// # use topcoat::{chota::components::button::{ButtonVariant, button_variants}, view::view};
/// # let _ =
/// view! {
///     <a href="/login" class=(button_variants(ButtonVariant::Outline))>"Sign in"</a>
/// }
/// # ;
/// ```
#[must_use]
pub fn button_variants(variant: ButtonVariant) -> Class<(StaticClass, StaticClass)> {
    class!(BASE, variant.classes())
}

/// A styled button.
///
/// `variant` defaults to `Primary`. Pass the content as children. `attrs` are
/// forwarded to the `<button>`, with extra classes added to its classes. Use
/// [`button_variants`] to apply the same styling to another element.
///
/// ```
/// # let cx = topcoat::context::Cx::new();
/// # use topcoat::{chota::components::button::{ButtonVariant, button}, view::{attributes, view}};
/// # let _ =
/// view! {
///     button(
///         variant: ButtonVariant::Destructive,
///         attrs: attributes! { type="submit" },
///         "Delete"
///     )
/// }
/// # ;
/// ```
#[component]
pub async fn button(
    cx: &Cx,
    #[default] variant: ButtonVariant,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            class=(class!("button", variant.classes(), attrs.remove("class")))
            (attrs)
        >
            (child)
        </button>
    })
}
