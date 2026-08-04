use stayhydated_xtask::trunk::{TrunkDemoBuildConfig, TrunkDemoPageConfig};

pub fn run() -> anyhow::Result<()> {
    let workspace_root = stayhydated_xtask::workspace_root_from_xtask_manifest()?;
    stayhydated_xtask::trunk::build(
        &TrunkDemoBuildConfig::builder()
            .workspace_root(workspace_root)
            .example_dir("examples/gpui-demo")
            .output_dir("web/public/gpui-demo")
            .example_name("demo")
            .required_marker("gpui-es-fluent-demo")
            .toolchain("nightly")
            .generated_page(
                TrunkDemoPageConfig::builder()
                    .title("gpui-es-fluent demo")
                    .demo_name("Localized GPUI")
                    .build(),
            )
            .build(),
    )
}
