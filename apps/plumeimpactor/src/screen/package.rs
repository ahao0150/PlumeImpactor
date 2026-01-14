use iced::widget::{
    button, checkbox, column, container, pick_list, row, scrollable, text, text_input,
};
use iced::{Alignment, Center, Element, Fill, Length, Task};
use plume_utils::{Package, PlistInfoTrait, SignerInstallMode, SignerMode, SignerOptions, t};

use crate::appearance;

#[derive(Debug, Clone)]
pub enum Message {
    UpdateCustomName(String),
    UpdateCustomIdentifier(String),
    UpdateCustomVersion(String),
    ToggleMinimumOsVersion(bool),
    ToggleFileSharing(bool),
    ToggleIpadFullscreen(bool),
    ToggleGameMode(bool),
    ToggleProMotion(bool),
    ToggleSingleProfile(bool),
    ToggleLiquidGlass(bool),
    UpdateSignerMode(SignerMode),
    UpdateInstallMode(SignerInstallMode),
    AddTweak,
    AddBundle,
    RemoveTweak(usize),
    Back,
    RequestInstallation,
}

#[derive(Debug, Clone)]
pub struct PackageScreen {
    pub selected_package: Option<Package>,
    pub options: SignerOptions,
}

impl PackageScreen {
    pub fn new(package: Option<Package>, options: SignerOptions) -> Self {
        Self {
            selected_package: package,
            options,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UpdateCustomName(name) => {
                let pkg_name = self
                    .selected_package
                    .as_ref()
                    .and_then(|p| p.get_name())
                    .unwrap_or_default();

                if name != pkg_name {
                    self.options.custom_name = Some(name);
                } else {
                    self.options.custom_name = None;
                }
                Task::none()
            }
            Message::UpdateCustomIdentifier(id) => {
                let pkg_id = self
                    .selected_package
                    .as_ref()
                    .and_then(|p| p.get_bundle_identifier())
                    .unwrap_or_default();

                if id != pkg_id {
                    self.options.custom_identifier = Some(id);
                } else {
                    self.options.custom_identifier = None;
                }
                Task::none()
            }
            Message::UpdateCustomVersion(ver) => {
                let pkg_ver = self
                    .selected_package
                    .as_ref()
                    .and_then(|p| p.get_version())
                    .unwrap_or_default();

                if ver != pkg_ver {
                    self.options.custom_version = Some(ver);
                } else {
                    self.options.custom_version = None;
                }
                Task::none()
            }
            Message::ToggleMinimumOsVersion(value) => {
                self.options.features.support_minimum_os_version = value;
                Task::none()
            }
            Message::ToggleFileSharing(value) => {
                self.options.features.support_file_sharing = value;
                Task::none()
            }
            Message::ToggleIpadFullscreen(value) => {
                self.options.features.support_ipad_fullscreen = value;
                Task::none()
            }
            Message::ToggleGameMode(value) => {
                self.options.features.support_game_mode = value;
                Task::none()
            }
            Message::ToggleProMotion(value) => {
                self.options.features.support_pro_motion = value;
                Task::none()
            }
            Message::ToggleSingleProfile(value) => {
                self.options.embedding.single_profile = value;
                Task::none()
            }
            Message::ToggleLiquidGlass(value) => {
                self.options.features.support_liquid_glass = value;
                Task::none()
            }
            Message::UpdateSignerMode(mode) => {
                self.options.mode = mode;
                Task::none()
            }
            Message::UpdateInstallMode(mode) => {
                self.options.install_mode = mode;
                Task::none()
            }
            Message::AddTweak => {
                let filter_name = t("tweak_files");
                let title = t("select_tweak_file");
                let path = rfd::FileDialog::new()
                    .add_filter(&filter_name, &["deb", "dylib"])
                    .set_title(&title)
                    .pick_file();

                if let Some(path) = path {
                    match &mut self.options.tweaks {
                        Some(vec) => vec.push(path),
                        None => self.options.tweaks = Some(vec![path]),
                    }
                }

                Task::none()
            }
            Message::AddBundle => {
                let title = t("select_bundle_folder");
                let path = rfd::FileDialog::new()
                    .set_title(&title)
                    .pick_folder();

                if let Some(path) = path {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        if ["framework", "bundle", "appex"].contains(&ext) {
                            match &mut self.options.tweaks {
                                Some(vec) => vec.push(path),
                                None => self.options.tweaks = Some(vec![path]),
                            }
                        }
                    }
                }

                Task::none()
            }
            Message::RemoveTweak(index) => {
                if let Some(tweaks) = &mut self.options.tweaks {
                    if index < tweaks.len() {
                        tweaks.remove(index);
                    }
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self, has_device: bool) -> Element<'_, Message> {
        let Some(pkg) = &self.selected_package else {
            return self.view_no_package();
        };

        let content = scrollable(
            row![
                self.view_package_info_column(pkg),
                self.view_options_column()
            ]
            .spacing(appearance::THEME_PADDING),
        );

        column![
            container(content).width(Fill).height(Fill),
            self.view_buttons(has_device)
        ]
        .spacing(appearance::THEME_PADDING)
        .into()
    }

    fn view_no_package(&self) -> Element<'_, Message> {
        column![
            text(t("no_package_selected")).size(32),
            text(t("go_back_select_file")).size(16),
        ]
        .spacing(appearance::THEME_PADDING)
        .align_x(Center)
        .into()
    }

