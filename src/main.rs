use druid::widget::{Button, Flex, Image, Label};
use druid::{AppLauncher, ImageBuf, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};
use image::{DynamicImage, GenericImageView, ImageReader};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder()).show_titlebar(false).set_window_state(druid::WindowState::Maximized);
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}

fn ui_builder() -> impl Widget<u32> {
    let image: DynamicImage = ImageReader::open("./media/test.jpg").unwrap().decode().unwrap();
    // The label text will be computed dynamically based on the current locale and count
    let raw = image.as_bytes();
    let buf = ImageBuf::from_raw(raw, druid::piet::ImageFormat::Rgb, image.width().try_into().unwrap(), image.height().try_into().unwrap());
    let image = Image::new(buf).fill_mode(druid::widget::FillStrat::FitHeight);

    Flex::row().with_child(image).center()
}