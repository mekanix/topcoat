[Chota](https://github.com/jenil/chota) is a small CSS framework with a grid, components, and utilities. It uses plain CSS and requires no JavaScript.

Topcoat provides Chota's minified stylesheet as an [asset](crate::asset). The asset bundler downloads the pinned release, verifies its checksum, and caches it. Your application serves the bundled stylesheet by default, so browsers do not need to connect to a CDN.

# Setup

Enable the `chota` feature. It also enables `asset`:

```toml
[dependencies]
topcoat = { version = "0.12.1", features = ["chota"] }
```

Link the stylesheet from your layout:

```rust
use topcoat::{
    Result,
    chota,
    context::Cx,
    router::{Slot, layout},
    view::{View, view},
};

#[layout]
async fn layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html>
            <head>
                <link rel="stylesheet" href=(chota::stylesheet!())>
            </head>
            <body>
                (slot)
            </body>
        </html>
    })
}
```

No `build.rs` is needed. `topcoat dev` bundles the stylesheet after each successful build. For a manual build, run:

```sh
topcoat asset bundle
```

Load the asset bundle on the router:

```rust,no_run
use topcoat::{
    asset::{AssetBundle, RouterBuilderAssetExt},
    router::{RouterBuilderDiscoverExt, module_router},
};

let router = module_router!()
    .discover()
    .assets(AssetBundle::load().unwrap())
    .build();
```

The router serves the stylesheet under `/_topcoat/assets` with a content-hashed filename. See the [assets guide](crate::asset) for deployment and external hosting.

# Customization

Chota exposes CSS variables for colors, typography, and grid settings. Override them in your own stylesheet and link that stylesheet after `chota::stylesheet!()`:

```css
:root {
  --color-primary: #7755cc;
  --grid-maxWidth: 96rem;
}
```

Declare a custom stylesheet with `asset!` as described in the [assets guide](crate::asset).
