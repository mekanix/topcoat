use topcoat_core::{context::Cx, error::Result};
use topcoat_view::{Attributes, Child, StaticClass, View};
use topcoat_view_macro::{class, component, view};

const CARD: StaticClass = class!("card");

/// A panel that groups related content.
///
/// Pass a header, body, footer, or table as children. `attrs` adds attributes
/// and classes to the card's `<div>`.
///
/// ```
/// # let cx = topcoat::context::Cx::new();
/// # use topcoat::{chota::components::card::{card, card_description, card_header, card_title}, view::view};
/// # let _ =
/// view! {
///     card(
///         card_header(
///             card_title("Workspace")
///             card_description("Manage your workspace settings.")
///         )
///     )
/// }
/// # ;
/// ```
#[component]
pub async fn card(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div class=(class!(CARD, attrs.remove("class"))) (attrs)>(child)</div> })
}

/// The opening section of a [`card`], stacking a [`card_title`] and an
/// optional [`card_description`].
#[component]
pub async fn card_header(
    cx: &Cx,
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <header (attrs)>(child)</header> })
}

/// The heading of a [`card`], rendered as an `<h3>`.
#[component]
pub async fn card_title(
    cx: &Cx,
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <h3 (attrs)>(child)</h3> })
}

/// The supporting text under a [`card_title`].
#[component]
pub async fn card_description(
    cx: &Cx,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p class=(class!("text-grey", attrs.remove("class"))) (attrs)>(child)</p>
    })
}

/// The main body of a [`card`].
#[component]
pub async fn card_content(
    cx: &Cx,
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <div (attrs)>(child)</div> })
}

/// The closing section of a [`card`], intended for actions.
#[component]
pub async fn card_footer(
    cx: &Cx,
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! { <footer (attrs)>(child)</footer> })
}
