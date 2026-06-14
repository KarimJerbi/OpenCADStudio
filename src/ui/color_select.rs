//! Shared colour selector: a main-colour dropdown plus a "more colours" button
//! that expands the full ACI palette. Used by the properties panel and every
//! style editor so colour selection looks and behaves the same everywhere.

use crate::app::Message;
use crate::ui::properties::acad_color_display;
use acadrust::types::Color as AcadColor;
use iced::widget::{button, column, container, pick_list, row, scrollable, text};
use iced::{Background, Border, Color, Element, Theme};

/// Which "logical" entries the main dropdown offers besides the standard ACI
/// colours.
#[derive(Clone, Copy, Default)]
pub struct ColorExtras {
    pub by_layer: bool,
    pub by_block: bool,
}

/// Encode a colour as the ACI integer string the style editors store
/// (ByBlock=0, ByLayer=256, indexed 1-255; RGB has no ACI slot → ByLayer).
pub fn color_to_aci_string(c: AcadColor) -> String {
    match c {
        AcadColor::ByBlock => "0".to_string(),
        AcadColor::ByLayer => "256".to_string(),
        AcadColor::Index(i) => i.to_string(),
        AcadColor::Rgb { .. } => "256".to_string(),
    }
}

/// Decode an ACI integer string back into an `AcadColor`.
pub fn aci_string_to_color(s: &str) -> AcadColor {
    match s.trim().parse::<i16>().unwrap_or(256) {
        0 => AcadColor::ByBlock,
        256 => AcadColor::ByLayer,
        n if (1..=255).contains(&n) => AcadColor::Index(n as u8),
        _ => AcadColor::ByLayer,
    }
}

/// A dropdown option wrapping an `AcadColor`, displayed by its colour name.
#[derive(Clone, PartialEq)]
struct ColorChoice(AcadColor);

impl std::fmt::Display for ColorChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (_, name) = acad_color_display(self.0);
        write!(f, "{name}")
    }
}

const PICKER_BG: Color = Color {
    r: 0.12,
    g: 0.12,
    b: 0.12,
    a: 1.0,
};
const BORDER: Color = Color {
    r: 0.35,
    g: 0.35,
    b: 0.35,
    a: 1.0,
};

/// Build a colour selector.
///
/// * `current` — the currently selected colour (shown in the dropdown).
/// * `palette_open` — whether the expanded 255-colour grid is shown.
/// * `extras` — whether ByLayer / ByBlock appear in the dropdown.
/// * `on_select` — called with the chosen colour (dropdown item or grid swatch).
/// * `on_more` — toggles the expanded palette.
pub fn color_selector<'a>(
    current: AcadColor,
    palette_open: bool,
    extras: ColorExtras,
    on_select: impl Fn(AcadColor) -> Message + 'a,
    on_more: Message,
) -> Element<'a, Message> {
    // Standard ACI 1-7 plus the requested extras, and the current colour if it
    // isn't already one of them (so a non-standard index still shows).
    let mut opts: Vec<ColorChoice> = Vec::new();
    if extras.by_layer {
        opts.push(ColorChoice(AcadColor::ByLayer));
    }
    if extras.by_block {
        opts.push(ColorChoice(AcadColor::ByBlock));
    }
    for i in 1u8..=7 {
        opts.push(ColorChoice(AcadColor::Index(i)));
    }
    let cur_choice = ColorChoice(current);
    if !opts.contains(&cur_choice) {
        opts.push(cur_choice.clone());
    }

    // Expanded ACI palette. Built before the dropdown closure so it can borrow
    // `on_select`; the dropdown then moves it.
    let grid: Option<Element<'a, Message>> = if palette_open {
        const COLS: u16 = 16;
        let mut rows = column![].spacing(1);
        let mut idx: u16 = 1;
        while idx <= 255 {
            let mut r = row![].spacing(1);
            for _ in 0..COLS {
                if idx > 255 {
                    break;
                }
                let ci = idx as u8;
                let (bg, _) = acad_color_display(AcadColor::Index(ci));
                let msg = on_select(AcadColor::Index(ci));
                r = r.push(
                    button(text("").width(12).height(12))
                        .on_press(msg)
                        .style(move |_: &Theme, status| button::Style {
                            background: Some(Background::Color(bg)),
                            border: Border {
                                color: if matches!(status, button::Status::Hovered) {
                                    Color::WHITE
                                } else {
                                    Color {
                                        r: 0.0,
                                        g: 0.0,
                                        b: 0.0,
                                        a: 0.4,
                                    }
                                },
                                width: if matches!(status, button::Status::Hovered) {
                                    1.5
                                } else {
                                    1.0
                                },
                                radius: 1.0.into(),
                            },
                            ..Default::default()
                        })
                        .padding(0),
                );
                idx += 1;
            }
            rows = rows.push(r);
        }
        Some(
            container(scrollable(rows).height(150))
                .style(|_: &Theme| container::Style {
                    background: Some(Background::Color(PICKER_BG)),
                    border: Border {
                        color: BORDER,
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                })
                .padding(4)
                .into(),
        )
    } else {
        None
    };

    let dropdown = pick_list(opts, Some(cur_choice), move |c| on_select(c.0))
        .text_size(11)
        .width(150);
    let more = button(text(if palette_open { "×" } else { "⋯" }).size(11))
        .on_press(on_more)
        .padding([2, 7]);
    let top = row![dropdown, more].spacing(4).align_y(iced::Center);

    match grid {
        Some(g) => column![top, g].spacing(2).into(),
        None => top.into(),
    }
}
