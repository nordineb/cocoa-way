use smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer;
use smithay::wayland::shm::with_buffer_contents;
#[allow(dead_code)]
pub fn render_surface(
    buffer: &WlBuffer,
    canvas: &mut [u32],
    canvas_width: u32,
    canvas_height: u32,
    x_offset: i32,
    y_offset: i32,
) {
    let _ = with_buffer_contents(buffer, |ptr, len, data| {
        log::info!(
            "Rendering buffer data: {}x{}, stride: {}",
            data.width,
            data.height,
            data.stride
        );
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
        let width = data.width;
        let height = data.height;
        let stride = data.stride;
        if width == 0 || height == 0 {
            return;
        }
        for y in 0..height {
            let canvas_y = y_offset + y;
            if canvas_y < 0 || canvas_y >= canvas_height as i32 {
                continue;
            }
            let src_base = (y * stride) as usize;
            let dst_base = (canvas_y as u32 * canvas_width) as usize;
            for x in 0..width {
                let canvas_x = x_offset + x;
                if canvas_x < 0 || canvas_x >= canvas_width as i32 {
                    continue;
                }
                let pixel_offset = src_base + (x * 4) as usize;
                if pixel_offset + 4 > slice.len() {
                    break;
                }
                let b = slice[pixel_offset];
                let g = slice[pixel_offset + 1];
                let r = slice[pixel_offset + 2];
                let _a = slice[pixel_offset + 3];
                let color = 0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                let dst_idx = dst_base + canvas_x as usize;
                if dst_idx < canvas.len() {
                    canvas[dst_idx] = color;
                }
            }
        }
    });
}
pub fn get_buffer_pixels(buffer: &WlBuffer) -> Option<(i32, i32, Vec<u8>)> {
    with_buffer_contents(buffer, |ptr, len, data| {
        let width = data.width;
        let height = data.height;
        let stride = data.stride;
        if width == 0 || height == 0 {
            return None;
        }
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
        let mut pixels = Vec::with_capacity((width * height * 4) as usize);
        for y in 0..height {
            let src_base = (y * stride) as usize;
            for x in 0..width {
                let pixel_offset = src_base + (x * 4) as usize;
                if pixel_offset + 4 <= slice.len() {
                    pixels.push(slice[pixel_offset]);
                    pixels.push(slice[pixel_offset + 1]);
                    pixels.push(slice[pixel_offset + 2]);
                    pixels.push(slice[pixel_offset + 3]);
                } else {
                    pixels.extend_from_slice(&[0, 0, 0, 255]);
                }
            }
        }
        Some((width, height, pixels))
    })
    .ok()
    .flatten()
}
