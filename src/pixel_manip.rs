use crate::wasm4::*;

pub fn pixel(x: u16, y: u16) {
    // The byte index into the framebuffer that contains (x, y)
    let idx = ((y * 160 + x) >> 2) as usize;
    // trace(format!("{:?}", (x, y)));

    // Calculate the bits within the byte that corresponds to our position
    let shift = (x & 0b11) << 1;
    let mask = 0b11 << shift;

    unsafe {
        let palette_color: u8 = (*DRAW_COLORS & 0xf) as u8;
        if palette_color == 0 {
            // Transparent
            return;
        }
        let color = (palette_color - 1) & 0b11;

        let framebuffer = &mut *FRAMEBUFFER;

        framebuffer[idx] = (color << shift) | (framebuffer[idx] & !mask);
    }
}
