const PIXEL_WIDTH: i32 = 50;
const LINE_LIMIT: i32 = 50;

extern crate raqote;
use raqote::*;

pub fn draw_img(trinums: Vec<[u8; 3]>) -> Result<(), Box<dyn std::error::Error>> {

    let lines = trinums.len() as i32 / (LINE_LIMIT + 1) + 1;

    let mut dt = DrawTarget::new(if trinums.len() < LINE_LIMIT as usize {trinums.len() as i32 * PIXEL_WIDTH} else {LINE_LIMIT * PIXEL_WIDTH}, lines * PIXEL_WIDTH * 3);

    let mut line_offset = 0;
    let mut x_offset = 0;
    let mut curr_characters_in_line = 0;

    for num in trinums.into_iter() {

        for (y_offset, &sqr) in num.iter().enumerate() {

            let src = Source::Solid(SolidSource {
                r: if sqr == 0u8 {0xFF} else {0x0},
                g: if sqr == 1u8 {0xFF} else {0x0},
                b: if sqr == 2u8 {0xFF} else {0x0},
                a: 0xFF
            });

            dt.fill_rect((x_offset as i32 * PIXEL_WIDTH) as f32, (y_offset as i32 * PIXEL_WIDTH + line_offset) as f32, PIXEL_WIDTH as f32, PIXEL_WIDTH as f32, &src, &DrawOptions::new());

        }

        if curr_characters_in_line == LINE_LIMIT - 1 {
            line_offset += 3 * PIXEL_WIDTH;
            x_offset = 0;
            curr_characters_in_line = 0;
        } else {
            curr_characters_in_line += 1;
            x_offset += 1;
        }
        
    }

    dt.write_png("output.png")?;

    Ok(())

}