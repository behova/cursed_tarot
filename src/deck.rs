//use std::fs::File;

#[derive(Debug)]
pub struct Deck {
    pub cards:Vec<std::ffi::OsString>,
} 

//add path to card files to call from main
impl Deck {
    pub fn new() -> Self {
        Self {
            cards:Deck::get_cards(),
        }

    }

}

impl Deck {
    fn get_cards() -> Vec<std::ffi::OsString>{
        let folder = "src/card_files/trumps/";

        let mut card_files = Vec::new();
    
        let paths = match std::fs::read_dir(folder) {
            Ok(p) => p,
            Err(e) => panic!("bad directory path {:?}", e)
        };

        for path in paths {
            match path {
                Ok(p) => card_files.push(p.path().into_os_string()),
                Err(e) => panic!("bad file path {:?}", e) 
            }
        }

        fastrand::shuffle(&mut card_files);
        card_files
    }
}

impl Deck {
    pub fn draw_card(&mut self) -> std::ffi::OsString {
        let card = match self.cards.pop() {
            Some(c) => c,
            None => panic!("couldn't draw card") 
        };

        card    
    }
}
