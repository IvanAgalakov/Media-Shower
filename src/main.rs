use druid::keyboard_types::Key;
use druid::widget::Image;
use druid::{
    AppLauncher, Event, ImageBuf, PlatformError, Widget, WindowDesc,
};
use image::{DynamicImage, ImageReader};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Struct to deserialize the properties from info.json in folders
#[derive(Deserialize, Serialize)]
struct InfoJson {
    framerate: u32,
    frame_extension: String,
    frame_num: u32,
}

// Information used internally within the ImageWidget
struct ImageWidget {
    cur_name: String,
    name: String,
    image_buf: Option<ImageBuf>,
    t: f64,
    hash: HashMap<String, (String, Option<InfoJson>)>,
    active_format: Option<druid::piet::ImageFormat>,

    cur_frame: u32,
}

// Druid Widget used to display the images and videos
impl Widget<()> for ImageWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        _data: &mut (),
        _env: &druid::Env,
    ) {
        match event {
            Event::KeyDown(event) => {
                if let Key::Character(character) = &event.key {
                    // Handle the character
                    self.name += character;
                } else if event.key == Key::Backspace {
                    // Handle backspace
                    // You can remove the last character from self.name here
                    self.name.pop();
                }
                ctx.request_anim_frame();
            }
            Event::AnimFrame(interval) => {
                ctx.request_anim_frame();

                if self.hash.contains_key(&self.name) {
                    if self.hash[&self.name].0 != "folder" { // For individual images
                        if self.name != self.cur_name {
                            ctx.request_paint();
                            let path =
                                format!("./media/{}.{}", &self.name, &self.hash[&self.name].0);
                            let image: DynamicImage =
                                ImageReader::open(&path).unwrap().decode().unwrap();
                            let width: usize = image.width().try_into().unwrap();
                            let height: usize = image.height().try_into().unwrap();
                            let raw = image.as_bytes();

                            let mut image_format: Option<druid::piet::ImageFormat> = None;
                            if raw.len()
                                == width
                                    * height
                                    * druid::piet::ImageFormat::RgbaSeparate.bytes_per_pixel()
                            {
                                image_format = Some(druid::piet::ImageFormat::RgbaSeparate);
                            }
                            if raw.len()
                                == width * height * druid::piet::ImageFormat::Rgb.bytes_per_pixel()
                            {
                                image_format = Some(druid::piet::ImageFormat::Rgb);
                            }

                            if let Some(image_format) = image_format {
                                self.image_buf =
                                    Some(ImageBuf::from_raw(raw, image_format, width, height));
                                self.cur_name = self.name.clone();
                            }
                        }
                    } else { // Folder of images to get rendered as a video
                        let mut load = self.cur_frame;
                        let info = self.hash[&self.name].1.as_ref().unwrap();

                        let mut process_this_frame = false;
                        self.t += (*interval as f64) * 1e-9;
                        if self.name != self.cur_name {
                            load = 1;
                            self.t = 0.0;
                            process_this_frame = true;
                            self.active_format = None;
                        } else if self.t > (1.0 / (info.framerate as f64)) {
                            self.t = 0.0;
                            self.cur_frame += 1;
                            load = self.cur_frame;
                            process_this_frame = true;
                        }

                        if self.cur_frame >= info.frame_num {
                            self.cur_frame = 1;
                            self.t = 0.0;
                            load = self.cur_frame;
                        }

                        if process_this_frame {
                            ctx.request_paint();
                            // load relevent frame
                            let frame_name = format!(
                                "{}/{}",
                                &self.name,
                                format_u32_with_leading_zeros(load, 4)
                            );
                            let path = format!(
                                "./media/{}.{}",
                                &frame_name,
                                self.hash[&self.name].1.as_ref().unwrap().frame_extension
                            );
                            let image: DynamicImage =
                                ImageReader::open(&path).unwrap().decode().unwrap();
                            let width: usize = image.width().try_into().unwrap();
                            let height: usize = image.height().try_into().unwrap();
                            let raw = image.as_bytes();

                            let mut image_format: Option<druid::piet::ImageFormat> = None;
                            if self.active_format.is_some() {
                                image_format = self.active_format;
                            } else {
                                if raw.len()
                                    == width
                                        * height
                                        * druid::piet::ImageFormat::RgbaSeparate.bytes_per_pixel()
                                {
                                    image_format = Some(druid::piet::ImageFormat::RgbaSeparate);
                                }
                                if raw.len()
                                    == width * height * druid::piet::ImageFormat::Rgb.bytes_per_pixel()
                                {
                                    image_format = Some(druid::piet::ImageFormat::Rgb);
                                }
                                self.active_format = image_format;
                            }

                            if let Some(image_format) = image_format {
                                self.image_buf =
                                    Some(ImageBuf::from_raw(raw, image_format, width, height));
                                self.cur_name = self.name.clone();
                            }
                        }
                    }
                }
            }
            _ => {
                ctx.request_focus();
            }
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &(),
        _env: &druid::Env,
    ) {
    }

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &(), _data: &(), _env: &druid::Env) {}

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &(),
        _env: &druid::Env,
    ) -> druid::Size {
        bc.max()                        // fill the full screen
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &(), env: &druid::Env) {
        if let Some(image_buf) = &self.image_buf {                          // paint if the image buffer exists
            let mut image =
                Image::new(image_buf.clone()).fill_mode(druid::widget::FillStrat::FitHeight);
            image.paint(ctx, data, env);
        }
    }
}

