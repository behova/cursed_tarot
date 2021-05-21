//use std::fs::File;
mod nomer;
mod card;

#[derive(Debug)]
pub struct Deck {
    pub cards:Vec<std::ffi::OsString>,
} 

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


//todo how to pass in card object to nom so that we only need one buffer??
impl Deck {
    pub fn draw_card(&mut self) -> card::Card {
        let card_path = match self.cards.pop() {
            Some(c) => c,
            None => panic!("couldn't draw card") 
        };

        let (card_title, img_data) = nomer::convert_to_vectors(card_path);

        let new_card = card::Card::new(card_title, img_data);

        new_card

    }
}