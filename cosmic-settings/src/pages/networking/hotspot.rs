// Copyright 2024 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::process::Stdio;

use cosmic::{Apply, Element, Task, widget};
use cosmic::iced::Length;
use cosmic_settings_page::{self as page, Section, section};

#[derive(Debug)]
pub struct Page {
    ssid: String,
    password: String,
    password_hidden: bool,
    qr_code_data: Option<widget::qr_code::Data>,
    state: HotspotState,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            ssid: String::new(),
            password: String::new(),
            password_hidden: true,
            qr_code_data: None,
            state: HotspotState::default(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct HotspotState {
    active: bool,
    connection_name: Option<String>,
    ssid: Option<String>,
    password: Option<String>,
    device: Option<String>,
    message: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Message {
    Error(String),
    Refresh,
    SetPassword(String),
    SetSsid(String),
    Start,
    Started(Result<(), String>),
    StateLoaded(HotspotState),
    Stop,
    Stopped(Result<(), String>),
    TogglePasswordVisibility,
}

impl From<Message> for crate::app::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Hotspot(message).into()
    }
}

impl From<Message> for crate::pages::Message {
    fn from(message: Message) -> Self {
        crate::pages::Message::Hotspot(message)
    }
}

impl page::Page<crate::pages::Message> for Page {
    fn info(&self) -> page::Info {
        page::Info::new("hotspot", "network-wireless-hotspot-symbolic")
            .title("Hotspot")
            .description("Create and control a local Wi-Fi hotspot")
    }

    fn content(
        &self,
        sections: &mut slotmap::SlotMap<section::Entity, Section<crate::pages::Message>>,
    ) -> Option<page::Content> {
        Some(vec![sections.insert(view())])
    }

    fn on_enter(&mut self) -> Task<crate::pages::Message> {
        Task::future(async { load_state().await }).map(crate::pages::Message::Hotspot)
    }
}

impl page::AutoBind<crate::pages::Message> for Page {}

fn escape_wifi_qr_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace(';', "\\;")
        .replace(',', "\\,")
        .replace(':', "\\:")
        .replace('"', "\\\"")
}

impl Page {
    pub fn update(&mut self, message: Message) -> Task<crate::app::Message> {
        match message {
            Message::Refresh => {
                return Task::future(async { load_state().await })
                    .map(crate::app::Message::from)
                    .map(Into::into);
            }
            Message::SetSsid(ssid) => self.ssid = ssid,
            Message::SetPassword(password) => self.password = password,
            Message::Start => {
                self.state.message = None;
                let ssid = self.ssid.trim().to_string();
                let password = self.password.trim().to_string();
                return Task::future(async move {
                    Message::Started(start_hotspot(ssid, password).await)
                })
                .map(crate::app::Message::from)
                .map(Into::into);
            }
            Message::Stop => {
                self.state.message = None;
                let connection_name = self.state.connection_name.clone();
                return Task::future(async move {
                    Message::Stopped(stop_hotspot(connection_name).await)
                })
                .map(crate::app::Message::from)
                .map(Into::into);
            }
            Message::Started(result) => {
                if let Err(err) = result {
                    self.state.message = Some(err);
                }
                return Task::future(async { load_state().await })
                    .map(crate::app::Message::from)
                    .map(Into::into);
            }
            Message::Stopped(result) => {
                if let Err(err) = result {
                    self.state.message = Some(err);
                }
                return Task::future(async { load_state().await })
                    .map(crate::app::Message::from)
                    .map(Into::into);
            }
            Message::StateLoaded(state) => {
                if self.ssid.is_empty() {
                    self.ssid = state
                        .ssid
                        .clone()
                        .unwrap_or_else(|| "ZIQA Hotspot".to_string());
                } else if self.state.ssid != state.ssid && state.ssid.is_some() {
                    self.ssid = state.ssid.clone().unwrap();
                }

                if self.password.is_empty() {
                    self.password = state
                        .password
                        .clone()
                        .unwrap_or_default();
                } else if self.state.password != state.password && state.password.is_some() {
                    self.password = state.password.clone().unwrap();
                }

                if state.active {
                    let qr_ssid = state.ssid.as_ref().unwrap_or(&self.ssid);
                    let qr_pass = state.password.as_ref().unwrap_or(&self.password);
                    let escaped_ssid = escape_wifi_qr_string(qr_ssid);
                    let qr_string = if !qr_pass.is_empty() {
                        let escaped_password = escape_wifi_qr_string(qr_pass);
                        format!("WIFI:T:WPA;S:{};P:{};;", escaped_ssid, escaped_password)
                    } else {
                        format!("WIFI:T:;S:{};;", escaped_ssid)
                    };
                    self.qr_code_data = widget::qr_code::Data::new(qr_string).ok();
                } else {
                    self.qr_code_data = None;
                }

                self.state = state;
            }
            Message::Error(err) => self.state.message = Some(err),
            Message::TogglePasswordVisibility => {
                self.password_hidden = !self.password_hidden;
            }
        }

        Task::none()
    }
}

