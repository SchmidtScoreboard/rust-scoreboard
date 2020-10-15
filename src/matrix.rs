use crate::common;
use crate::common::ScoreboardSettingsData;
use png;
use rpi_led_matrix;
use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;
use std::fs;
use std::str;
use std::sync::mpsc;
use std::time::Duration;

pub struct Matrix<'a> {
    led_matrix: rpi_led_matrix::LedMatrix,
    receiver: mpsc::Receiver<common::MatrixCommand>,
    screens_map: HashMap<common::ScreenId, Box<dyn ScreenProvider + 'a>>,
    screen_on: bool,
    active_id: common::ScreenId,
}

impl<'a> Matrix<'a> {
    pub fn new(
        led_matrix: rpi_led_matrix::LedMatrix,
        receiver: mpsc::Receiver<common::MatrixCommand>,
        map: HashMap<common::ScreenId, Box<dyn ScreenProvider + 'a>>,
        screen_on: bool,
        active_id: common::ScreenId,
    ) -> Matrix<'a> {
        Matrix {
            led_matrix,
            receiver,
            screens_map: map,
            screen_on,
            active_id,
        }
    }

    fn get_mut_screen(self: &mut Self, id: common::ScreenId) -> &mut Box<dyn ScreenProvider + 'a> {
        self.screens_map
            .get_mut(&id)
            .expect(&format!("Could not find screen {:?}", id))
    }

    fn activate_screen(self: &mut Self) {
        let screen = self.get_mut_screen(self.active_id.clone());
        screen.activate();
    }
    fn deactivate_screen(self: &mut Self) {
        let screen = self.get_mut_screen(self.active_id.clone());
        screen.deactivate();
    }

    // This is the main loop of the entire code
    // Call this after everything else is set up
    pub fn run(self: &mut Self) {
        let mut canvas = self.led_matrix.offscreen_canvas();
        self.screen_on = true;
        self.activate_screen();
        loop {
            let command = self.receiver.recv().unwrap();
            // let command = command.unwrap(); // Get the actual command
            match command {
                common::MatrixCommand::SetActiveScreen(id) => {
                    self.active_id = id;
                    self.screen_on = true;
                    self.activate_screen();
                }
                common::MatrixCommand::SetPower(on) => {
                    self.screen_on = on;
                    if self.screen_on {
                        self.activate_screen();
                    } else {
                        self.deactivate_screen();
                    }
                    canvas.clear();
                    canvas = self.led_matrix.swap(canvas);
                }
                common::MatrixCommand::Display(id) => {
                    if id == self.active_id && self.screen_on {
                        // If the id received matches the active id, display the image
                        self.get_mut_screen(self.active_id.clone())
                            .draw(&mut canvas);
                        canvas = self.led_matrix.swap(canvas);
                        canvas.clear();
                    }
                }
                common::MatrixCommand::UpdateSettings(settings) => {
                    self.get_mut_screen(self.active_id.clone())
                        .update_settings(settings);
                }
            };
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Dimensions {
    pub width: i32,
    pub height: i32,
}

impl Dimensions {
    pub fn new(width: i32, height: i32) -> Dimensions {
        Dimensions { width, height }
    }
}

pub struct Font {
    pub led_font: rpi_led_matrix::LedFont,
    pub dimensions: Dimensions,
}

impl Font {
    fn dump_file(root_path: &std::path::Path, file_name: &str) {
        let bytes =
            FontAssets::get(file_name).expect(&format!("Could not find font {}", file_name));
        let target_dir = root_path.join("fonts");
        let _create_dir_result = fs::create_dir(&target_dir);
        fs::write(&target_dir.join(file_name), bytes).expect("Failed to write file");
    }
    pub fn new(root_path: &std::path::Path, font_file: &str, width: i32, height: i32) -> Font {
        Font::dump_file(root_path, font_file);
        let full_path = root_path.join(format!("fonts/{}", font_file));
        Font {
            led_font: rpi_led_matrix::LedFont::new(std::path::Path::new(&full_path))
                .expect(&format!("Failed to find font file {:?}", &full_path)),
            dimensions: Dimensions::new(width, height),
        }
    }

    pub fn get_text_dimensions(self: &Self, display_text: &str) -> Dimensions {
        Dimensions::new(
            display_text.len() as i32 * self.dimensions.width,
            self.dimensions.height,
        )
    }
}

#[derive(RustEmbed)]
#[folder = "fonts"]
struct FontAssets;

pub struct FontBook {
    pub font4x6: Font,
    pub font5x8: Font,
    pub font7x13: Font,
}

impl FontBook {
    pub fn new(root_path: &std::path::Path) -> FontBook {
        FontBook {
            font4x6: Font::new(
                root_path, "4x6.bdf", 4, 5, // True text height is 5
            ),
            font5x8: Font::new(
                root_path, "5x8.bdf", 5, 6, // True text height is 6
            ),
            font7x13: Font::new(root_path, "7x13.bdf", 7, 9), // True text height is 9
        }
    }
}
#[derive(RustEmbed)]
#[folder = "assets"]
struct Asset;

#[derive(Clone)]
pub struct Pixels {
    pub data: Vec<Vec<rpi_led_matrix::LedColor>>,
}

pub struct PixelBook {
    pub small_arrow: Pixels,
    pub empty_square: Pixels,
    pub filled_square: Pixels,
    pub wifi: Pixels,
    pub phone_frame: Pixels,
    pub green_check: Pixels,
}

impl PixelBook {
    pub fn new(root_path: &std::path::Path) -> PixelBook {
        PixelBook {
            small_arrow: Pixels::from_file(root_path, "small_arrow.png")
                .expect("Could not load small arrow"),
            empty_square: Pixels::from_file(root_path, "empty_square.png")
                .expect("Could not load empty square"),
            filled_square: Pixels::from_file(root_path, "filled_square.png")
                .expect("Could not load filled square"),
            wifi: Pixels::from_file(root_path, "wifi.png").expect("Could not load wifi"),
            phone_frame: Pixels::from_file(root_path, "phone_frame.png")
                .expect("Could not load phone frame"),
            green_check: Pixels::from_file(root_path, "check.png")
                .expect("Could not load green check"),
        }
    }
}

impl Pixels {
    pub fn dump_file(root_path: &std::path::Path, file_name: &str) {
        let target_dir = root_path.join("assets");
        let _create_dir_result = fs::create_dir(&target_dir);
        let contents = Asset::get(file_name).unwrap();
        fs::write(&target_dir.join(file_name), contents).expect("Failed to write file");
    }

    pub fn from_file(
        root_path: &std::path::Path,
        file: &'static str,
    ) -> Result<Pixels, Box<dyn Error>> {
        Pixels::dump_file(root_path, file);
        let full_path = root_path.join(format!("assets/{}", file));
        let decoder = png::Decoder::new(fs::File::open(full_path).unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();
        let width = info.width as usize;
        let height = info.height as usize;
        let mut data: Vec<Vec<rpi_led_matrix::LedColor>> =
            vec![vec![common::new_color(0, 0, 0); width]; height];
        for y in 0..height {
            let row = reader.next_row().unwrap().unwrap();
            for x in 0..width {
                let index = x * 4;
                data[y][x] = common::color_from_slice(&row[index..index + 3]);
            }
        }
        Ok(Pixels { data: data })
    }

    pub fn size(self: &Self) -> Dimensions {
        Dimensions::new(
            self.data[0].len().try_into().unwrap(),
            self.data.len().try_into().unwrap(),
        )
    }

    pub fn flip_vertical(self: &Self) -> Pixels {
        let mut copy = self.data.to_vec();
        copy.reverse();
        Pixels { data: copy }
    }
    pub fn _flip_horizontal(self: &Self) -> Pixels {
        let mut copy = self.data.to_vec();
        copy.iter_mut().for_each(|row| {
            row.reverse();
        });
        Pixels { data: copy }
    }
}
// Common drawing things
pub fn draw_rectangle(
    canvas: &mut rpi_led_matrix::LedCanvas,
    top_left: (i32, i32),
    bottom_right: (i32, i32),
    color: &rpi_led_matrix::LedColor,
) {
    let (x0, y0) = top_left;
    let (x1, y1) = bottom_right;

    for i in y0..y1 {
        canvas.draw_line(x0, i, x1, i, color);
    }
}

pub fn draw_lines(
    canvas: &mut rpi_led_matrix::LedCanvas,
    lines: &[&str],
    x_baseline: i32,
    font: &Font,
    color: &rpi_led_matrix::LedColor,
) {
    let spacing = 2;
    let total_height = (lines.len() - 1) * spacing + lines.len() * font.dimensions.height as usize;
    let top_offset =
        (canvas.canvas_size().1 as usize - total_height) / 2 + font.dimensions.height as usize;
    let top_offset: i32 = top_offset.try_into().unwrap();
    for (i, text) in lines.iter().enumerate() {
        let index: i32 = i.try_into().unwrap();
        canvas.draw_text(
            &font.led_font,
            &text,
            x_baseline,
            top_offset + (index * (&font.dimensions.height + spacing as i32)),
            &color,
            0,
            false,
        );
    }
}

pub fn draw_pixels(canvas: &mut rpi_led_matrix::LedCanvas, pixels: &Pixels, top_left: (i32, i32)) {
    let (x0, y0) = top_left;
    let mut y = 0;
    pixels.data.iter().for_each(|row| {
        let mut x = 0;
        row.iter().for_each(|pixel| {
            canvas.set(x0 + x, y0 + y, &pixel);
            x += 1;
        });
        y += 1;
    });
}

pub trait ScreenProvider {
    // Activate is called by the Display driver
    // Activate sets up whatever refreshing this screen needs
    fn activate(self: &mut Self) {}

    // Cleanup any unused resources
    // Most screens won't have to do anything here
    // If there are owned threads, cancel them
    fn deactivate(self: &Self) {}

    // Draw is not blocking--fills in the canvas and returns it immediately
    // Draw can check for new data on an internal try_recv, and update internal variables, but
    // it must not issue any network requests or perform any other asynchronous action
    // Asynchronous actions must be driven by a refresh thread set up in `activate`
    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas);

    // Handle recieving new scoreboard settings
    // This may change timezone and any other screen specific features
    fn update_settings(self: &mut Self, settings: ScoreboardSettingsData);

    fn get_screen_id(self: &Self) -> common::ScreenId;

    fn get_sender(self: &Self) -> mpsc::Sender<common::MatrixCommand>;

    fn send_draw_command(self: &Self, duration: Option<Duration>) {
        let id = self.get_screen_id();
        let sender = self.get_sender();
        if let Some(duration) = duration {
            let _next_draw_thread = std::thread::spawn(move || {
                std::thread::sleep(duration);
                draw_command(sender, id);
            });
        } else {
            draw_command(sender, id);
        }
    }
}

fn draw_command(sender: mpsc::Sender<common::MatrixCommand>, id: common::ScreenId) {
    sender.send(common::MatrixCommand::Display(id)).unwrap();
}
