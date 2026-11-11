use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    router::{Slot, layout, module_router, page},
    tailwind::{
        self,
        components::{
            badge::{BadgeVariant, badge},
            button::{ButtonSize, ButtonVariant, button_variants},
        },
    },
    view::{View, attributes, class, view},
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

                // The stylesheet build.rs generated, served as an asset.
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
            </head>

            <body
                class="flex min-h-screen items-center justify-center bg-slate-100 font-sans"
            >
                (slot)
            </body>
        </html>
    })
}

#[page]
async fn home(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <main
            class="mx-4 w-full max-w-md rounded-2xl bg-white p-8 shadow-lg ring-1 ring-slate-200"
        >
            // Tailwind's `hidden` utility removes this warning, so it only
            // appears when the generated stylesheet is missing.
            <p class="hidden">
                "Tailwind is not working: this page should look styled."
            </p>

            // The inline style hides this badge until Tailwind's important
            // `flex!` utility overrides `display: none`.
            badge(
                variant: BadgeVariant::Secondary,
                attrs: attributes! { style="display: none" class="flex!" },
                "Tailwind is working"
            )

            <h1 class="mt-4 text-2xl font-bold tracking-tight text-slate-900">
                "Topcoat + Tailwind"
            </h1>

            <p class="mt-2 text-slate-600">
                "Utility classes in your Rust sources are compiled into this page's stylesheet by the standalone Tailwind CLI."
            </p>

            <a
                href="https://tailwindcss.com/docs"
                class=(class!(
                    "mt-6",
                    button_variants(ButtonVariant::Primary, ButtonSize::Md),
                ))
            >
                "Read the Tailwind docs"
            </a>
        </main>
    })
}
