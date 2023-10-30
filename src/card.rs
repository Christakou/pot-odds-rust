pub mod card {
    pub struct Card {
        suit:String,
        rank:u8,
    }

    impl Card {
        fn print(&self){
            print!("{}{}",self.rank,self.suit);
        }
    }
}