fn main() {
    let mut book = Book{
        title: String::from("The Batman"),
        author: String::from("idk :("),
        publication_year: 1990,
    };

    book.publication_year = 1995;

    println!(
        "The book {} is written by {} in {}",
        book.title, book.author, book.publication_year
    );

    let book_data = get_book_data(book);
    for data in book_data{
        println!("{data}");
    }

    let book2 = create_book("Hero".to_string(),"Adil".to_string(),2011);
  
    println!("My book is {:?}",book2);

    let tuple_book = Tuple_Book("Some Book".to_string(), "Some author".to_string(), 2029);

}

#[derive(Debug)]

// Structs

struct Book{
    title: String,
    author: String,
    publication_year: u32,
}

struct Tuple_Book(String, String, u32);

struct Unit_Book;

fn get_book_data(book: Book) -> [String; 3] {
    let title = book.title;
    let author = book.author;
    let publication_year = book.publication_year;

    let data: [String; 3] = [title, author, publication_year.to_string()];
    data
}

fn create_book(title: String, author: String, publication_year: u32) -> Book {
    let book = Book {
        title, author, publication_year,
    };
    book
}

struct Rectangle{
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
