use wxdragon::prelude::*;

#[derive(Clone)]
pub struct LoginDialog {
    pub dialog: Dialog,
    pub email_field: TextCtrl,
    pub password_field: TextCtrl,
    pub cancel_button: Button,
    pub next_button: Button,
}

pub fn create_login_dialog(parent: &Window) -> LoginDialog {
    let dialog = Dialog::builder(parent, "Sign in with your Apple ID")
        .with_style(DialogStyle::DefaultDialogStyle)
        .build();

    let sizer = BoxSizer::builder(Orientation::Vertical).build();

    sizer.add_spacer(16);

    let email_label = StaticText::builder(&dialog)
        .with_label("Email:")
        .build();
    let email_field = TextCtrl::builder(&dialog).build();
    sizer.add(&email_label, 0, SizerFlag::All, 8);
    sizer.add(&email_field, 0, SizerFlag::Expand | SizerFlag::All, 8);

    let password_label = StaticText::builder(&dialog)
        .with_label("Password:")
        .build();
    let password_field = TextCtrl::builder(&dialog)
        .with_style(TextCtrlStyle::Password)
        .build();
    sizer.add(&password_label, 0, SizerFlag::All, 8);
    sizer.add(&password_field, 0, SizerFlag::Expand | SizerFlag::All, 8);

    let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    let cancel_button = Button::builder(&dialog)
        .with_label("Cancel")
        .build();
    let next_button = Button::builder(&dialog)
        .with_label("Next")
        .build();
    button_sizer.add(&cancel_button, 0, SizerFlag::All, 8);
    button_sizer.add_spacer(16);
    button_sizer.add(&next_button, 0, SizerFlag::All, 8);

    sizer.add_spacer(16);
    sizer.add_sizer(&button_sizer, 0, SizerFlag::AlignRight | SizerFlag::All, 8);

    dialog.set_sizer(sizer, true);

    LoginDialog {
        dialog,
        email_field,
        password_field,
        cancel_button,
        next_button,
    }
}

impl LoginDialog {
    pub fn get_email(&self) -> String {
        self.email_field.get_value().to_string()
    }

    pub fn get_password(&self) -> String {
        self.password_field.get_value().to_string()
    }
    
    pub fn clear_fields(&self) {
        self.email_field.set_value("");
        self.password_field.set_value("");
    }
    
    pub fn show_modal(&self) {
        self.dialog.show_modal();
    }
    
    pub fn hide(&self) {
        self.dialog.end_modal(0);
    }
    
    pub fn set_cancel_handler(&self, on_cancel: impl Fn() + 'static) {
        self.cancel_button.on_click(move |_evt| {
            on_cancel();
        });
    }
    
    pub fn set_next_handler(&self, on_next: impl Fn() + 'static) {
        self.next_button.on_click(move |_evt| {
            on_next();
        });
    }
}

pub fn create_single_field_dialog(parent: &Window, title: &str, label: &str) -> Result<String, String> {
    let dialog = Dialog::builder(parent, title)
        .with_style(DialogStyle::DefaultDialogStyle)
        .build();

    let sizer = BoxSizer::builder(Orientation::Vertical).build();
    sizer.add_spacer(16);

    let field_label = StaticText::builder(&dialog)
        .with_label(label)
        .build();
    let text_field = TextCtrl::builder(&dialog).build();
    sizer.add(&field_label, 0, SizerFlag::All, 8);
    sizer.add(&text_field, 0, SizerFlag::Expand | SizerFlag::All, 8);

    dialog.set_sizer(sizer, true);

    dialog.show_modal();
    let value = text_field.get_value().to_string();
    dialog.destroy();
    Ok(value)
}
