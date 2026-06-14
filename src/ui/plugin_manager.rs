//! Plugin Manager window — lists the add-ons compiled into this build.
//!
//! Phase-1 stub: read-only inventory of installed plugins (name, version, id,
//! API level, description). Enable/disable and dynamic loading come with the
//! phase-2 loader; see `docs/plugin-architecture.md`.

use crate::app::Message;
use crate::plugin::manifest::PluginManifest;
use iced::widget::{column, container, row, scrollable, text, Space};
use iced::{Background, Border, Color, Element, Fill, Theme};

// Register the command names for autocomplete.
inventory::submit!(crate::command::CommandRegistration {
    names: &["PLUGINS", "PLUGINMANAGER"]
});

const BG: Color = Color {
    r: 0.15,
    g: 0.15,
    b: 0.15,
    a: 1.0,
};
const CARD: Color = Color {
    r: 0.12,
    g: 0.12,
    b: 0.12,
    a: 1.0,
};
const BORDER: Color = Color {
    r: 0.30,
    g: 0.30,
    b: 0.30,
    a: 1.0,
};
const DIM: Color = Color {
    r: 0.55,
    g: 0.55,
    b: 0.55,
    a: 1.0,
};
const ACCENT: Color = Color {
    r: 0.30,
    g: 0.62,
    b: 0.95,
    a: 1.0,
};
const WHITE: Color = Color {
    r: 0.92,
    g: 0.92,
    b: 0.92,
    a: 1.0,
};

fn badge<'a>(label: String) -> Element<'a, Message> {
    container(text(label).size(11).color(WHITE))
        .padding([2, 8])
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(Color {
                r: 0.20,
                g: 0.34,
                b: 0.52,
                a: 1.0,
            })),
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
}

fn plugin_card<'a>(m: &PluginManifest) -> Element<'a, Message> {
    let header = row![
        text(m.name.to_string()).size(15).color(WHITE),
        Space::new().width(Fill),
        badge(format!("v{}", m.version)),
        Space::new().width(8),
        badge(format!("API {}", m.api_version.major)),
    ]
    .align_y(iced::Center);

    let id_line = text(m.id.to_string()).size(11).color(ACCENT);
    let desc = text(m.description.to_string()).size(12).color(DIM);

    let mut body = column![header, id_line, desc].spacing(5);

    if !m.command_prefixes.is_empty() {
        body = body.push(
            text(format!("Commands: {}", m.command_prefixes.join(", ")))
                .size(11)
                .color(DIM),
        );
    }

    container(body.padding([12, 14]))
        .width(Fill)
        .style(|_: &Theme| container::Style {
            background: Some(Background::Color(CARD)),
            border: Border {
                color: BORDER,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
        .into()
}

pub fn view_window<'a>(plugins: &[&'static PluginManifest]) -> Element<'a, Message> {
    let title = text("Installed Plugins").size(20).color(WHITE);
    let subtitle = text(format!(
        "{} add-on{} compiled into this build",
        plugins.len(),
        if plugins.len() == 1 { "" } else { "s" }
    ))
    .size(12)
    .color(DIM);

    let body: Element<'_, Message> = if plugins.is_empty() {
        container(text("No plugins installed.").size(13).color(DIM))
            .padding(20)
            .into()
    } else {
        let mut list = column![].spacing(10);
        for m in plugins {
            list = list.push(plugin_card(m));
        }
        scrollable(list.width(Fill)).height(Fill).into()
    };

    container(
        column![title, subtitle, Space::new().height(12), body]
            .spacing(4)
            .padding(20)
            .width(Fill)
            .height(Fill),
    )
    .style(|_: &Theme| container::Style {
        background: Some(Background::Color(BG)),
        ..Default::default()
    })
    .width(Fill)
    .height(Fill)
    .into()
}
