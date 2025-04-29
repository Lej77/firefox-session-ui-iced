//! A wizard modal window that helps the user select a Firefox profile.

use iced::widget::{column, container, row, text};
use iced::Element;
use iced_aw::selection_list;
use std::path::PathBuf;

use crate::host::FirefoxProfileInfo;

#[derive(Debug, Clone)]
pub struct State {
    profiles: Vec<FirefoxProfileInfo>,
    profile_names: Vec<String>,
    active: bool,
}
impl State {
    pub fn new() -> Self {
        let profiles = FirefoxProfileInfo::all_profiles();

        let profile_names = profiles
            .iter()
            .map(|info| {
                // Add padding in the beginning since the UI selection_list
                // doesn't seem to have any...
                "  ".to_string() + &*info.name()
            })
            .collect::<Vec<_>>();
        Self {
            profiles,
            profile_names,
            active: false,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Hide => {
                self.active = false;
            }
            Message::Show => {
                self.active = true;
            }
            Message::SelectedSessionFile(_) => {}
        }
    }

    pub fn view(&self) -> Option<iced::Element<'_, Message>> {
        if !self.active {
            return None;
        }

        let content = container(
            column![
                text("Select Firefox Session Data").size(24),
                row![column![
                    text("Firefox Profiles:"),
                    selection_list::SelectionList::new_with(
                        self.profile_names.as_slice(),
                        {
                            let profiles = self.profiles.clone();
                            move |ix, _name| {
                                let selected = &profiles[ix];
                                Message::SelectedSessionFile(selected.find_sessionstore_file())
                            }
                        },
                        16.0,
                        12.0,
                        // Use default style:
                        |theme, status| {
                            let style_class =
                                <iced::Theme as iced_aw::style::selection_list::Catalog>::default();
                            <iced::Theme as iced_aw::style::selection_list::Catalog>::style(
                                theme,
                                &style_class,
                                status,
                            )
                        },
                        None,
                        Default::default()
                    ),
                ]]
                .spacing(5)
            ]
            .spacing(20),
        )
        .width(600)
        .padding(10)
        .style(iced::widget::container::bordered_box);

        let content = container(content).padding(30);

        Some(Element::from(content))
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Hide,
    Show,
    SelectedSessionFile(PathBuf),
}
