use druid::keyboard_types::Key;
use druid::widget::{Flex, Image};
use druid::{
    AppLauncher, Event, ImageBuf, PlatformError, UpdateCtx, Widget, WidgetExt, WindowDesc,
};
use image::{DynamicImage, ImageFormat, ImageReader};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

// Struct to deserialize the framerate from info.json
#[derive(Deserialize, Serialize)]
struct InfoJson {
    framerate: u32,
    frame_extension: String,
    frame_num: u32
}

struct ImageWidget {
    curName: String,
    name: String,
    change: bool,
    image_buf: Option<ImageBuf>,
    t: f64,
    hash: HashMap<String, (String, Option<InfoJson>)>,

    curFrame: u32,
}

impl Widget<()> for ImageWidget {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut (),
        env: &druid::Env,
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
                //println!("{:?}", interval);

                ctx.request_paint();
                ctx.request_anim_frame();
                
                if self.hash.contains_key(&self.name) {
                    if self.hash[&self.name].0 != "folder" {
                        if self.name != self.curName {
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
                                self.curName = self.name.clone();
                            }
                        }
                    } else {
                        let mut load = self.curFrame;
                        let info = self.hash[&self.name].1.as_ref().unwrap();

                        //println!("{:?} and {:?}", self.t, );

                        self.t += (*interval as f64) * 1e-9;
                        if self.name != self.curName  {
                            load = 0;
                            self.t = 0.0;
                        } else if self.t > (1.0 / (info.framerate as f64)){
                            self.t = 0.0;
                            self.curFrame += 1;
                            load = self.curFrame;
                        }

                        if self.curFrame >= info.frame_num {
                            self.curFrame = 0;
                            self.t = 0.0;
                            load = self.curFrame;
                        }

                        // load relevent frame
                        let frame_name = format!("{}/{}", &self.name, format_u32_with_leading_zeros(load, 4));
                        let path =
                            format!("./media/{}.{}", &frame_name, self.hash[&self.name].1.as_ref().unwrap().frame_extension);
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
                            self.curName = self.name.clone();
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
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &(),
        env: &druid::Env,
    ) {
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &(), data: &(), env: &druid::Env) {
        //todo!()
        // if self.change {
        //     if self.image_buf.is_some() {
        //         ctx.request_paint();
        //     }
        //     self.change = false;
        // }
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &(),
        env: &druid::Env,
    ) -> druid::Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &(), env: &druid::Env) {
        if let Some(image_buf) = &self.image_buf {
            let mut image =
                Image::new(image_buf.clone()).fill_mode(druid::widget::FillStrat::FitHeight);
            image.paint(ctx, data, env);
        }
    }
}

fn main() -> Result<(), PlatformError> {
    let file_info_map = get_file_names().unwrap();
    //println!("{:?}", file_info_map);

    let image: DynamicImage = ImageReader::open("./media/test.png")
        .unwrap()
        .decode()
        .unwrap();
    let raw = image.as_bytes();
    let buf = ImageBuf::from_raw(
        raw,
        druid::piet::ImageFormat::RgbaSeparate,
        image.width().try_into().unwrap(),
        image.height().try_into().unwrap(),
    );

    let main_window = WindowDesc::new(ImageWidget {
        curFrame: 0,
        hash: file_info_map,
        t: 0.0,
        name: "test".to_owned(),
        curName: "test".to_owned(),
        change: true,
        image_buf: Some(buf),
    })
    .show_titlebar(false)
    .set_window_state(druid::WindowState::Maximized);
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(())
}

// fn ui_builder() -> impl Widget<()> {
// let image: DynamicImage = ImageReader::open("./media/test.png").unwrap().decode().unwrap();
// let raw = image.as_bytes();
// let buf = ImageBuf::from_raw(raw, druid::piet::ImageFormat::RgbaSeparate, image.width().try_into().unwrap(), image.height().try_into().unwrap());
//     let image = Image::new(buf).fill_mode(druid::widget::FillStrat::FitHeight);

//     Flex::row().with_child(image).center()
// }

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
            let mut framerate = 0;

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