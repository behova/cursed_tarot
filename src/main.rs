
mod nomer;
mod deck;
use colorsys::{Rgb, Hsl, ColorAlpha};
use cursive::theme::{Color, ColorStyle};
// use cursive::view::Boxable;
// use cursive::views::Canvas;
use cursive::Printer;
// use colorsys;
// use std::io::{BufReader};
// use std::io::prelude::*;
// use std::fs::File;

fn main() {

    let mut deck_test = deck::Deck::new();
    let card = deck_test.draw_card();
    
    let img = nomer::convert_to_vectors(card);

    draw(img);

}

fn draw(_: &(), p:&Printer, vecs: Vec<Vec<(String, String)>>) {

    for line in vecs {
        for tag in line {
            
            let rgb = match colorsys::Rgb::from_hex_str(&tag.0) {
                Ok(o) => o,
                Err(e) => panic!("err {:?}", e)
            };
        
            let r = rgb.red() as u8;
            let b = rgb.blue() as u8;
            let g = rgb.green() as u8;

            println!("{:?}, {:?}, {:?}", r, g, b);

            let style = ColorStyle::new(
                Color::Rgb(r,b,g), 
                Color::Rgb(0,0,0),
            );

            p.with_color(style, |printer| {
                printer.print((x, y), &tag.1);
            });
            
            
        }

    }

}
