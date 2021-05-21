
mod deck;
//use colorsys::{Rgb, Hsl, ColorAlpha};
use cursive::theme::{Color, ColorStyle};
use cursive::view::Boxable;
use cursive::views::{Canvas, Dialog};
use cursive::Printer;
use unicode_segmentation::UnicodeSegmentation;
// use colorsys;
// use std::io::{BufReader};
// use std::io::prelude::*;
// use std::fs::File;


fn main() {

    let mut new_deck = deck::Deck::new();
    let card = new_deck.draw_card();
    let img = card.img_data;

    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(Dialog::around(
        Canvas::new(())
        .with_draw(move |&(), printer| { draw(&(), printer, img.clone())})
        .fixed_size((58, 40)))
        .title(card.title));
    

    siv.run();

}


pub fn draw(_: &(), p:&Printer, vecs: Vec<Vec<(String, String)>>) {
    
    let mut y:u8 = 0;
    let amp = "amp";

    for line in vecs {

        let mut x:u8 = 0;

        for tag in line {
            
            let rgb = match colorsys::Rgb::from_hex_str(&tag.0) {
                Ok(o) => o,
                Err(e) => panic!("err {:?}", e)
            };
        
            let r = rgb.red() as u8;
            let g = rgb.green() as u8;
            let b = rgb.blue() as u8;

            let style = ColorStyle::new(
                Color::Rgb(r, g, b), 
                Color::Rgb(0, 0, 0),
            );

            //conditional error checking for escape char.. 
            //TODO implement a match check for '<', '>', '&'

            if tag.1.contains(amp) {
                
                p.with_color(style, |printer| {
                    printer.print((x, y), "A");
                });

                x += 1;

            } else {

                p.with_color(style, |printer| {
                    printer.print((x, y), &tag.1);
                });

                x += tag.1.graphemes(true).count() as u8;

            }
                
        }
        
        y += 1;
        
    }

}
