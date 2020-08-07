use sport_common;

struct HockeyGame {
    sport_common::
}

impl ImageProvider for HockeyGame {
    fn get_image(self: &Self, canvas: &mut rpi_led_matrix::LedCanvas) {
        let (width, height) = canvas.size();
        canvas.draw_line(0, 0, width, 0, &self.home_team.primary_color);
    }
}