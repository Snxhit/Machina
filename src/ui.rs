const BAR_HEIGHT: usize = 12;

pub struct UIState {
    pub brush_color: [u8; 4],
    pub is_paused: bool,
    pub fps: u32,
}

pub fn is_over_ui(y: usize, height: usize) -> bool {
    y < BAR_HEIGHT || y >= height.saturating_sub(BAR_HEIGHT)
}

pub fn render_ui(frame: &mut [u8], width: usize, height: usize, state: &UIState) {
    // top and bottom bar bgs
    draw_rect(frame, width, height, 0, 0, width, BAR_HEIGHT, [100, 100, 100, 255]);
    draw_rect(frame, width, height, 0, height - BAR_HEIGHT, width, BAR_HEIGHT, [100, 100, 100, 255]);
}

// drawing helpers
fn set_pixel(frame: &mut [u8], width: usize, height: usize, x: usize, y: usize, color: [u8; 4]) {
    if x >= width || y >= height {
        return;
    }
    let idx = (y * width + x) * 4;
    frame[idx..idx + 4].copy_from_slice(&color);
}

fn draw_rect(frame: &mut [u8], width: usize, height: usize, rx: usize, ry: usize, rw: usize, rh: usize, color: [u8; 4]) {
    for y in ry..(ry + rh) {
        for x in rx..(rx + rw) {
            set_pixel(frame, width, height, x, y, color);
        }
    }
}