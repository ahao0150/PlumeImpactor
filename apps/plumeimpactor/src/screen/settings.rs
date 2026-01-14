use iced::widget::{button, column, container, row, text, pick_list};
use iced::{Alignment, Center, Element, Fill, Length, Task};
use iced_aw::SelectionList;
use plume_store::AccountStore;
use plume_utils::{Language, t};

use crate::appearance;

#[derive(Debug, Clone)]
pub enum Message {
    ShowLogin,
    SelectAccount(usize),
    RemoveAccount(usize),
    ExportP12,
    LanguageChanged(Language),
}

#[derive(Debug)]
pub struct SettingsScreen {
    pub account_store: Option<AccountStore>,
    selected_language: Language,
}

impl SettingsScreen {
    pub fn new(account_store: Option<AccountStore>) -> Self {
        let current_language = plume_utils::get_language();
        Self {
            account_store,
            selected_language: current_language,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectAccount(index) => {
                if let Some(store) = &mut self.account_store {
                    let mut emails: Vec<_> = store.accounts().keys().cloned().collect();
                    emails.sort();
                    if let Some(email) = emails.get(index) {
                        let _ = store.account_select_sync(email);
                    }
                }
                Task::none()
            }
            Message::RemoveAccount(index) => {
                if let Some(store) = &mut self.account_store {
                    let mut emails: Vec<_> = store.accounts().keys().cloned().collect();
                    emails.sort();
                    if let Some(email) = emails.get(index) {
                        let _ = store.accounts_remove_sync(email);
                    }
                }
                Task::none()
            }
            Message::ExportP12 => {
                if let Some(account) = self
                    .account_store
                    .as_ref()
                    .and_then(|s| s.selected_account().cloned())
                {
                    std::thread::spawn(move || {
                        let rt = tokio::runtime::Builder::new_current_thread()
                            .enable_all()
                            .build()
                            .unwrap();

                        let _ = rt.block_on(async move {
                            crate::subscriptions::export_certificate(account).await
                        });
                    });
                }
                Task::none()
            }
            Message::LanguageChanged(language) => {
                self.selected_language = language;
                // Note: Language change will be handled by parent screen
                // We return None here and let parent handle the actual language change
                Task::none()
            }
            Message::ShowLogin => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let Some(store) = &self.account_store else {
            return column![text(t("loading_accounts"))]
                .spacing(appearance::THEME_PADDING)
                .padding(appearance::THEME_PADDING)
                .into();
        };

        let mut accounts: Vec<_> = store.accounts().iter().collect();
        accounts.sort_by_key(|(email, _)| *email);

        let selected_index = store
            .selected_account()
            .and_then(|acc| accounts.iter().position(|(e, _)| *e == acc.email()));

        let mut content = column![].spacing(appearance::THEME_PADDING);

        // Language selector
        content = content.push(self.view_language_selector());

        // Accounts section
        content = content.push(text(t("accounts")));

        if !accounts.is_empty() {
            content = content.push(self.view_account_list(&accounts, selected_index));
        } else {
            content = content.push(text(t("no_accounts_added")));
        }

        content = content.push(self.view_account_buttons(selected_index));
        content.into()
    }

    fn view_language_selector(&self) -> Element<'_, Message> {
        let languages: Vec<Language> = vec![Language::English, Language::Chinese];
        let language_names: Vec<String> = languages.iter().map(|l| l.as_str().to_string()).collect();
        let selected_lang = self.selected_language.as_str().to_string();

        row![
            text(t("language")).width(Fill),
            pick_list(language_names, Some(selected_lang), |lang_str| {
                Message::LanguageChanged(Language::from_str(&lang_str))
            })
            .width(Length::Fixed(200.0))
        ]
        .spacing(appearance::THEME_PADDING)
        .align_y(Alignment::Center)
        .padding(appearance::THEME_PADDING)
        .into()
    }

    fn view_account_list(
        &self,
        accounts: &[(&String, &plume_store::GsaAccount)],
        selected_index: Option<usize>,
    ) -> Element<'_, Message> {
        let account_labels: &'static [String] = Box::leak(
            accounts
                .iter()
                .enumerate()
                .map(|(i, (_, account))| {
                    let name = if !account.first_name().is_empty() {
                        format!("{} ({})", account.first_name(), account.email())
                    } else {
                        account.email().to_string()
                    };
                    let marker = if Some(i) == selected_index {
                        " [✓] "
                    } else {
                        " [ ] "
                    };
                    format!("{}{}", marker, name)
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        );

        let selection_list = SelectionList::new_with(
            account_labels,
            |index, _| Message::SelectAccount(index),
            appearance::THEME_FONT_SIZE.into(),
            5.0,
            iced_aw::style::selection_list::primary,
            selected_index,
            appearance::p_font(),
        );

        container(selection_list)
            .height(Fill)
            .style(|theme: &iced::Theme| container::Style {
                border: iced::Border {
                    width: 1.0,
                    color: theme.palette().background.scale_alpha(0.5),
                    radius: appearance::THEME_CORNER_RADIUS.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn view_account_buttons(&self, selected_index: Option<usize>) -> Element<'_, Message> {
        let mut buttons = row![
            button(text(t("add_account")).align_x(Center))
                .on_press(Message::ShowLogin)
                .style(appearance::s_button)
        ]
        .spacing(appearance::THEME_PADDING);

        if let Some(index) = selected_index {
            buttons = buttons
                .push(
                    button(text(t("remove_selected")).align_x(Center))
                        .on_press(Message::RemoveAccount(index))
                        .style(appearance::s_button),
                )
                .push(
                    button(text(t("export_p12")).align_x(Center))
                        .on_press(Message::ExportP12)
                        .style(appearance::s_button),
                );
        }

        buttons.align_y(Alignment::Center).into()
    }
}
