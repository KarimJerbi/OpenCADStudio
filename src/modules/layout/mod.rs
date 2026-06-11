// Layout module — paper space tools (viewports, scale, plot settings).
// This tab is only shown when the active layout is not "Model".

pub mod mview;
pub mod vplayer;

use crate::modules::{CadModule, IconKind, ModuleEvent, RibbonGroup, ToolDef};

/// Paper-space context tools, as a flat list for the right-edge side toolbar
/// (the contextual ribbon tab is no longer shown). Viewport + plot actions.
pub fn paper_space_tools() -> Vec<ToolDef> {
    vec![
        mview::tool(),
        ToolDef {
            id: "PAGESETUP",
            label: "Page Setup",
            icon: IconKind::Glyph("📋"),
            event: ModuleEvent::Command("PAGESETUP".to_string()),
        },
        ToolDef {
            id: "PLOT",
            label: "Export PDF",
            icon: IconKind::Glyph("🖨"),
            event: ModuleEvent::Command("PLOT".to_string()),
        },
    ]
}

pub struct LayoutModule;

impl CadModule for LayoutModule {
    fn id(&self) -> &'static str {
        "layout"
    }
    fn title(&self) -> &'static str {
        "Layout"
    }

    fn ribbon_groups(&self) -> Vec<RibbonGroup> {
        vec![
            RibbonGroup {
                title: "Viewport",
                tools: vec![mview::tool().into()],
            },
            RibbonGroup {
                title: "Plot",
                tools: vec![
                    ToolDef {
                        id: "PAGESETUP",
                        label: "Page Setup",
                        icon: IconKind::Glyph("📋"),
                        event: ModuleEvent::Command("PAGESETUP".to_string()),
                    }
                    .into(),
                    ToolDef {
                        id: "PLOT",
                        label: "Export PDF",
                        icon: IconKind::Glyph("🖨"),
                        event: ModuleEvent::Command("PLOT".to_string()),
                    }
                    .into(),
                ],
            },
        ]
    }
}
