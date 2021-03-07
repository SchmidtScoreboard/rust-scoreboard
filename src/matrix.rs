use crate::animation;
use crate::common;
use crate::common::ScoreboardSettingsData;
use crate::message;
use crate::scheduler;
use crate::scoreboard_settings::ScoreboardSettings;
use crate::setup_screen;
use png;
use rpi_led_matrix;
use std::any::Any;
use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;
use std::fs;
use std::str;
use std::sync::mpsc;
use std::time::Duration;

pub struct Matrix<'a> {
    led_matrix: rpi_led_matrix::LedMatrix, // The actual matrix
    receiver: mpsc::Receiver<common::MatrixCommand>, // Receive commands from the button, the webserver, and responses to shell commands
    screens_map: HashMap<common::ScreenId, Box<dyn ScreenProvider + 'a>>, // The map of all the active screens
    settings: ScoreboardSettings, // The main scoreboard settings
    webserver_responder: mpsc::Sender<common::WebserverResponse>, // Send responses to the webserver
    shell_sender: mpsc::Sender<common::ShellCommand>, // Send commands to shell
    message_screen: message::MessageScreen, // If this is set, display this message until it is unset
}

impl<'a> Matrix<'a> {
    pub fn new(
        led_matrix: rpi_led_matrix::LedMatrix,
        message_screen: message::MessageScreen,
        receiver: mpsc::Receiver<common::MatrixCommand>,
        map: HashMap<common::ScreenId, Box<dyn ScreenProvider + 'a>>,
        settings: ScoreboardSettings,
        webserver_responder: mpsc::Sender<common::WebserverResponse>,
        shell_sender: mpsc::Sender<common::ShellCommand>,
    ) -> Matrix<'a> {
        Matrix {
            led_matrix,
            receiver,
            screens_map: map,
            settings,
            webserver_responder,
            shell_sender,
            message_screen,
        }
    }

    fn get_mut_screen(self: &mut Self, id: &common::ScreenId) -> &mut Box<dyn ScreenProvider + 'a> {
        self
            .screens_map
            .get_mut(id)
            .expect(&format!("Could not find screen {:?}", id))
    }

    fn get_mut_active_screen(self: &mut Self) -> &mut Box<dyn ScreenProvider + 'a> {
        let id = self.settings.get_active_screen().get_base_id().clone();
        self.get_mut_screen(&id)
    }

    fn activate_screen(self: &mut Self) {
        self.update_settings_on_active_screen();
        let screen = self.get_mut_active_screen();
        screen.activate();
    }

    fn update_settings_on_active_screen(self: &mut Self) {
        let settings_copy = self.settings.get_settings_clone();
        let screen = self.get_mut_active_screen();
        screen.update_settings(settings_copy);
    }
    fn deactivate_screen(self: &mut Self) {
        let screen = self.get_mut_active_screen();
        screen.deactivate();
    }
    fn send_command(self: &Self, response: common::ShellCommand) {
        self.shell_sender.send(response).unwrap();
    }

    fn send_response(self: &Self, response: common::WebserverResponse) {
        self.webserver_responder.send(response).unwrap();
    }

    fn show_message(self: &mut Self, message: String) {
        self.message_screen.set_message(message);
        self.message_screen.send_draw_command(None);
    }

    fn hide_message(self: &mut Self) {
        self.message_screen.unset_message();
        self.get_mut_active_screen().send_draw_command(None);
    }

    fn get_setup_screen(self: &mut Self) -> &mut setup_screen::SetupScreen {
        match self
            .get_mut_screen(&common::ScreenId::Setup)
            .as_any()
            .downcast_mut::<setup_screen::SetupScreen>()
        {
            Some(setup_screen) => setup_screen,
            None => panic!("Found screen is NOT the setup screen"),
        }
    }

    // This is the main loop of the entire code
    // Call this after everything else is set up
    pub fn run(self: &mut Self) {
        let mut canvas = self.led_matrix.offscreen_canvas();
        self.settings.set_power(&true);
        self.activate_screen();
        loop {
            let command = self.receiver.recv().unwrap();
            // let command = command.unwrap(); // Get the actual command
            match command {
                common::MatrixCommand::SetActiveScreen(id) => {
                    self.deactivate_screen();
                    self.settings.set_active_screen(&id);
                    self.settings.set_power(&true);
                    self.activate_screen();
                    self.send_response(common::WebserverResponse::SetActiveScreenResponse(
                        self.settings.get_settings_clone(),
                    ));
                }
                common::MatrixCommand::SetPower {
                    from_webserver,
                    power,
                } => {
                    let on = match power {
                        Some(power) => power,
                        None => !self.settings.get_power(),
                    };
                    self.settings.set_power(&on);
                    if *self.settings.get_power() {
                        self.activate_screen();
                    } else {
                        self.deactivate_screen();
                    }
                    canvas.clear();
                    canvas = self.led_matrix.swap(canvas);
                    canvas.clear();
                    if from_webserver {
                        self.send_response(common::WebserverResponse::SetPowerResponse(
                            self.settings.get_settings_clone(),
                        ));
                    }
                }
                common::MatrixCommand::Display(id) => {
                    if self.message_screen.is_message_set() {
                        self.message_screen.draw(&mut canvas);
                        canvas = self.led_matrix.swap(canvas);
                        canvas.clear();
                    } else {
                        if id == *self.settings.get_active_screen().get_base_id() && *self.settings.get_power() {
                            // If the id received matches the active id, display the image
                            self.get_mut_screen(&id).draw(&mut canvas);
                            canvas = self.led_matrix.swap(canvas);
                            canvas.clear();
                        }
                    }
                }
                common::MatrixCommand::_CheckSmartScreen() => {}
                common::MatrixCommand::GetSettings() => {
                    self.send_response(common::WebserverResponse::GetSettingsResponse(
                        self.settings.get_settings_clone(),
                    ));
                }
                common::MatrixCommand::UpdateSettings(settings) => {
                    let original_brightness = self.settings.get_brightness();
                    self.settings.update_settings(settings);
                    let new_brightness = self.settings.get_brightness();
                    self.update_settings_on_active_screen();
                    self.send_response(common::WebserverResponse::UpdateSettingsResponse(
                        self.settings.get_settings_clone(),
                    ));
                    if original_brightness != new_brightness {
                        // Restart the scoreboard
                        self.settings.set_power(&true);
                        self.show_message("Rebooting...".to_string());
                        self.send_command(common::ShellCommand::Reboot { settings: None });
                    }
                }
                common::MatrixCommand::Reboot() => {
                    self.settings.set_power(&true);
                    self.show_message("Rebooting...".to_string());
                    self.send_command(common::ShellCommand::Reboot {
                        settings: Some(self.settings.get_settings_clone()),
                    });
                }
                common::MatrixCommand::Reset { from_webserver } => {
                    self.settings.set_power(&true);
                    // Reset scoreboard settings,  updating the screen to show the message
                    self.deactivate_screen();
                    self.settings.set_setup_state(&common::SetupState::Hotspot);
                    self.settings.set_active_screen(&common::ScreenId::Setup);
                    self.activate_screen();
                    self.update_settings_on_active_screen();

                    self.show_message("Resetting...".to_string());

                    self.send_command(common::ShellCommand::Reset {
                        from_matrix: true,
                        from_webserver: if from_webserver {
                            Some(self.settings.get_settings_clone())
                        } else {
                            None
                        },
                    });
                }
                common::MatrixCommand::GotHotspotConnection() => {
                    // Change setup state
                    self.settings.set_power(&true);

                    match self.settings.get_setup_state() {
                        common::SetupState::Hotspot | common::SetupState::WifiConnect => {
                            self.settings
                                .set_setup_state(&common::SetupState::WifiConnect);
                            self.update_settings_on_active_screen();
                            self.send_response(
                                common::WebserverResponse::GotHotspotConnectionResponse(Some(
                                    self.settings.get_settings_clone(),
                                )),
                            );
                        }
                        _ => {
                            self.send_response(
                                common::WebserverResponse::GotHotspotConnectionResponse(None),
                            );
                        }
                    }
                }
                common::MatrixCommand::GotWifiDetails { ssid, password } => {
                    self.settings.set_power(&true);
                    // Got wifi details, set the wpa supplicant file and restart
                    self.deactivate_screen();
                    self.settings
                        .set_setup_state(&common::SetupState::WifiConnect);
                    self.settings.set_active_screen(&common::ScreenId::Setup);
                    self.activate_screen();
                    let setup = self.get_setup_screen();
                    setup.attempting_connection();

                    self.send_command(common::ShellCommand::SetupWifi {
                        ssid,
                        password,
                        settings: self.settings.get_settings_clone(),
                    });
                    // Send the response immediately
                    self.send_response(common::WebserverResponse::GotWifiDetailsResponse(Some(
                        self.settings.get_settings_clone(),
                    )));
                }
                common::MatrixCommand::FinishedWifiConnection(result) => match result {
                    Ok(_) => {
                        self.get_setup_screen()
                            .set_sync_code(common::get_sync_code());
                        self.settings.set_setup_state(&common::SetupState::Sync);
                        self.update_settings_on_active_screen();
                    }
                    Err(e) => {
                        error!("Error setting up wifi {:?} ", e);
                        let setup = self.get_setup_screen();
                        setup.failed_connection();
                        // TODO display an error on the wifi details screen
                    }
                },
                common::MatrixCommand::FinishedReset(result) => {
                    self.settings.set_power(&true);
                    self.hide_message();
                    match result {
                        Ok(_) => {
                            self.settings.set_setup_state(&common::SetupState::Hotspot);
                            self.update_settings_on_active_screen();
                        }
                        Err(e) => {
                            error!("Error setting up wifi {:?} ", e);
                            // TODO display an error on the wifi details screen
                        }
                    }
                }
                common::MatrixCommand::SyncCommand {
                    from_webserver,
                    show_sync,
                } => {
                    // Got a sync command with optional showSync.
                    self.settings.set_power(&true);
                    let current_setup_state = self.settings.get_setup_state();
                    if current_setup_state == &common::SetupState::Ready
                        || current_setup_state == &common::SetupState::Sync
                    {
                        self.deactivate_screen();
                        let show_sync = match show_sync {
                            Some(show_sync) => show_sync,
                            None => self.settings.get_setup_state() != &common::SetupState::Sync,
                        };
                        if show_sync {
                            debug!("Showing sync screen");
                            self.settings.set_setup_state(&common::SetupState::Sync);
                            self.settings.set_active_screen(&common::ScreenId::Setup);
                        } else {
                            debug!("Showing hockey screen");
                            self.settings.set_setup_state(&common::SetupState::Ready);
                            self.settings.set_active_screen(&common::ScreenId::Hockey);
                        }
                        self.update_settings_on_active_screen();
                        self.activate_screen();
                        if from_webserver {
                            self.send_response(common::WebserverResponse::SyncCommandResponse(
                                Some(self.settings.get_settings_clone()),
                            ));
                        }
                    } else {
                        if from_webserver {
                            self.send_response(common::WebserverResponse::SyncCommandResponse(
                                None,
                            ));
                        }
                    }
                }
            };
        }
    }
}
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Dimensions {
    pub width: i32,
    pub height: i32
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct FontDimensions {
    pub width: i32,
    pub height: i32,
    pub width_overrides: HashMap<char, i32>
}

impl FontDimensions {
    pub fn new(width: i32, height: i32, width_overrides: HashMap<char, i32>) -> FontDimensions {
        FontDimensions { width , height, width_overrides}
    }
}

impl Dimensions {
    pub fn new(width: i32, height: i32) -> Dimensions {
        Dimensions {width, height}
    }
}

pub struct Font {
    pub led_font: rpi_led_matrix::LedFont,
    pub dimensions: FontDimensions,
}

impl Font {
    fn dump_file(root_path: &std::path::Path, file_name: &str) {
        let bytes =
            FontAssets::get(file_name).expect(&format!("Could not find font {}", file_name));
        let target_dir = root_path.join("fonts");
        let _create_dir_result = fs::create_dir(&target_dir);
        fs::write(&target_dir.join(file_name), bytes).expect("Failed to write file");
    }
    pub fn new(root_path: &std::path::Path, font_file: &str, width: i32, height: i32, width_overrides: HashMap<char, i32>) -> Font {
        Font::dump_file(root_path, font_file);
        let full_path = root_path.join(format!("fonts/{}", font_file));
        Font {
            led_font: rpi_led_matrix::LedFont::new(std::path::Path::new(&full_path))
                .expect(&format!("Failed to find font file {:?}", &full_path)),
            dimensions: FontDimensions::new(width, height, width_overrides),
        }
    }

    pub fn get_text_dimensions(self: &Self, display_text: &str) -> Dimensions{
        let width = display_text.chars().map(|c| {
            match self.dimensions.width_overrides.get(&c) {
                Some(w) => w,
                None => &self.dimensions.width
            }
        } ).sum();
        Dimensions::new(width, self.dimensions.height)

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
        let mut override4x6= HashMap::new();
        let mut override5x8= HashMap::new();

        override4x6.insert('N',5);
        override5x8.insert('I',4);
        override5x8.insert('T',4);
        override5x8.insert('Y',6);
        FontBook {
            font4x6: Font::new(
                root_path, "4x6.bdf", 4, 5, override4x6 // True text height is 5
            ),
            font5x8: Font::new(
                root_path, "5x8.bdf", 5, 6, override5x8 // True text height is 6
            ),
            font7x13: Font::new(root_path, "7x13.bdf", 7, 9, HashMap::new()), // True text height is 9
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
    pub red_x: Pixels,
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
            red_x: Pixels::from_file(root_path, "red-x.png").expect("Could not load red X"),
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

pub fn draw_text_centered_horizontally(
    canvas: &mut rpi_led_matrix::LedCanvas,
    text: &str,
    y_center: i32,
    font: &Font,
    color: &rpi_led_matrix::LedColor,
) {
    let text_dim = font.get_text_dimensions(text);
    let (canvas_width, _canvas_height) = canvas.canvas_size();
    canvas.draw_text(
        &font.led_font,
        text,
        (canvas_width - text_dim.width) / 2,
        y_center + (text_dim.height / 2),
        color,
        0,
        false,
    );
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

pub fn draw_message(
    canvas: &mut rpi_led_matrix::LedCanvas,
    font: &Font,
    message: &str,
    waves_anim: &mut animation::WavesAnimation,
) {
    let text_dimensions = font.get_text_dimensions(message);
    let white = common::new_color(255, 255, 255);
    canvas.draw_text(
        &font.led_font,
        &message,
        1,
        1 + text_dimensions.height,
        &white,
        0,
        false,
    );

    waves_anim.draw(canvas);
}

pub trait ScreenProvider {
    // Activate is called by the Display driver
    // Activate sets up whatever refreshing this screen needs
    fn activate(self: &mut Self) {}

    // Cleanup any unused resources
    // Most screens won't have to do anything here
    // If there are owned threads, cancel them
    fn deactivate(self: &mut Self) {}

    // Draw is not blocking--fills in the canvas and returns it immediately
    // Draw can check for new data on an internal try_recv, and update internal variables, but
    // it must not issue any network requests or perform any other asynchronous action
    // Asynchronous actions must be driven by a refresh thread set up in `activate`
    fn draw(self: &mut Self, canvas: &mut rpi_led_matrix::LedCanvas);

    // Handle recieving new scoreboard settings
    // This may change timezone and any other screen specific features
    fn update_settings(self: &mut Self, settings: ScoreboardSettingsData);

    fn get_screen_id(self: &Self) -> common::ScreenId;

    fn get_sender(self: &Self) -> mpsc::Sender<scheduler::DelayedCommand>;

    fn send_draw_command(self: &Self, duration: Option<Duration>) {
        let id = self.get_screen_id();
        let sender = self.get_sender();
        sender
            .send(scheduler::DelayedCommand::new(
                scheduler::Command::MatrixCommand(common::MatrixCommand::Display(id)),
                duration,
            ))
            .unwrap();
    }

    fn as_any(&mut self) -> &mut dyn Any;

    fn has_priority(self: &Self, _team_id: u32) -> bool {
        false
    }
}