fn view() -> Section<crate::pages::Message> {
    Section::default().view::<Page>(move |_binder, page, _section| {
        let spacing = cosmic::theme::spacing();

        let status_text = if page.state.active {
            page.state
                .ssid
                .as_ref()
                .map(|ssid| format!("Active on {ssid}"))
                .unwrap_or_else(|| "Hotspot active".to_string())
        } else {
            "Hotspot is stopped".to_string()
        };

        let details = [
            page.state
                .device
                .as_ref()
                .map(|device| format!("Device: {device}")),
            page.state
                .connection_name
                .as_ref()
                .map(|name| format!("Connection: {name}")),
            page.state.message.clone(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

        let mut start_btn = widget::button::suggested(if page.state.active {
            "Apply changes"
        } else {
            "Start hotspot"
        });
        if page.password.is_empty() || page.password.len() >= 8 {
            start_btn = start_btn.on_press(Message::Start);
        }

        let mut stop_btn = widget::button::standard("Stop hotspot");
        if page.state.active {
            stop_btn = stop_btn.on_press(Message::Stop);
        }

        let mut controls = widget::column::with_capacity(5)
            .push(
                widget::settings::section()
                    .title("Hotspot")
                    .add(widget::settings::item(
                        "Status",
                        widget::container(widget::text::body(status_text))
                            .padding([6, 10])
                            .class(super::network_status_style()),
                    ))
                    .add(widget::settings::item(
                        "SSID",
                        widget::text_input("", &page.ssid).on_input(Message::SetSsid),
                    ))
                    .add(widget::settings::item(
                        "Password",
                        widget::text_input::secure_input(
                            "Password",
                            &page.password,
                            Some(Message::TogglePasswordVisibility),
                            page.password_hidden
                        )
                        .on_input(Message::SetPassword),
                    ))
                    .add(widget::settings::item(
                        "Actions",
                        widget::row::with_capacity(3)
                            .push(start_btn)
                            .push(stop_btn)
                            .push(widget::button::text("Refresh").on_press(Message::Refresh))
                            .spacing(spacing.space_s),
                    )),
            );

        if let Some(qr_data) = &page.qr_code_data {
            controls = controls.push(
                widget::settings::section()
                    .title("Share Hotspot")
                    .add(widget::settings::item(
                        "",
                        widget::container(widget::qr_code(qr_data).cell_size(5))
                            .center_x(Length::Fill)
                            .padding([12, 12]),
                    )),
            );
        }

        controls = controls
            .push_maybe((!details.is_empty()).then(|| {
                let items = details
                    .into_iter()
                    .fold(widget::column::with_capacity(3), |col, line| {
                        col.push(widget::text::body(line))
                    });
                widget::settings::section()
                    .title("Status details")
                    .add(widget::container(items.spacing(spacing.space_xxs)).padding([0, 4]))
            }))
            .spacing(spacing.space_l);

        controls
            .apply(Element::from)
            .map(crate::pages::Message::Hotspot)
    })
}

async fn load_state() -> Message {
    match hotspot_status().await {
        Ok(state) => Message::StateLoaded(state),
        Err(err) => Message::Error(err),
    }
}

async fn hotspot_status() -> Result<HotspotState, String> {
    let output = tokio::process::Command::new("nmcli")
        .args([
            "-t",
            "-f",
            "NAME,DEVICE,TYPE",
            "connection",
            "show",
            "--active",
        ])
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|err| err.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let active_lines = String::from_utf8_lossy(&output.stdout);
    let active = active_lines.lines().find_map(|line| {
        let mut parts = line.split(':');
        let name = parts.next()?.to_string();
        let device = parts.next().map(str::to_string);
        let kind = parts.next()?;
        (kind == "802-11-wireless" && name.to_lowercase().contains("hotspot"))
            .then_some((name, device))
    });

    let mut state = HotspotState::default();
    if let Some((name, device)) = active {
        state.active = true;
        state.connection_name = Some(name.clone());
        state.device = device;
        state.ssid = hotspot_ssid(&name).await.ok();
        state.password = hotspot_password(&name).await.ok();
    }

    Ok(state)
}

async fn hotspot_ssid(connection_name: &str) -> Result<String, String> {
    let output = tokio::process::Command::new("nmcli")
        .args([
            "-g",
            "802-11-wireless.ssid",
            "connection",
            "show",
            connection_name,
        ])
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|err| err.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

async fn hotspot_password(connection_name: &str) -> Result<String, String> {
    let output = tokio::process::Command::new("nmcli")
        .args([
            "-s",
            "-g",
            "802-11-wireless-security.psk",
            "connection",
            "show",
            connection_name,
        ])
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|err| err.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

async fn start_hotspot(ssid: String, password: String) -> Result<(), String> {
    if ssid.trim().is_empty() {
        return Err("SSID is required".to_string());
    }

    let wifi_ifname = default_wifi_ifname().await?;

    let mut args = vec![
        "device",
        "wifi",
        "hotspot",
        "ifname",
        wifi_ifname.as_str(),
        "ssid",
        ssid.trim(),
    ];

    if !password.is_empty() {
        if password.len() < 8 {
            return Err("Password must be at least 8 characters".to_string());
        }
        args.push("password");
        args.push(password.as_str());
    }

    let output = tokio::process::Command::new("nmcli")
        .args(args)
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|err| err.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

async fn stop_hotspot(connection_name: Option<String>) -> Result<(), String> {
    let connection_name = if let Some(name) = connection_name {
        name
    } else {
        hotspot_status()
            .await?
            .connection_name
            .ok_or_else(|| "No active hotspot connection was found".to_string())?
    };

    let output = tokio::process::Command::new("nmcli")
        .args(["connection", "down", connection_name.as_str()])
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|err| err.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

async fn default_wifi_ifname() -> Result<String, String> {
    let output = tokio::process::Command::new("nmcli")
        .args(["-t", "-f", "DEVICE,TYPE", "device", "status"])
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|err| err.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find_map(|line| {
            let mut parts = line.split(':');
            let device = parts.next()?;
            let kind = parts.next()?;
            (kind == "wifi").then(|| device.to_string())
        })
        .ok_or_else(|| "No Wi-Fi device is available for hotspot mode".to_string())
}
