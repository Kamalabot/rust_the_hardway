#[allow(dead_code)]
#[allow(unused_variables)]

struct Book {
    pages: i32,
    price: i32,
}

fn display_price(bk1: &Book){
    println!("Price of book is {}", bk1.price);
}

fn display_pages(bk1: &Book){
    println!("Pages in book is {}", bk1.pages);
}

fn main(){
    let bk = Book{
        pages:57,
        price:75
    };
    display_pages(&bk);
    display_price(&bk);
}