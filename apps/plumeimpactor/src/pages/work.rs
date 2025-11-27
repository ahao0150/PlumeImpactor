use wxdragon::prelude::*;

const DONE_IMAGE_BYTES: &[u8] = include_bytes!("check.png");
const DONE_IMAGE_DIMENSIONS: u32 = 32;

#[derive(Clone)]
pub struct WorkPage {
    pub panel: Panel,
    status_text: StaticText,
    back_button: Button,
    done_image: StaticBitmap,
    activity_indicator: ActivityIndicator,
}

pub fn create_work_page(frame: &Frame) -> WorkPage {
    let panel = Panel::builder(frame).build();
    let sizer = BoxSizer::builder(Orientation::Vertical).build();

    sizer.add_stretch_spacer(1);

    let activity_indicator = ActivityIndicator::builder(&panel).build();
    activity_indicator.start();

    let stack_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    let done_image = if let Ok(img) = image::load_from_memory_with_format(DONE_IMAGE_BYTES, image::ImageFormat::Png) {
        let resized = img.resize_exact(
            DONE_IMAGE_DIMENSIONS,
            DONE_IMAGE_DIMENSIONS,
            image::imageops::FilterType::Lanczos3,
        );
        let rgba = resized.to_rgba8();
        let bitmap = Bitmap::from_rgba(
            rgba.as_raw(),
            DONE_IMAGE_DIMENSIONS,
            DONE_IMAGE_DIMENSIONS,
        );
        let static_bitmap = StaticBitmap::builder(&panel)
            .with_bitmap(bitmap)
            .with_size(Size::new(
                DONE_IMAGE_DIMENSIONS as i32,
                DONE_IMAGE_DIMENSIONS as i32,
            ))
            .build();
        static_bitmap.hide();
        stack_sizer.add(&activity_indicator, 0, SizerFlag::AlignCenterVertical, 0);
        stack_sizer.add(&static_bitmap, 0, SizerFlag::AlignCenterVertical, 0);
        static_bitmap
    } else {
        let static_bitmap = StaticBitmap::builder(&panel).build();
        static_bitmap.hide();
        stack_sizer.add(&activity_indicator, 0, SizerFlag::AlignCenterVertical, 0);
        stack_sizer.add(&static_bitmap, 0, SizerFlag::AlignCenterVertical, 0);
        static_bitmap
    };
    sizer.add_sizer(&stack_sizer, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, 10);

    let status_text = StaticText::builder(&panel)
        .with_label("Idle")
        .with_style(StaticTextStyle::AlignCenterHorizontal)
        .with_size(Size { width: 300, height: 30 })
        .build();
    sizer.add(&status_text, 0, SizerFlag::AlignCenterHorizontal | SizerFlag::All, 10);

    sizer.add_stretch_spacer(1);

    let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();

    let back_button = Button::builder(&panel)
        .with_label("Back")
        .build();

    back_button.enable(false);

    button_sizer.add(&back_button, 0, SizerFlag::All, 0);
    button_sizer.add_stretch_spacer(1);

    sizer.add_sizer(&button_sizer, 0, SizerFlag::Expand | SizerFlag::Left | SizerFlag::Bottom, 13);

    panel.set_sizer(sizer, true);

    WorkPage { 
        panel,
        status_text,
        back_button,
        done_image,
        activity_indicator,
    }
}

impl WorkPage {
    pub fn set_status_text(&self, text: &str) {
        self.status_text.set_label(text);
    }

    pub fn enable_back_button(&self, enable: bool) {
        self.back_button.enable(enable);
        if enable {
            self.done_image.show(true);
            self.activity_indicator.hide();
        } else {
            self.done_image.hide();
            self.activity_indicator.show(true);
        }
        self.panel.layout();
    }

    pub fn set_back_handler(&self, on_back: impl Fn() + 'static) {
        self.back_button.on_click(move |_| {
            on_back();
        });
    }
}
