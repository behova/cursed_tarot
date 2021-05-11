#[derive(Debug)]
pub struct Card {
    
    pub title: String,
    pub img_data: Vec<Vec<(String, String)>>

}

impl Card {
    pub fn new(title: String, img_data: Vec<Vec<(String, String)>> ) -> Self {
        Self {
            
            title: title,
            img_data: img_data,
        }

    }

}