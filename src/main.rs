#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use iced::widget::{
    button, center, checkbox, column, container, horizontal_space, mouse_area, opaque, pane_grid,
    pick_list, row, scrollable, stack, text, text_editor, text_input, tooltip, vertical_slider,
};
use iced::{time, Alignment, Color, Element, Length, Subscription, Task, Theme};

mod host;
mod wizard;

pub fn main() -> iced::Result {
    iced::application(
        SessionDataUtility::title,
        SessionDataUtility::update,
        SessionDataUtility::view,
    )
    .theme(SessionDataUtility::theme)
    .subscription(SessionDataUtility::subscription)
    .run_with(SessionDataUtility::start)
}

/// From <https://github.com/iced-rs/iced/blob/a687a837653a576cb0599f7bc8ecd9c6054213a9/examples/modal/src/main.rs>
fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: Option<impl Into<Element<'a, Message>>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    if let Some(content) = content {
        stack![
            base.into(),
            opaque(
                mouse_area(center(opaque(content)).style(|_theme| {
                    container::Style {
                        background: Some(
                            Color {
                                a: 0.8,
                                ..Color::BLACK
                            }
                            .into(),
                        ),
                        ..container::Style::default()
                    }
                }))
                .on_press(on_blur)
            )
        ]
        .into()
    } else {
        base.into()
    }
}

/// Slider style that doesn't highlight one side of the slider's cursor.
fn no_highlight_slider_style<Theme: iced::widget::slider::Catalog>(
    theme: &Theme,
    status: iced::widget::slider::Status,
) -> iced::widget::slider::Style {
    let theme_class = Theme::default();
    let mut style = Theme::style(theme, &theme_class, status);
    style.rail.backgrounds.0 = style.rail.backgrounds.1;
    style
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SidebarPane {
    Sidebar,
    MainContent,
}

#[derive(Debug, Clone)]
enum Message {
    /// Triggered after we detect that the system theme changed.
    SetSystemThemeMode(Theme),
    SetSplit(pane_grid::ResizeEvent),
    FirefoxProfileWizard(wizard::Message),
    /// User interactions with the preview text editor.
    Preview(text_editor::Action),
    SetPreview(String),
    SetInputPath(String),
    BrowseInputPath,
    LoadInputData,
    UpdateLoadedData(host::FileInfo),
    ParsedTabGroups(host::AllTabGroups),
    ChangeTabGroupSelection {
        open: bool,
        index: u32,
        select: bool,
    },
    SetStatus(String),
    SetSavePath(String),
    BrowseSavePath,
    SetOverwrite(bool),
    SetCreateFolder(bool),
    SetOutputFormat(&'static str),
    CopyLinksToClipboard,
    SaveLinksToFile,
    Nothing,
}

#[derive(Debug)]
struct SessionDataUtility {
    theme: Theme,
    preview: text_editor::Content,
    /// Guess of which line the `preview` is scrolled to (at the top of view).
    /// Incorrect when there is word wrapping.
    preview_scroll: u32,
    input_path: String,
    loaded_data: Option<host::FileInfo>,
    selected_tab_groups: host::GenerateOptions,
    save_path: String,
    output_options: host::OutputOptions,
    tab_groups: host::AllTabGroups,
    status: String,
    split_divider: pane_grid::State<SidebarPane>,
    firefox_profile_wizard: wizard::State,
}
impl SessionDataUtility {
    fn regenerate_preview_task(&mut self) -> Task<Message> {
        let Some(data) = self.loaded_data.clone() else {
            return Task::none();
        };
        let options = self.selected_tab_groups.clone();
        self.status = "Generating preview".to_string();
        Task::perform(
            async move { data.to_text_links(options).await },
            |result| match result {
                Ok(preview) => Message::SetPreview(preview),
                Err(e) => Message::SetStatus(format!("Failed to generate preview: {e}")),
            },
        )
    }
    fn tab_group_view<'a>(
        &'a self,
        index: usize,
        group: &'a host::TabGroup,
        open_window: bool,
    ) -> Element<'a, Message> {
        let index = index as u32;
        let is_selected = if open_window {
            &self.selected_tab_groups.open_group_indexes
        } else {
            &self.selected_tab_groups.closed_group_indexes
        }
        .as_ref()
        .is_some_and(|indexes| indexes.contains(&index));

        button(group.name.as_str())
            .width(Length::Fill)
            .style(if is_selected {
                iced::widget::button::success
            } else {
                iced::widget::button::secondary
            })
            .on_press(Message::ChangeTabGroupSelection {
                open: open_window,
                index,
                select: !is_selected,
            })
            .into()
    }
}
impl SessionDataUtility {
    fn new() -> Self {
        Self {
            theme: system_theme_mode(),
            split_divider: {
                let (mut panes, first_pane_id) = pane_grid::State::new(SidebarPane::Sidebar);
                let (_, split) = panes
                    .split(
                        pane_grid::Axis::Vertical,
                        first_pane_id,
                        SidebarPane::MainContent,
                    )
                    .expect("Splitting panel should succeed");
                panes.resize(split, 0.2);
                panes
            },
            preview: text_editor::Content::new(),
            preview_scroll: 0,
            input_path: "".to_string(),
            loaded_data: None,
            selected_tab_groups: host::GenerateOptions::default(),
            // TODO: more robust finding of downloads folder.
            save_path: std::env::var("USERPROFILE")
                .map(|home| home + r"\Downloads\firefox-links")
                .unwrap_or_default(),
            output_options: Default::default(),
            #[cfg(debug_assertions)]
            tab_groups: host::AllTabGroups {
                open: vec![
                    host::TabGroup {
                        index: 0,
                        name: "Window 1".into(),
                    },
                    host::TabGroup {
                        index: 1,
                        name: "Window 2".into(),
                    },
                ],
                closed: vec![host::TabGroup {
                    index: 3,
                    name: "Closed window 1".into(),
                }],
            },
            #[cfg(not(debug_assertions))]
            tab_groups: Default::default(),
            status: "".to_string(),
            firefox_profile_wizard: wizard::State::new(),
        }
    }
    fn start() -> (Self, Task<Message>) {
        (Self::new(), Task::none())
    }

