// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only
// Enhanced for Axiq-IQ / ZIQA OS

use cosmic::iced::Color;
use cosmic::widget::{button, column, container, icon, row, settings, text};
use cosmic::{Task, theme};
use cosmic_settings_page::{self as page, Section, section};

#[derive(Clone, Debug)]
pub enum Message {
    Check,
    Update,
    Status(String),
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Updates(message)
    }
}

#[derive(Default)]
pub struct Page {
    entity: page::Entity,
    status: String,
    last_checked: Option<String>,
    updates_available: bool,
}

impl page::Page<crate::pages::Message> for Page {
    fn set_id(&mut self, entity: page::Entity) {
        self.entity = entity;
    }

    fn content(
        &self,
        sections: &mut slotmap::SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![
            sections.insert(updates_hero()),
            sections.insert(updates_section()),
        ])
    }

    fn info(&self) -> page::Info {
        page::Info::new("updates", "system-software-update-symbolic")
            .title(fl!("updates"))
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn updates_hero() -> Section<crate::pages::Message> {
    Section::default()
        .view::<Page>(move |_binder, page, _section| {
            let gold = Color::from_rgb8(0xC8, 0xA9, 0x51);
            
            let status_text = if page.status.is_empty() {
                if page.updates_available {
                    "Updates available for Axiq-IQ"
                } else {
                    "Your system is up to date"
                }
            } else {
                &page.status
            };

            container(
                column::with_capacity(3)
                    .push(
                        row::with_capacity(2)
                            .push(
                                container(icon::from_name("system-software-update-symbolic").size(40))
                                    .padding(10)
                                    .class(theme::Container::Custom(Box::new(move |_| {
                                        let mut bg = gold;
                                        bg.a = 0.15;
                                        cosmic::iced::widget::container::Style {
                                            background: Some(cosmic::iced::Background::Color(bg)),
                                            icon_color: Some(gold),
                                            border: cosmic::iced::Border {
                                                radius: [12.0; 4].into(),
                                                width: 1.5,
                                                color: gold,
                                            },
                                            ..Default::default()
                                        }
                                    })))
                            )
                            .push(
                                column::with_capacity(2)
                                    .push(text(status_text).size(20).font(cosmic::font::bold()))
                                    .push(text("Axiq-IQ / ZIQA OS Rolling Release").size(12).class(theme::Text::Color(Color { a: 0.6, ..gold })))
                            )
                            .spacing(20)
                            .align_y(cosmic::iced::Alignment::Center)
                    )
                    .spacing(16)
            )
            .padding(24)
            .width(cosmic::iced::Length::Fill)
            .class(theme::Container::Custom(Box::new(move |_| {
                let mut bg = gold;
                bg.a = 0.08;
                cosmic::iced::widget::container::Style {
                    background: Some(cosmic::iced::Background::Color(bg)),
                    border: cosmic::iced::Border {
                        radius: [14.0; 4].into(),
                        width: 1.5,
                        color: Color { a: 0.3, ..gold },
                    },
                    ..Default::default()
                }
            })))
            .into()
        })
}

fn updates_section() -> Section<crate::pages::Message> {
    Section::default()
        .view::<Page>(move |_binder, page, _section| {
            let mut s = settings::section();
            
            s = s.add(
                settings::item::builder(fl!("updates", "check"))
                    .icon(icon::from_name("view-refresh-symbolic").size(20))
                    .control(
                        button::text(fl!("updates", "check"))
                            .on_press(Message::Check.into())
                    )
            );

            if page.updates_available {
                s = s.add(
                    settings::item::builder(fl!("updates", "update"))
                        .icon(icon::from_name("system-software-update-symbolic").size(20))
                        .control(
                            button::suggested(fl!("updates", "update"))
                                .on_press(Message::Update.into())
                        )
                );
            }

            if let Some(last) = &page.last_checked {
                s = s.add(
                    settings::item::builder(fl!("updates", "last-checked", time = last))
                        .icon(icon::from_name("preferences-system-time-symbolic").size(20))
                        .control(text(""))
                );
            }

            s.into()
        })
}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::pages::Message> {
        match message {
            Message::Check => {
                self.status = fl!("updates", "syncing");
                return Task::future(async move {
                    // Simulating 'emerge --sync'
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    Message::Status("updates_available".to_string()).into()
                });
            }
            Message::Update => {
                self.status = fl!("updates", "upgrading");
                return Task::future(async move {
                    // Simulating 'emerge -auDU @world'
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                    Message::Status("success".to_string()).into()
                });
            }
            Message::Status(status) => {
                if status == "updates_available" {
                    self.status = String::new();
                    self.updates_available = true;
                    self.last_checked = Some("Just now".to_string());
                } else if status == "success" {
                    self.status = String::new();
                    self.updates_available = false;
                    self.last_checked = Some("Just now".to_string());
                } else {
                    self.status = status;
                }
            }
        }
        Task::none()
    }
}
