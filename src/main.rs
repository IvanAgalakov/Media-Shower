use druid::widget::{Flex, Image};
use druid::{AppLauncher, ImageBuf, PlatformError, Widget, WidgetExt, WindowDesc};
use image::{DynamicImage, ImageReader};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// Struct to deserialize the framerate from info.json
#[derive(Deserialize, Serialize)]
struct InfoJson {
    framerate: u32,
}

fn main() -> Result<(), PlatformError> {
    let file_info_map = get_file_names().unwrap();
    println!("{:?}", file_info_map);

    let main_window = WindowDesc::new(ui_builder()).show_titlebar(false).set_window_state(druid::WindowState::Maximized);
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}

fn ui_builder() -> impl Widget<u32> {
    let image: DynamicImage = ImageReader::open("./media/test.png").unwrap().decode().unwrap();
    let raw = image.as_bytes();
    let buf = ImageBuf::from_raw(raw, druid::piet::ImageFormat::RgbaSeparate, image.width().try_into().unwrap(), image.height().try_into().unwrap());
    let image = Image::new(buf).fill_mode(druid::widget::FillStrat::FitHeight);

    Flex::row().with_child(image).center()
}

// Function to get file names and record them in a HashMap
fn get_file_names() -> std::io::Result<HashMap<String, (String, u32)>> {
    let path = Path::new("./media/"); // Specify the directory path
    let mut file_info_map = HashMap::new(); // HashMap to store file names, extensions, and framerate

    // Read the directory
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let file_name = entry.file_name(); // This is OsString
        let file_path = entry.path(); // Full path of the file/directory

        if file_type.is_dir() {
            // If it's a directory, the extension is "folder" and framerate comes from info.json
            let folder_base = file_name.to_string_lossy().to_string();
            let mut framerate = 0;

            // Check if info.json exists and extract framerate if available
            let info_json_path = file_path.join("info.json");
            if info_json_path.exists() {
                if let Ok(info_json_file) = fs::File::open(&info_json_path) {
                    if let Ok(info) = serde_json::from_reader::<_, InfoJson>(info_json_file) {
                        framerate = info.framerate;
                    }
                }
            }

            file_info_map.insert(folder_base, ("folder".to_string(), framerate));
        } else {
            // If it's a file, separate the base name and extension
            if let Some(extension) = file_path.extension() {
                if let Some(base_name) = file_path.file_stem() {
                    let base_name_str = base_name.to_string_lossy().to_string();
                    let ext = extension.to_string_lossy().to_string();
                    file_info_map.insert(base_name_str, (ext, 0)); // Framerate is 0 for files
                }
            }
        }
    }

    Ok(file_info_map)
}
