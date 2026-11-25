#!/usr/bin/env bash
set -euo pipefail

selection=${1:?Pass default, no-default, or component}
dependency='default-features = false'
api='pub use gpui_es_fluent::{CurrentLanguage, I18n};

pub fn check_app(app: &mut gpui::App) {
    let _ = gpui_es_fluent::init(app);
    let _ = gpui_es_fluent::change_locale(app, "fr".parse::<unic_langid::LanguageIdentifier>().unwrap());
}'
case "$selection" in
  default) dependency='default-features = true' ;;
  no-default) ;;
  component)
    dependency+=', features = ["component"]'
    api+='
pub fn check_component_app(app: &mut gpui::App) {
    let _ = gpui_es_fluent::init_from_component_locale(app);
    let _ = gpui_es_fluent::set_component_locale(app, "fr");
    let _ = gpui_es_fluent::component_language();
}'
    ;;
  *) printf 'Unknown feature selection: %s\n' "$selection" >&2; exit 2 ;;
esac

repo_dir=$(pwd)
consumer=$(mktemp -d)
trap 'rm -rf "$consumer"' EXIT
mkdir "$consumer/src"
cat > "$consumer/Cargo.toml" <<EOF
[package]
name = "fluent-feature-consumer"
version = "0.0.0"
edition = "2024"
publish = false

[workspace]

[dependencies]
gpui = { package = "gpui-pre", version = "=0.3.14", default-features = false }
gpui-es-fluent = { path = "$repo_dir", $dependency }
unic-langid = "0.9.6"
EOF
printf '%s\n' "$api" > "$consumer/src/lib.rs"

# Seed resolution with the workspace lock, then admit the new consumer root.
# No demo or dev-dependencies can enable the component feature here.
cp Cargo.lock "$consumer/Cargo.lock"
cargo metadata --manifest-path "$consumer/Cargo.toml" --format-version 1 > /dev/null
cargo check --manifest-path "$consumer/Cargo.toml" --lib --locked \
  --target-dir "${CARGO_TARGET_DIR:-$repo_dir/target/feature-consumers}"
