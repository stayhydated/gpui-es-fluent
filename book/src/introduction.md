# Introduction

`gpui-es-fluent` makes an embedded `es-fluent` manager available as GPUI global
state. After installation, a view can localize typed messages and labels through
any context that borrows `gpui::App`.

This guide is for Rust developers adding localization to an existing GPUI
application. The application supplies its typed messages and locale assets;
`gpui-es-fluent` supplies the shared GPUI state and lookup helpers.

The integration has three parts:

- `es-fluent` derives typed messages and generates Fluent resources.
- `es-fluent-manager-embedded` discovers those resources and selects a language.
- `gpui-es-fluent` stores that manager as the `I18n` global so views can use it.

## Choose a task

Start with the page that matches the outcome you need:

- [Install the integration and render a message](getting_started.md).
- [Choose global installation and lookup behavior](global.md).
- [Select a startup or runtime locale](locales.md).
- [Synchronize `gpui-component` locale state](component.md).