    fn title(&self) -> String {
        "Firefox Session Data Utility".into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetSystemThemeMode(theme) => {
                self.theme = theme;
                Task::none()
            }
            Message::SetSplit(event) => {
                self.split_divider.resize(event.split, event.ratio);
                Task::none()
            }
            Message::FirefoxProfileWizard(wizard::Message::SelectedSessionFile(selected)) => {
                self.firefox_profile_wizard.update(wizard::Message::Hide);
                self.input_path = selected.to_string_lossy().into_owned();
                self.update(Message::LoadInputData)
            }
            Message::FirefoxProfileWizard(msg) => {
                self.firefox_profile_wizard.update(msg);
                Task::none()
            }
            Message::SetPreview(preview) => {
                self.status = "Successfully loaded session data".to_string();
                self.preview = text_editor::Content::with_text(&preview);
                self.preview_scroll = 0;
                Task::none()
            }
            Message::Preview(action) => {
                if let text_editor::Action::Scroll { lines } = &action {
                    self.preview_scroll = self
                        .preview_scroll
                        .saturating_add_signed(*lines)
                        .min(self.preview.line_count() as u32);
                }
                if cfg!(debug_assertions) || !action.is_edit() {
                    let (prev_line, prev_col) = self.preview.cursor_position();

                    // Update the state of the multi-line text editor:
                    self.preview.perform(action);

                    let (new_line, new_col) = self.preview.cursor_position();
                    if prev_line != new_line || prev_col != new_col {
                        // Also need to update tracked scroll when moving cursor outside
                        // of the visible area.
                        self.preview_scroll = new_line as u32;
                    }
                }
                Task::none()
            }
            Message::SetInputPath(v) => {
                self.input_path = v;
                Task::none()
            }
            Message::BrowseInputPath => Task::perform(host::prompt_load_file(), |path| {
                path.as_ref()
                    .and_then(|v| v.to_str())
                    .map(|v| v.to_string())
                    .map(Message::SetInputPath)
                    .unwrap_or(Message::Nothing)
            }),
            Message::LoadInputData => {
                let mut data = host::FileInfo::new(PathBuf::from(self.input_path.clone()));
                self.loaded_data = Some(data.clone());
                self.selected_tab_groups.open_group_indexes = None;
                self.selected_tab_groups.closed_group_indexes = Some(Vec::new());
                self.status = "Reading input file".to_string();
                // FIXME: handle cancellation
                Task::perform(
                    async move { data.load_data().await.map(|_| data) },
                    |result| match result {
                        Ok(data) => Message::UpdateLoadedData(data),
                        Err(e) => Message::SetStatus(format!("Failed to read file: {e}")),
                    },
                )
            }
            Message::UpdateLoadedData(mut data) => {
                self.loaded_data = Some(data.clone());

                match &data.data {
                    Some(host::FileData::Compressed { .. }) => {
                        self.status = "Decompressing data".to_string();
                        Task::perform(
                            async move { data.decompress_data().await.map(|_| data) },
                            |result| match result {
                                Ok(data) => Message::UpdateLoadedData(data),
                                Err(e) => {
                                    Message::SetStatus(format!("Failed to decompress data: {e}"))
                                }
                            },
                        )
                    }
                    Some(host::FileData::Uncompressed { .. }) => {
                        self.status = "Parsing session data".to_string();
                        Task::perform(
                            async move { data.parse_session_data().await.map(|_| data) },
                            |result| match result {
                                Ok(data) => Message::UpdateLoadedData(data),
                                Err(e) => {
                                    Message::SetStatus(format!("Failed to parse session data: {e}"))
                                }
                            },
                        )
                    }
                    Some(host::FileData::Parsed { .. }) => Task::perform(
                        async move { data.get_groups_from_session(true).await },
                        |result| match result {
                            Ok(all_groups) => Message::ParsedTabGroups(all_groups),
                            Err(e) => Message::SetStatus(format!(
                                "Failed to list windows in session: {e}"
                            )),
                        },
                    ),
                    None => unreachable!("Always have data when updating file info"),
                }
            }
            Message::ParsedTabGroups(groups) => {
                self.tab_groups = groups;
                self.regenerate_preview_task()
            }
            Message::ChangeTabGroupSelection {
                index,
                open,
                select,
            } => {
                let (mut indexes, mut other) = (
                    &mut self.selected_tab_groups.open_group_indexes,
                    &mut self.selected_tab_groups.closed_group_indexes,
                );
                if !open {
                    std::mem::swap(&mut indexes, &mut other);
                }
                if select {
                    let indexes = indexes.get_or_insert_with(Vec::new);
                    other.get_or_insert_with(Vec::new);
                    if !indexes.contains(&index) {
                        indexes.push(index);
                        self.regenerate_preview_task()
                    } else {
                        Task::none()
                    }
                } else if let Some(indexes) = indexes {
                    let len = indexes.len();
                    indexes.retain(|v| *v != index);
                    if indexes.len() == len {
                        Task::none()
                    } else {
                        if self.selected_tab_groups.selected_groups() == 0 {
                            // Nothing selected => select all open windows:
                            self.selected_tab_groups.open_group_indexes = None;
                            self.selected_tab_groups
                                .closed_group_indexes
                                .get_or_insert_with(Vec::new);
                        }
                        self.regenerate_preview_task()
                    }
                } else {
                    Task::none()
                }
            }
            Message::SetStatus(status) => {
                self.status = status;
                Task::none()
            }
            Message::SetSavePath(v) => {
                self.save_path = v;
                Task::none()
            }
            Message::BrowseSavePath => Task::perform(host::prompt_save_file(), |path| {
                path.map(Message::SetSavePath).unwrap_or(Message::Nothing)
            }),
            Message::SetOverwrite(v) => {
                self.output_options.overwrite = v;
                Task::none()
            }
            Message::SetCreateFolder(v) => {
                self.output_options.create_folder = v;
                Task::none()
            }
            Message::SetOutputFormat(v) => {
                self.output_options.format = *host::FormatInfo::all()
                    .iter()
                    .find(|f| f.as_str() == v)
                    .expect("Invalid output format");
                Task::none()
            }
            Message::CopyLinksToClipboard => Task::batch([
                iced::clipboard::write(self.preview.text()),
                iced::clipboard::write_primary(self.preview.text()),
            ]),
            Message::SaveLinksToFile => {
                let Some(data) = self.loaded_data.clone() else {
                    return Task::none();
                };
                let save_path = PathBuf::from(self.save_path.as_str());
                let selected = self.selected_tab_groups.clone();
                let output_options = self.output_options.clone();

                self.status = "Saving links to file".to_string();
                Task::perform(
                    async move { data.save_links(save_path, selected, output_options).await },
                    |result| match result {
                        Ok(()) => {
                            Message::SetStatus("Successfully saved links to a file".to_string())
                        }
                        Err(e) => Message::SetStatus(format!("Failed to save links to file: {e}")),
                    },
                )
            }
            Message::Nothing => Task::none(),
        }
    }

    fn view_sidebar(&self) -> Element<Message> {
        container(scrollable(
            column(
                self.tab_groups
                    .open
                    .iter()
                    .enumerate()
                    .map(|(index, group)| self.tab_group_view(index, group, true))
                    .chain(
                        [
                            Element::from(text("")),
                            Element::from(text("Closed Windows:")),
                        ]
                        .into_iter()
                        .filter(|_| !self.tab_groups.closed.is_empty()),
                    )
                    .chain(
                        self.tab_groups
                            .closed
                            .iter()
                            .enumerate()
                            .map(|(index, group)| self.tab_group_view(index, group, false)),
                    ),
            )
            .spacing(10)
            .padding(16)
            .width(Length::Fill)
            .align_x(Alignment::Start),
        ))
        .style(iced::widget::container::bordered_box)
        .height(Length::Fill)
        .into()
    }

    fn view_main_content(&self) -> Element<Message> {
        column![
            row![
                text("Path to sessionstore file: "),
                text_input("", self.input_path.as_str()).on_input(Message::SetInputPath),
                button("Wizard").on_press(Message::FirefoxProfileWizard(wizard::Message::Show)),
                button("Browse").on_press(Message::BrowseInputPath),
            ]
            .spacing(5)
            .align_y(Alignment::Center),
            row![
                text("Current data was loaded from: "),
                text_input(
                    "",
                    self.loaded_data
                        .as_ref()
                        .and_then(|info| info.file_path.to_str())
                        .unwrap_or("")
                )
                .on_input(|_| Message::Nothing),
                button("Load new data").on_press(Message::LoadInputData)
            ]
            .spacing(5)
            .align_y(Alignment::Center),
            column([
                text("Tabs as links: ").into(),
                #[cfg(debug_assertions)]
                {
                    text(format!(
                        "Scroll: {}/{}",
                        self.preview_scroll,
                        self.preview.line_count()
                    ))
                    .into()
                },
                row![
                    text_editor(&self.preview)
                        .on_action(Message::Preview)
                        .height(Length::Fill),
                    {
                        let max = self.preview.line_count() as u32;
                        vertical_slider(
                            0..=max,
                            max.saturating_sub(self.preview_scroll),
                            move |new| {
                                let new = max.saturating_sub(new);
                                Message::Preview(text_editor::Action::Scroll {
                                    lines: i32::try_from(
                                        i64::from(new) - i64::from(self.preview_scroll),
                                    )
                                    .expect("too large scroll distance when using slider"),
                                })
                            },
                        )
                    }
                    .style(no_highlight_slider_style)
                ]
                .width(Length::Fill)
                .into(),
            ])
            .height(Length::Fill),
            row![
                text("File path to write links to: "),
                text_input("", self.save_path.as_str()).on_input(Message::SetSavePath),
                button("Browse").on_press(Message::BrowseSavePath)
            ]
            .spacing(5)
            .align_y(Alignment::Center),
            row![
                checkbox(
                    "Create folder if it doesn't exist",
                    self.output_options.create_folder
                )
                .on_toggle(Message::SetCreateFolder),
                checkbox(
                    "Overwrite file if it already exists",
                    self.output_options.overwrite
                )
                .on_toggle(Message::SetOverwrite),
            ]
            .spacing(10),
            row![
                button("Copy links to clipboard").on_press(Message::CopyLinksToClipboard),
                horizontal_space(),
                tooltip(
                    pick_list(
                        host::FormatInfo::all()
                            .iter()
                            .map(|v| v.as_str())
                            .collect::<Vec<_>>(),
                        Some(self.output_options.format.as_str()),
                        Message::SetOutputFormat
                    ),
                    container(text(self.output_options.format.to_string()))
                        .padding(8)
                        .style(iced::widget::container::bordered_box),
                    tooltip::Position::Top
                ),
                button("Save links to file").on_press(Message::SaveLinksToFile),
            ]
            .spacing(5),
            row![
                text("Status: "),
                text_input("", self.status.as_str()).on_input(|_| Message::Nothing)
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(20)
        .padding(10)
        .align_x(Alignment::Start)
        .width(Length::Fill)
        .into()
    }

    fn view(&self) -> Element<Message> {
        let content = pane_grid(&self.split_divider, |_pane, state, _is_maximized| {
            pane_grid::Content::new(match state {
                SidebarPane::Sidebar => self.view_sidebar(),
                SidebarPane::MainContent => self.view_main_content(),
            })
        })
        .on_resize(10, Message::SetSplit);

        modal(
            content,
            self.firefox_profile_wizard
                .view()
                .map(|ele| ele.map(Message::FirefoxProfileWizard)),
            Message::FirefoxProfileWizard(wizard::Message::Hide),
        )
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        // "time" requires one of the following iced features to be enabled:
        // tokio, async-std, or smol
        time::every(time::Duration::from_secs(10))
            .map(|_| Message::SetSystemThemeMode(system_theme_mode()))
    }
}

/// For more info see: https://github.com/iced-rs/iced/issues/1022
fn system_theme_mode() -> Theme {
    match dark_light::detect() {
        Ok(dark_light::Mode::Light) | Ok(dark_light::Mode::Unspecified) => Theme::Light,
        Ok(dark_light::Mode::Dark) => Theme::Dark,
        Err(_e) => {
            #[cfg(debug_assertions)]
            {
                eprintln!("Failed to detect system theme: {_e}");
            }
            Theme::Light
        }
    }
}
