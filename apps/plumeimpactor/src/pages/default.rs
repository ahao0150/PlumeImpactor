use wxdragon::prelude::*;

const INSTALLER_IMAGE_BYTES: &[u8] = include_bytes!("../../resources/install.png");
const INSTALLER_IMAGE_DIMENSIONS: u32 = 100;
const WELCOME_TEXT: &str = "Drop your .ipa here";

#[derive(Clone)]
pub struct DefaultPage {
    pub panel: Panel,
}

impl DefaultPage {
    fn is_allowed_file(path: &str) -> bool {
        path.ends_with(".ipa") || path.ends_with(".tipa")
    }

    // it seems that image is on top of the panel...
    pub fn set_file_handlers(&self, on_drop: impl Fn(String) + 'static, on_click: impl Fn() + 'static) {
        _ = FileDropTarget::builder(&self.panel)
            .with_on_drop_files(move |files, _, _| {
                if files.len() != 1 || !DefaultPage::is_allowed_file(&files[0]) {
                    return false;
                }
                on_drop(files[0].clone());
                true
            })
            .with_on_drag_over(move |_, _, _| DragResult::Move)
            .with_on_enter(move |_, _, _| DragResult::Move)
            .build();
            
        self.panel.on_mouse_left_down(move |_evt| {
            on_click();
        });
    }
    
    
}

pub fn create_default_page(frame: &Frame) -> DefaultPage {
    let panel = Panel::builder(frame).build();
    let sizer = BoxSizer::builder(Orientation::Vertical).build();

    sizer.add_stretch_spacer(1);
    
    if let Ok(img) = image::load_from_memory_with_format(INSTALLER_IMAGE_BYTES, image::ImageFormat::Png) {
        let resized = img.resize_exact(
            INSTALLER_IMAGE_DIMENSIONS,
            INSTALLER_IMAGE_DIMENSIONS,
            image::imageops::FilterType::Lanczos3,
        );
        let rgba = resized.to_rgba8();
        let bitmap = Bitmap::from_rgba(
            rgba.as_raw(),
            INSTALLER_IMAGE_DIMENSIONS,
            INSTALLER_IMAGE_DIMENSIONS,
        );
        let static_bitmap = StaticBitmap::builder(&panel)
            .with_bitmap(bitmap)
            .with_size(Size::new(
                INSTALLER_IMAGE_DIMENSIONS as i32,
                INSTALLER_IMAGE_DIMENSIONS as i32,
            ))
            .build();
        sizer.add(&static_bitmap, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, 20);
    }

    let welcome_text = StaticText::builder(&panel)
        .with_label(WELCOME_TEXT)
        .with_style(StaticTextStyle::AlignCenterHorizontal)
        .build();

    sizer.add(&welcome_text, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, 0);

    sizer.add_stretch_spacer(2);

    panel.set_sizer(sizer, true);

    DefaultPage { 
        panel 
    }
}