    fn view_package_info_column(&self, pkg: &Package) -> Element<'_, Message> {
        let pkg_name = pkg.get_name().unwrap_or_default();
        let pkg_id = pkg.get_bundle_identifier().unwrap_or_default();
        let pkg_ver = pkg.get_version().unwrap_or_default();

        column![
            text(t("name")).size(12),
            text_input(
                &t("app_name_placeholder"),
                self.options.custom_name.as_ref().unwrap_or(&pkg_name)
            )
            .on_input(Message::UpdateCustomName)
            .padding(8),
            text(t("identifier")).size(12),
            text_input(
                &t("bundle_identifier"),
                self.options.custom_identifier.as_ref().unwrap_or(&pkg_id)
            )
            .on_input(Message::UpdateCustomIdentifier)
            .padding(8),
            text(t("version")).size(12),
            text_input(
                &t("version"),
                self.options.custom_version.as_ref().unwrap_or(&pkg_ver)
            )
            .on_input(Message::UpdateCustomVersion)
            .padding(8),
            text(t("tweaks")).size(12),
            self.view_tweaks(),
            row![
                button(text(t("add_tweak")).align_x(Center))
                    .on_press(Message::AddTweak)
                    .style(appearance::p_button),
                button(text(t("add_bundle")).align_x(Center))
                    .on_press(Message::AddBundle)
                    .style(appearance::p_button),
            ]
            .spacing(8),
        ]
        .spacing(8)
        .width(Fill)
        .into()
    }

    fn view_options_column(&self) -> Element<'_, Message> {
        column![
            text(t("general")).size(12),
            checkbox(self.options.features.support_minimum_os_version)
                .label(t("support_older_versions"))
                .on_toggle(Message::ToggleMinimumOsVersion),
            checkbox(self.options.features.support_file_sharing)
                .label(t("force_file_sharing"))
                .on_toggle(Message::ToggleFileSharing),
            checkbox(self.options.features.support_ipad_fullscreen)
                .label(t("force_ipad_fullscreen"))
                .on_toggle(Message::ToggleIpadFullscreen),
            checkbox(self.options.features.support_game_mode)
                .label(t("force_game_mode"))
                .on_toggle(Message::ToggleGameMode),
            checkbox(self.options.features.support_pro_motion)
                .label(t("force_pro_motion"))
                .on_toggle(Message::ToggleProMotion),
            text(t("advanced")).size(12),
            checkbox(self.options.embedding.single_profile)
                .label(t("only_register_main_bundle"))
                .on_toggle(Message::ToggleSingleProfile),
            checkbox(self.options.features.support_liquid_glass)
                .label(t("force_liquid_glass"))
                .on_toggle(Message::ToggleLiquidGlass),
            text(t("mode")).size(12),
            pick_list(
                &[SignerInstallMode::Install, SignerInstallMode::Export][..],
                Some(self.options.install_mode),
                Message::UpdateInstallMode
            )
            .style(appearance::s_pick_list)
            .placeholder(&t("select_mode")),
            text(t("signing_method")).size(12),
            pick_list(
                &[SignerMode::Pem, SignerMode::Adhoc, SignerMode::None][..],
                Some(self.options.mode),
                Message::UpdateSignerMode
            )
            .style(appearance::s_pick_list)
            .placeholder(&t("select_signing_method")),
        ]
        .spacing(8)
        .width(Fill)
        .into()
    }

    fn view_buttons(&self, has_device: bool) -> Element<'_, Message> {
        let (button_enabled, button_label) = match self.options.install_mode {
            SignerInstallMode::Install => (has_device, t("install")),
            SignerInstallMode::Export => (true, t("export")),
        };

        container(
            row![
                button(text(t("back")).align_x(Center))
                    .on_press(Message::Back)
                    .style(appearance::s_button)
                    .width(Fill),
                button(text(button_label).align_x(Center))
                    .on_press_maybe(button_enabled.then_some(Message::RequestInstallation))
                    .style(appearance::p_button)
                    .width(Fill),
            ]
            .spacing(appearance::THEME_PADDING),
        )
        .width(Fill)
        .into()
    }

    fn view_tweaks(&self) -> Element<'_, Message> {
        let tweaks = self.options.tweaks.as_ref();

        if let Some(tweaks) = tweaks {
            if tweaks.is_empty() {
                return text(t("no_tweaks_added")).size(12).into();
            }

            let mut tweak_list = column![].spacing(4);

            for (i, tweak) in tweaks.iter().enumerate() {
                let tweak_row = row![
                    text(tweak.file_name().and_then(|n| n.to_str()).unwrap_or("???"))
                        .size(12)
                        .width(Fill),
                    button(text(t("remove")).align_x(Center))
                        .on_press(Message::RemoveTweak(i))
                        .style(appearance::p_button)
                        .padding(6)
                ]
                .spacing(8)
                .align_y(Alignment::Center);

                tweak_list = tweak_list.push(tweak_row);
            }

            scrollable(tweak_list).height(Length::Fixed(100.0)).into()
        } else {
            text(t("no_tweaks_added")).size(12).into()
        }
    }
}