use std::process::Command;

fn is_ffmpeg_installed() -> bool {          // for later ffmpeg implementation
    let output = Command::new("ffmpeg")
        .arg("-version")
        .output();
    
    match output {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}


fn main() -> Result<(), PlatformError> {    
    let file_info_map = get_file_names().unwrap();

    if is_ffmpeg_installed() {                  // for later ffmpeg implementation
        println!("ffmpeg is installed.");
    } else {
        println!("ffmpeg is not installed.");
    }

    let main_window = WindowDesc::new(ImageWidget {
        cur_frame: 1,
        hash: file_info_map,
        t: 0.0,
        name: "".to_owned(),
        cur_name: "".to_owned(),
        image_buf: None,
        active_format: None
    })
    .show_titlebar(false)
    .set_window_state(druid::WindowState::Maximized);
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(())
}

// Function to get file names and record them in a HashMap
fn get_file_names() -> std::io::Result<HashMap<String, (String, Option<InfoJson>)>> { 
    let path = Path::new("./media/"); // Specify the directory path
    let mut file_info_map: HashMap<String, (String, Option<InfoJson>)> = HashMap::new(); // HashMap to store file names, extensions, and framerate

    // Read the directory
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let file_name = entry.file_name(); // This is OsString
        let file_path = entry.path(); // Full path of the file/directory

        if file_type.is_dir() {
            // If it's a directory, the extension is "folder" and framerate comes from info.json
            let folder_base = file_name.to_string_lossy().to_string();

            // Check if info.json exists and extract framerate if available
            let info_json_path = file_path.join("info.json");
            if info_json_path.exists() {
                if let Ok(info_json_file) = fs::File::open(&info_json_path) {
                    if let Ok(info) = serde_json::from_reader::<_, InfoJson>(info_json_file) {
                        file_info_map.insert(folder_base, ("folder".to_string(), Some(info)));
                    }
                }
            }
        } else {
            // If it's a file, separate the base name and extension
            if let Some(extension) = file_path.extension() {
                if let Some(base_name) = file_path.file_stem() {
                    let base_name_str = base_name.to_string_lossy().to_string();
                    let ext = extension.to_string_lossy().to_string();
                    file_info_map.insert(base_name_str, (ext, None)); // Framerate is 0 for files
                }
            }
        }
    }

    Ok(file_info_map)
}

fn format_u32_with_leading_zeros(value: u32, width: usize) -> String {
    format!("{:0width$}", value, width = width)
}
