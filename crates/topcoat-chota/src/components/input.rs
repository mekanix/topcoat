use topcoat_core::{context::Cx, error::Result};
use topcoat_view::{Attributes, View};
use topcoat_view_macro::{class, component, view};

/// A styled input using Chota's native form styling.
///
/// Pass input attributes and event handlers through `attrs`. Extra classes are
/// added to the input's classes. Chota fills the container by default.
///
/// ```
/// # let cx = topcoat::context::Cx::new();
/// # use topcoat::{chota::components::input::input, view::{attributes, view}};
/// # let _ =
/// view! {
///     input(attrs: attributes! { type="email" placeholder="you@example.com" })
/// }
/// # ;
/// ```
#[component]
pub async fn input(cx: &Cx, #[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! { <input class=(class!(attrs.remove("class"))) (attrs)> })
}
