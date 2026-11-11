use topcoat_core::{context::Cx, error::Result};
use topcoat_view::{Attributes, Child, Class, StaticClass, View};
use topcoat_view_macro::{class, component, view};

/// The visual style of a [`badge`].
///
/// [`Default`] is `BadgeVariant::Primary`, used when no variant is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum BadgeVariant {
    /// The primary-filled badge for highlighted statuses.
    #[default]
    Primary,
    /// A light badge for neutral statuses.
    Secondary,
    /// A primary-outlined badge on the page background.
    Outline,
    /// An error-filled badge for errors and warnings.
    Destructive,
}

impl BadgeVariant {
    fn classes(self) -> StaticClass {
        match self {
            Self::Primary => class!("bg-primary text-white"),
            Self::Secondary => class!("bg-light"),
            Self::Outline => class!("bd-primary text-primary"),
            Self::Destructive => class!("bg-error text-white"),
        }
    }
}

const BASE: StaticClass = class!("tag");

/// Builds the full class list for a badge of the given `variant`.
///
/// Use it to give badge styling to another element, such as a link:
///
/// ```
/// # let cx = topcoat::context::Cx::new();
/// # use topcoat::{chota::components::badge::{BadgeVariant, badge_variants}, view::view};
/// # let _ =
/// view! {
///     <a href="/releases/v2" class=(badge_variants(BadgeVariant::Outline))>"v2.0"</a>
/// }
/// # ;
/// ```
#[must_use]
pub fn badge_variants(variant: BadgeVariant) -> Class<(StaticClass, StaticClass)> {
    class!(BASE, variant.classes())
}

/// A small label for a status or count.
///
/// `variant` defaults to `Primary`. Pass the label as children and extra attributes
/// through `attrs`. Attributes go on the `<span>`, with classes added to its classes.
/// Use [`badge_variants`] to apply the same styling to another element.
///
/// ```
/// # let cx = topcoat::context::Cx::new();
/// # use topcoat::{chota::components::badge::{BadgeVariant, badge}, view::view};
/// # let _ =
/// view! {
///     badge(variant: BadgeVariant::Destructive, "Failed")
/// }
/// # ;
/// ```
#[component]
pub async fn badge(
    cx: &Cx,
    #[default] variant: BadgeVariant,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span class=(class!("tag", variant.classes(), attrs.remove("class"))) (attrs)>
            (child)
        </span>
    })
}
