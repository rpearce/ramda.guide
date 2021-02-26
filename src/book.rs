use mdbook::MDBook;
use std::io;

pub fn load(src: &str) -> Result<MDBook, io::Error> {
    let book = MDBook::load(src).expect("Failed to load book data");
    book.build().expect("Failed to create book");
    println!("Wrote {:#?}", "./web/book");

    Ok(book)
}
