// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{collections::BTreeMap, path::PathBuf};

use super::Message;
use cosmic::{
    Apply, Element,
    iced::{Background, Length},
    widget::{button, icon, text},
};
use tokio::io::AsyncBufReadExt;

#[allow(dead_code)]
const CURSOR_PREV_N: usize = 1;
const CURSOR_THUMB_SIZE: u16 = 32;
const CURSOR_NAME_TRUNC: usize = 20;

pub type CursorThemes = Vec<CursorTheme>;

/// Button with a preview of the cursor theme.
pub fn button(
    name: &str,
    id: usize,
    selected: bool,
    callback: impl Fn(usize) -> super::Message,
) -> Element<'static, Message> {
    let theme = cosmic::theme::active();
    let theme = theme.cosmic();
    let background = Background::Color(theme.palette.neutral_4.into());

    cosmic::widget::column::with_capacity(2)
        .push(
            cosmic::widget::button::custom(
                cosmic::widget::container(
                    icon::from_name("preferences-desktop-cursor-symbolic")
                        .size(CURSOR_THUMB_SIZE)
                        .apply(cosmic::widget::container)
                        .padding(theme.space_xs())
                )
                .center_x(Length::Fill)
                .center_y(Length::Fill),
            )
            .on_press(callback(id))
            .selected(selected)
            .padding(theme.space_xs())
            .width(Length::Fixed(80.0))
            .height(Length::Fixed(80.0))
            .class(button::ButtonClass::Custom {
                active: Box::new(move |focused, theme| {
                    let mut appearance = <cosmic::theme::Theme as button::Catalog>::active(
                        theme,
                        focused,
                        selected,
                        &cosmic::theme::Button::Image,
                    );
                    appearance.background = Some(background);
                    appearance
                }),
                disabled: Box::new(move |theme| {
                    let mut appearance = <cosmic::theme::Theme as button::Catalog>::disabled(
                        theme,
                        &cosmic::theme::Button::Image,
                    );
                    appearance.background = Some(background);
                    appearance
                }),
                hovered: Box::new(move |focused, theme| {
                    let mut appearance = <cosmic::theme::Theme as button::Catalog>::hovered(
                        theme,
                        focused,
                        selected,
                        &cosmic::theme::Button::Image,
                    );
                    appearance.background = Some(background);
                    appearance
                }),
                pressed: Box::new(move |focused, theme| {
                    let mut appearance = <cosmic::theme::Theme as button::Catalog>::pressed(
                        theme,
                        focused,
                        selected,
                        &cosmic::theme::Button::Image,
                    );
                    appearance.background = Some(background);
                    appearance
                }),
            }),
        )
        .push(
            text::body(if name.len() > CURSOR_NAME_TRUNC {
                format!("{name:.CURSOR_NAME_TRUNC$}...")
            } else {
                name.into()
            })
            .width(Length::Fixed(80.0)),
        )
        .spacing(theme.space_xxs())
        .align_x(cosmic::iced::Alignment::Center)
        .into()
}

/// Find all cursor themes available on the system.
pub async fn fetch() -> Message {
    let mut cursor_themes = BTreeMap::new();

    let mut buffer = String::new();

    let xdg_data_home = std::env::var("XDG_DATA_HOME")
        .ok()
        .and_then(|value| {
            if value.is_empty() {
                None
            } else {
                Some(PathBuf::from(value))
            }
        })
        .or_else(dirs::home_dir)
        .map(|dir| dir.join(".local/share/icons"));

    let xdg_data_dirs = std::env::var("XDG_DATA_DIRS").ok();

    let xdg_data_dirs = xdg_data_dirs
        .as_deref()
        // Default from the XDG Base Directory Specification
        .or(Some("/usr/local/share/:/usr/share/"))
        .into_iter()
        .flat_map(|arg| std::env::split_paths(arg).map(|dir| dir.join("icons")));

    for icon_dir in xdg_data_dirs.chain(xdg_data_home) {
        let Ok(read_dir) = std::fs::read_dir(&icon_dir) else {
            continue;
        };

        'icon_dir: for entry in read_dir.filter_map(Result::ok) {
            let Ok(path) = entry.path().canonicalize() else {
                continue;
            };

            let Some(id) = entry.file_name().to_str().map(String::from) else {
                continue;
            };

            // Check if it's a cursor theme: it must have a 'cursors' directory
            if !path.join("cursors").is_dir() {
                continue;
            }

            let manifest = path.join("index.theme");
            let mut name = None;

            if manifest.exists() {
                if let Ok(file) = tokio::fs::File::open(&manifest).await {
                    buffer.clear();
                    let mut line_reader = tokio::io::BufReader::new(file);
                    while let Ok(read) = line_reader.read_line(&mut buffer).await {
                        if read == 0 {
                            break;
                        }

                        if let Some(is_hidden) = buffer.strip_prefix("Hidden=") {
                            if is_hidden.trim() == "true" {
                                continue 'icon_dir;
                            }
                        } else if name.is_none()
                            && let Some(value) = buffer.strip_prefix("Name=")
                        {
                            name = Some(value.trim().to_owned());
                        }
                        buffer.clear();
                    }
                }
            }

            let name = name.unwrap_or_else(|| id.clone());
            cursor_themes.insert(id, name);
        }
    }

    Message::DrawerCursor(super::drawer::CursorMessage::CursorLoaded(
        cursor_themes.into_iter().map(|(id, name)| CursorTheme { id, name }).collect(),
    ))
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CursorTheme {
    pub id: String,
    pub name: String,
}
