# Chota support

[Chota](https://github.com/jenil/chota) is a small CSS framework with a grid, components, and utilities. This crate provides Topcoat's optional Chota integration, including the framework stylesheet and reusable view components.

Chota is an alternative to Tailwind for applications that want a conventional CSS framework without a CSS compilation step. Enable it with the `chota` feature on the `topcoat` dependency.

## Stylesheet delivery

`topcoat::chota::stylesheet!()` declares Chota's pinned, minified stylesheet as a Topcoat asset. The asset bundler downloads the stylesheet, verifies its checksum, and caches it. No `build.rs`, Node.js tooling, or platform-specific CSS compiler is required. This also makes the integration suitable for platforms where Tailwind's standalone compiler is unavailable, including FreeBSD.

The application serves the bundled stylesheet under `/_topcoat/assets` with a content-hashed filename by default. The CDN is used when bundling the asset, not by the browser when loading the application. External asset hosting remains available through Topcoat's asset configuration.

Chota itself requires no JavaScript. See the [Chota guide](../../docs/chota.md) for setup, asset bundling, and customization.

## Framework-owned components

The `topcoat::chota::components` module provides badges, buttons, cards, and inputs using Chota's native classes. Component variants express common visual styles while attributes and extra classes can still be passed to the underlying elements.

These components belong to the Chota integration so the component implementation and its CSS framework stay together. Generated applications import them instead of receiving copied component sources. This keeps Chota applications independent of Tailwind and lets the starter use the same component approach with either framework.

## Application generation

Select Chota when creating an application:

```sh
topcoat new my-app --recommended --css chota
```

The `--css tailwind|chota` option selects the framework. Tailwind is the default, except for `--minimal`, which uses plain CSS. `--no-css` explicitly selects plain CSS. UI components require a framework, so a preset that enables UI also needs `--no-ui` when combined with `--no-css`.

With `--ui`, generated code imports components from `topcoat::chota::components`. Without `--ui`, the application still receives Chota's stylesheet but does not use the component library.

## Starter theme

The Chota starter includes a small `theme.js` helper and a custom `styles.css`. Chota itself does not depend on this script.

The script reads the saved `topcoat-theme-mode` preference from local storage. If there is no saved light or dark preference, it uses the browser's `prefers-color-scheme` setting. It then sets `data-theme` on the root `<html>` element. The script runs before the stylesheets so the correct theme is selected before rendering.

The starter stylesheet defines Chota variables for both themes, including the page background, surfaces, text, and primary color. The body uses `var(--bg-color)`, ensuring that the page background follows the selected theme instead of remaining light in dark mode.

This crate is part of [`topcoat`](https://github.com/tokio-rs/topcoat).
