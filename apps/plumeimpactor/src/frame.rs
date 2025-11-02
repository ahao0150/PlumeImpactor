use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::{env, ptr, thread};

use grand_slam::AnisetteConfiguration;
use grand_slam::auth::Account;
use grand_slam::developer::DeveloperSession;
use wxdragon::prelude::*;

use futures::StreamExt;
use idevice::usbmuxd::{UsbmuxdConnection, UsbmuxdListenEvent};
use tokio::runtime::Builder;
use tokio::sync::mpsc;

use types::{Device, Package};
use crate::pages::login::{LoginDialog, create_single_field_dialog};
use crate::pages::{DefaultPage, InstallPage, create_default_page, create_install_page, create_login_dialog};
use crate::APP_NAME;
use crate::handlers::{PlumeFrameMessage, PlumeFrameMessageHandler};

pub struct PlumeFrame {
    pub frame: Frame,
    pub default_page: DefaultPage,
    pub install_page: InstallPage,
    pub settings_button: Button,
    pub usbmuxd_picker: Choice,
    
    pub apple_id_picker: Choice,
    pub apple_id_button: Button,
    pub login_dialog: LoginDialog,
}

impl PlumeFrame {
    pub fn new() -> Self {
        let frame = Frame::builder()
            .with_title(APP_NAME)
            .with_size(Size::new(530, 410))
            .with_style(FrameStyle::CloseBox | FrameStyle::MinimizeBox)
            .build();
        
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        let top_panel = Panel::builder(&frame).build();
        let top_row = BoxSizer::builder(Orientation::Horizontal).build();

        let device_picker = Choice::builder(&top_panel)
            .build();

        let apple_id_picker = Choice::builder(&top_panel)
            .build();
        
        let apple_id_button = Button::builder(&top_panel)
            .with_label("+")
            .build();

        let settings_button = Button::builder(&top_panel)
            .with_label("Settings")
            .build();

        top_row.add(&device_picker, 1, SizerFlag::Expand | SizerFlag::All, 0);
        top_row.add_spacer(8);
        top_row.add(&apple_id_picker, 1, SizerFlag::Expand | SizerFlag::All, 0);
        top_row.add_spacer(8);
        top_row.add(&apple_id_button, 0, SizerFlag::All, 0);
        top_row.add_spacer(8);
        top_row.add(&settings_button, 0, SizerFlag::All, 0);

        top_panel.set_sizer(top_row, true);

        let default_page = create_default_page(&frame);
        let install_page = create_install_page(&frame);
        sizer.add(&top_panel, 0, SizerFlag::Expand | SizerFlag::All, 8);
        sizer.add(&default_page.panel, 1, SizerFlag::Expand | SizerFlag::All, 0);
        sizer.add(&install_page.panel, 1, SizerFlag::Expand | SizerFlag::All, 0);
        frame.set_sizer(sizer, true);
        install_page.panel.hide();

        let mut s = Self {
            frame: frame.clone(),
            default_page,
            install_page,
            settings_button,
            usbmuxd_picker: device_picker,
            apple_id_picker,
            apple_id_button,
            login_dialog: create_login_dialog(&frame),
        };

        s.setup_event_handlers();
        
        s
    }

    pub fn show(&mut self) {
        self.frame.show(true);
        self.frame.centre();
        self.frame.set_extra_style(ExtraWindowStyle::ProcessIdle);
    }

