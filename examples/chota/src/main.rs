use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    chota::{
        self,
        components::{
            badge::{BadgeVariant, badge},
            button::{ButtonVariant, button_variants},
            card::{card, card_content, card_description, card_header, card_title},
        },
    },
    context::Cx,
    router::{Slot, layout, module_router, page},
    view::{View, view},
};

#[tokio::main]
async fn main() {
    let router = module_router!()
        .assets(AssetBundle::load().unwrap())
        .build();

    topcoat::start(router).await.unwrap();
}

#[layout]
async fn root_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html>
            <head>
                topcoat::dev::script()
                <link rel="stylesheet" href=(chota::stylesheet!())>
            </head>
            <body>(slot)</body>
        </html>
    })
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <nav class="nav">
            <div class="nav-left"><a class="brand" href="/">"Topcoat + Chota"</a></div>
            <div class="nav-right">
                <a href="https://jenil.github.io/chota/">"Chota docs"</a>
            </div>
        </nav>

        <main class="container">
            <div class="row">
                <section class="col-12 col-8-md">
                    card(
                        card_header(
                            badge(variant: BadgeVariant::Secondary, "CSS only")
                            card_title("A small page with Chota")
                            card_description(
                                "Chota provides semantic HTML styles, a responsive grid, components, and utilities. The pinned stylesheet is bundled and served by Topcoat."
                            )
                        )
                        card_content(
                            <a
                                class=(button_variants(ButtonVariant::Primary))
                                href="https://github.com/jenil/chota"
                            >
                                "View Chota on GitHub"
                            </a>
                        )
                    )
                </section>

                <aside class="col-12 col-4-md">
                    card(
                        card_header(
                            card_title("No build script")
                            card_description(
                                "Enable the feature, link the stylesheet, and load the asset bundle."
                            )
                        )
                        card_content(
                            <span class="text-success">"No JavaScript required"</span>
                        )
                    )
                </aside>
            </div>
        </main>
    })
}
