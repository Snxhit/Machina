use font8x8::{UnicodeFonts, BASIC_FONTS};

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
    draw_rect(frame, width, height, 0, 0, width, BAR_HEIGHT, [100, 100, 100, 100]);
    draw_rect(frame, width, height, 0, height - BAR_HEIGHT, width, BAR_HEIGHT, [100, 100, 100, 30]);

    // divider line
    draw_rect(frame, width, height, 0, BAR_HEIGHT - 1, width, 1, [200, 200, 200, 100]);
    draw_rect(frame, width, height, 0,  height - BAR_HEIGHT, width, 1, [200, 200, 200, 100]);

    // heading at top
    draw_text(frame, width, height, width / 2 - 28 - 4, 2, "Machina", [200, 200, 200, 155]);
}

// drawing helpers
#[inline(always)]
fn set_pixel(frame: &mut [u8], width: usize, height: usize, x: usize, y: usize, color: [u8; 4]) {
    if x >= width || y >= height {
        return;
    }

    let alpha = color[3] as u16;
    if alpha == 0 {
        return;
    }

    let idx = (y * width + x) * 4;

    if alpha == 255 {
        frame[idx..idx + 4].copy_from_slice(&color);
        return;
    }

    let inv_alpha = 255 - alpha;
    frame[idx] = ((color[0] as u16 * alpha + frame[idx] as u16 * inv_alpha) / 255) as u8;
    frame[idx + 1] = ((color[1] as u16 * alpha + frame[idx + 1] as u16 * inv_alpha) / 255) as u8;
    frame[idx + 2] = ((color[2] as u16 * alpha + frame[idx + 2] as u16 * inv_alpha) / 255) as u8;
    frame[idx + 3] = 255;
}

fn draw_rect(frame: &mut [u8], width: usize, height: usize, rx: usize, ry: usize, rw: usize, rh: usize, color: [u8; 4]) {
    for y in ry..(ry + rh) {
        for x in rx..(rx + rw) {
            set_pixel(frame, width, height, x, y, color);
        }
    }
}

fn draw_text(frame: &mut [u8], width: usize, height: usize, x: usize, y: usize, text: &str, color: [u8; 4]) {
    let mut cursor_x = x;
    for ch in text.chars() {
        if let Some(glyph) = BASIC_FONTS.get(ch) {
            for (row_idx, &row_bits) in glyph.iter().enumerate() {
                for col_idx in 0..8 {
                    if (row_bits >> col_idx) & 1 == 1 {
                        set_pixel(frame, width, height, cursor_x + col_idx, y + row_idx, color);
                    }
                }
            }
        }
        cursor_x += 8;
    }
}