    fn setup_event_handlers(&mut self) {
        let (sender, receiver) = mpsc::unbounded_channel::<PlumeFrameMessage>();
        
        let message_handler = Rc::new(
            RefCell::new(PlumeFrameMessageHandler::new(
                receiver,
                unsafe { ptr::read(self) },
            ))
        );
        
        let handler_for_idle = message_handler.clone();
        self.frame.on_idle(move |event_data| {
            if let WindowEventData::Idle(event) = event_data {
                event.request_more(handler_for_idle.borrow_mut().process_messages());
            }
        });

        // --- Usbmuxd Listener ---
        
        thread::spawn({
            let sender = sender.clone();
            move || {
                let rt = Builder::new_current_thread().enable_io().build().unwrap();

                rt.block_on(async move {
                    let mut muxer = match UsbmuxdConnection::default().await {
                        Ok(muxer) => muxer,
                        Err(e) => {
                            sender.send(PlumeFrameMessage::Error(format!("Failed to connect to usbmuxd: {}", e))).ok();
                            return;
                        }
                    };

                    match muxer.get_devices().await {
                        Ok(devices) => {
                            for dev in devices {
                                sender.send(PlumeFrameMessage::DeviceConnected(Device::new(dev).await)).ok();
                            }
                        }
                        Err(e) => {
                            sender.send(PlumeFrameMessage::Error(format!("Failed to get initial device list: {}", e))).ok();
                        }
                    }

                    let mut stream = match muxer.listen().await {
                        Ok(stream) => stream,
                        Err(e) => {
                            sender.send(PlumeFrameMessage::Error(format!("Failed to listen for events: {}", e))).ok();
                            return;
                        }
                    };

                    while let Some(event) = stream.next().await {
                        let msg = match event {
                            Ok(dev_event) => match dev_event {
                                UsbmuxdListenEvent::Connected(dev) => {
                                    PlumeFrameMessage::DeviceConnected(Device::new(dev).await)
                                }
                                UsbmuxdListenEvent::Disconnected(device_id) => {
                                    PlumeFrameMessage::DeviceDisconnected(device_id)
                                }
                            },
                            Err(e) => {
                                PlumeFrameMessage::Error(format!("Failed to listen for events: {}", e))
                            }
                        };
                        if sender.send(msg).is_err() {
                            break;
                        }
                    }
                });
            }
        });

        // --- GUI Handlers ---

        let handler_for_choice = message_handler.clone();
        let picker_clone = self.usbmuxd_picker.clone();
        self.usbmuxd_picker.on_selection_changed(move |_event_data| {
            let mut handler = handler_for_choice.borrow_mut();
            
            if let Some(index) = picker_clone.get_selection() {
                if let Some(selected_item) = handler.usbmuxd_device_list.get(index as usize) {
                    handler.usbmuxd_selected_device_id = Some(selected_item.usbmuxd_device.device_id.to_string());
                }
            } else {
                handler.usbmuxd_selected_device_id = None;
            }
        });
        
        self.settings_button.on_click(|_| {
            println!("Settings");
        });

        let login_dialog_rc = Rc::new(self.login_dialog.clone());
        self.apple_id_button.on_click({
            let login_dialog = login_dialog_rc.clone();
            move |_| {
                login_dialog.show_modal();
            }
        });
        
        self.login_dialog.set_cancel_handler({
            let login_dialog = login_dialog_rc.clone();
            move || {
                login_dialog.clear_fields();
                login_dialog.hide();
            }
        });
        
        let frame_clone = self.frame.clone();
        self.login_dialog.set_next_handler({
            let login_dialog = login_dialog_rc.clone();
            move || {
                let email = login_dialog.get_email();
                let password = login_dialog.get_password();

                login_dialog.clear_fields();
                login_dialog.hide();


                println!("Email: {}, Password: {}", email, password);

                let anisette_config = AnisetteConfiguration::default()
                    .set_configuration_path(env::temp_dir());

                thread::spawn({
                    let email = email.clone();
                    let password = password.clone();
                    let anisette_config = anisette_config;
                    move || {
                        let rt = Builder::new_current_thread().enable_all().build().unwrap();
                        rt.block_on(async {

                            let get_2fa_code = || {

                                Err("2FA code request not implemented in background thread".into())
                            };

                            let a = Account::login(
                                || Ok((email.clone(), password.clone())),
                                get_2fa_code,
                                anisette_config,
                            ).await.unwrap();
                            
                            let session = DeveloperSession::with(a);
                            let a = session.qh_list_teams().await.unwrap();
                            println!("{:#?}", env::temp_dir());
                            println!("{:#?}", a);

                        });
                    }
                });
            }
        });
        
        
        
        let handler_for_import = self.frame.clone();
        let sender_for_file = sender.clone();
        let sender_for_dialog = sender.clone();
        self.default_page.set_file_handlers(
            move |file_path| {
                match Package::new(PathBuf::from(file_path)) {
                    Ok(package) => {
                        sender_for_file.send(PlumeFrameMessage::PackageSelected(package)).ok();
                    }
                    Err(e) => {
                        sender_for_file.send(PlumeFrameMessage::Error(format!("Failed to open package: {}", e))).ok();
                    }
                }
            },
            move || {
                let dialog = FileDialog::builder(&handler_for_import)
                    .with_message("Open IPA File")
                    .with_style(FileDialogStyle::default() | FileDialogStyle::Open)
                    .with_wildcard("IPA files (*.ipa;*.tipa)|*.ipa;*.tipa")
                    .build();

                if dialog.show_modal() != ID_OK {
                    return;
                }
                if let Some(file_path) = dialog.get_path() {
                    match Package::new(PathBuf::from(file_path)) {
                        Ok(package) => {
                            sender_for_dialog.send(PlumeFrameMessage::PackageSelected(package)).ok();
                        }
                        Err(e) => {
                            sender_for_dialog.send(PlumeFrameMessage::Error(format!("Failed to open package: {}", e))).ok();
                        }
                    }
                }
            },
        );

        self.install_page.set_cancel_handler(move || {
            sender.send(PlumeFrameMessage::PackageDeselected).ok();
        });
    }
}
