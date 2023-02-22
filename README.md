# Rust bindings for Asciidoctor.js in WASM

[![crates.io](https://img.shields.io/crates/v/asciidoctor-web.svg)](https://crates.io/crates/asciidoctor-web)
[![docs.rs](https://docs.rs/asciidoctor-web/badge.svg)](https://docs.rs/asciidoctor-web)

> Asciidoctor is a fast, open source, Ruby-based text processor for parsing AsciiDoc® into a document model and converting it to output formats such as HTML 5, DocBook 5, manual pages, PDF, EPUB 3, and other formats.

This crate provides WASM bindings for running Asciidoctor `convert` from a Rust, WASM application (like a web page).

It also provides (optional) direct integration with Yew by providing an `Asciidoc` Yew component.

## Updating the JS dependency

Go to `js/hell` and update the `package.json` and/or `package-lock.json` file. Re-run the build using `npm run build`.
If everything works, commit the output and run a build/release.