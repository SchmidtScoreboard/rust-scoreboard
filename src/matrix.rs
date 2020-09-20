use crate::common;
use crate::common::ScoreboardSettingsData;
use rpi_led_matrix;
use std::collections::HashMap;
use std::error::Error;
use std::str;
use std::sync::mpsc;

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
    fn get_screen(self: &Self, id: &common::ScreenId) -> &Box<dyn ScreenProvider + 'a> {
        self.screens_map
            .get(id)
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

    fn handle_power(self: &mut Self, screen_on: bool) {
        // TODO set power to the matrix
        self.screen_on = screen_on;
        if self.screen_on {
            self.activate_screen();
        } else {
            self.deactivate_screen();
            let mut canvas = self.led_matrix.offscreen_canvas();
            canvas.clear();
            self.led_matrix.swap(canvas);
        }
    }
    // This is the main loop of the entire code
    // Call this after everything else is set up
    pub fn run(self: &mut Self) {
        loop {
            let command = self.receiver.recv().unwrap();
            // let command = command.unwrap(); // Get the actual command
            match command {
                common::MatrixCommand::SetActiveScreen(id) => {
                    self.handle_power(true);
                }
                common::MatrixCommand::SetPower(on) => {
                    self.handle_power(on);
                }
                common::MatrixCommand::Display(id) => {
                    if id == self.active_id && self.screen_on {
                        // If the id received matches the active id, display the image
                        let mut canvas = self.led_matrix.offscreen_canvas();
                        self.get_mut_screen(self.active_id.clone())
                            .draw(&mut canvas);
                        self.led_matrix.swap(canvas);
                    }
                }
                common::MatrixCommand::UpdateSettings(settings) => {}
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
    pub fn new(led_font: rpi_led_matrix::LedFont, width: i32, height: i32) -> Font {
        Font {
            led_font,
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
#[folder = "assets"]
struct Asset;

pub struct FontBook {
    pub font4x6: Font,
    pub font5x8: Font,
    pub font7x13: Font,
}

impl FontBook {
    pub fn new() -> FontBook {
        FontBook {
            font4x6: Font::new(
                rpi_led_matrix::LedFont::new(std::path::Path::new("fonts/4x6.bdf")).unwrap(),
                4,
                5, // True text height is 5
            ),
            font5x8: Font::new(
                rpi_led_matrix::LedFont::new(std::path::Path::new("fonts/5x8.bdf")).unwrap(),
                5,
                6, // True text height is 6
            ),
            font7x13: Font::new(
                rpi_led_matrix::LedFont::new(std::path::Path::new("fonts/7x13.bdf")).unwrap(),
                7,
                13,
            ),
        }
    }
}
pub struct Pixels {
    pub data: Vec<Vec<rpi_led_matrix::LedColor>>,
}

impl Pixels {
    fn new(data: Vec<Vec<rpi_led_matrix::LedColor>>) -> Pixels {
        Pixels { data }
    }

    pub fn from_file(file: &'static str) -> Result<Pixels, Box<dyn Error>> {
        let contents = Asset::get(file).unwrap();
        let contents = str::from_utf8(&contents).unwrap();

        let data: Result<Vec<_>, _> = contents
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|word| common::color_from_string(&word[2..]))
                    .collect()
            })
            .collect();

        Ok(Pixels { data: data? })
    }

    pub fn flip_vertical(self: &mut Self) {
        self.data.reverse();
    }
    pub fn flip_horizontal(self: &mut Self) {
        // self.data.iter().for_each(|&mut row| {
        //     row.reverse();
        // });
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

pub fn draw_pixels(canvas: &mut rpi_led_matrix::LedCanvas, pixels: &Pixels, top_left: (i32, i32)) {
    let (x0, y0) = top_left;
    let mut x = 0;
    pixels.data.iter().for_each(|row| {
        let mut y = 0;
        row.iter().for_each(|pixel| {
            canvas.set(x0 + x, y0 + y, &pixel);
            y += 1;
        });
        x += 1;
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
    fn update_settings(self: &mut Self, settings: ScoreboardSettingsData) {}
}
