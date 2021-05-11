
use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;
use nom::{
    IResult,
    sequence::{delimited, terminated, tuple},
    multi::many0,
    bytes::complete::{tag, take_until},
    combinator::opt,
};

fn title(i: &str) -> IResult<&str, std::option::Option::<&str>> {
    opt(delimited(tag("[title="), take_until("]"), tag("]")))(i)
}

fn color_tag(i: &str) -> IResult<&str, &str> {
    delimited(tag("[color="), take_until("]"), tag("]"))(i)
}

fn char_take(i: &str) -> IResult<&str, &str> {
    terminated(take_until("[/color]"), tag("[/color]"))(i)
}


fn color_char(i: &str) -> IResult<&str, (&str, &str)> {
    tuple((color_tag, char_take))(i)
}

fn color_char_multiple(i: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many0(color_char)(i)
}

//utilities for interfacing and formating image data

fn convert_nom(i: &str) -> IResult<&str, Vec<(String, String)>>{
    let (i, output) = color_char_multiple(i)?;
    let output_conv:Vec<_> = output.iter().map(|(x, y)| (x.to_string(), y.to_string())).collect();
    Ok((i, output_conv))
}


fn convert_path(path: std::ffi::OsString) -> File{
    let file = match File::open(path){
        Ok(o) => o,
        Err(e) => panic!("bad .txt file {:?}", e),
    };
    file
}

//TODO combine these two functions to a single buffer lol

pub fn get_title(path: std::ffi::OsString) -> String {
    let file = self::convert_path(path);
    let buf = BufReader::new(file);
    let mut card_title = String::new();

    for line in buf.lines() {
        let l = match line {
            Ok(o) => o,
            Err(e) => panic!("bad file conversion {:?}", e)
        };

        let (_, output) = match self::title(&l) {
            Ok(o) => o,
            Err(e) => panic!("bad file conversion nom {:?}", e)
        };

        //set title variable if it exists
        match output {
            Some(o) => card_title = o.to_string(),
            None => (),
        };

    }
    
    card_title
}

//pass in a card struct to this function and it will return it filled (TODO)
pub fn convert_to_vectors(path: std::ffi::OsString) -> Vec<Vec<(String, String)>> {
    let file = self::convert_path(path);
    let buf = BufReader::new(file);
    let mut mvec = Vec::new();
    //let mut card_title = String::new();

    for line in buf.lines() {
        //buffer error
        let l = match line {
            Ok(o) => o,
            Err(e) => panic!("bad file conversion {:?}", e)
        };

        //nom img data error
        let (_, output) = match self::convert_nom(&l) {
            Ok(o) => o,
            Err(e) => panic!("bad file conversion nom {:?}", e)
        };

        // //nom title error
        // let (i, t) = match self::title(i) {
        //     Ok(o) => o,
        //     Err(e) => panic!("no title for card {:?}", e)
        // };
        
        // //set title variable if it exists
        // match t {
        //     Some(o) => card_title = o.to_string(),
        //     None => (),
        // };

        mvec.push(output);
         
    }

    mvec
    
}


  
  