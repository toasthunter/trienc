use std::fs;

mod triconv;
use triconv::text_to_trinary;

mod genimg;
use genimg::draw_img;

fn main() {

   let input = fs::read_to_string("input.txt").expect("Failed to read input");
    
   let tri = text_to_trinary(input);

    if let Err(e) = draw_img(tri) {
        println!("Error: {}", e);
    }

}
