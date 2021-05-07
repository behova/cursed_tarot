
mod nomer;
mod deck;
//use colorsys::{Rgb, Hsl, ColorAlpha};
use cursive::theme::{Color, ColorStyle};
use cursive::view::Boxable;
use cursive::views::Canvas;
use cursive::Printer;
use unicode_segmentation::UnicodeSegmentation;
// use colorsys;
// use std::io::{BufReader};
// use std::io::prelude::*;
// use std::fs::File;


fn main() {

    let mut deck_test = deck::Deck::new();
    let card = deck_test.draw_card();
    let img = nomer::convert_to_vectors(card);

    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(Canvas::new(()).with_draw(move |&(), printer| { draw(&(), printer, img.clone())}).fixed_size((80, 40)));
    

    siv.run();

}


pub fn draw(_: &(), p:&Printer, vecs: Vec<Vec<(String, String)>>) {
    
    let mut y:u8 = 0;

    for line in vecs {

        let mut x:u8 = 0;

        for tag in line {
            
            let rgb = match colorsys::Rgb::from_hex_str(&tag.0) {
                Ok(o) => o,
                Err(e) => panic!("err {:?}", e)
            };
        
            let r = rgb.red() as u8;
            let b = rgb.blue() as u8;
            let g = rgb.green() as u8;

            //println!("{:?}, {:?}, {:?}", r, g, b);

            let style = ColorStyle::new(
                Color::Rgb(r,b,g), 
                Color::Rgb(0, 0, 0),
            );

            p.with_color(style, |printer| {
                printer.print((x, y), &tag.1);
            });

            //let increment = tag.1.as_str();
            //let s = "pdopfdfs";

            x += tag.1.graphemes(true).count() as u8;
            //x += 1;
                
        }
        
        y += 1;
        
    }

}
