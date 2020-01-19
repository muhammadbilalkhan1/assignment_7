use bookshop::science_books;
fn main() {
    science_books::sold();
    
}
pub mod bookshop{
    pub mod science_books{
        pub fn sold(){
            println!("Hello, world!");
        }
    }
}