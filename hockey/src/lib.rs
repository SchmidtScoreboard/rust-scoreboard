use rpi_led_matrix;
use std::collections::HashMap;
use std::sync::mpsc;

pub struct Hockey<'a> {
    led_matrix: &'a rpi_led_matrix::LedMatrix,
    sender: mpsc::Sender<common::MatrixCommand>,
}

impl<'a> Hockey<'a> {
    pub fn new(
        led_matrix: &'a rpi_led_matrix::LedMatrix,
        sender: mpsc::Sender<common::MatrixCommand>,
    ) -> Hockey {
        Hockey { led_matrix, sender }
    }
}

impl matrix::ScreenProvider for Hockey<'_> {
    fn activate(self: &Self) {
        // Fetch scores from the web
        // Setup a timer to get scores every minute
        // Setup a timer to shift screens every Y seconds
        // For now, let's just get a canvas back
        let mut canvas = self.led_matrix.offscreen_canvas();
        let (width, height) = canvas.size();

        canvas.draw_line(0, 0, width, height, &common::new_color(255, 255, 255));

        self.sender
            .send(common::MatrixCommand::Display {
                id: common::ScreenId::Hockey,
                canvas: canvas,
            })
            .expect("Failed to send canvas");
    }
    fn should_show_refresh_on_activate(self: &Self) -> bool {
        // TODO, this should be true if the hockey data is out of date
        false
    }
}
