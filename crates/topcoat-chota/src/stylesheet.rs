/// Returns Chota's minified stylesheet as a Topcoat asset.
///
/// The asset bundler downloads and caches the pinned stylesheet. Its checksum
/// is verified during bundling.
///
/// ```
/// # use topcoat::view::view;
/// # let cx = topcoat::context::Cx::new();
/// # let _ =
/// view! {
///     <link rel="stylesheet" href=(topcoat::chota::stylesheet!())>
/// }
/// # ;
/// ```
#[macro_export]
macro_rules! stylesheet {
    () => {
        ::topcoat::asset::asset!(
            "https://unpkg.com/chota@0.9.2/dist/chota.min.css",
            checksum: "sha256:f72b41c91c966b91ac7e1e3934c903aaabd7c0fbe4e01ffab75c1a2d7650f91d"
        )
    };
}
