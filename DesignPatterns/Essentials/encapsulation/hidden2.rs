#[path = "./hidden.rs"]
mod hidden;
use hidden::*;

fn main() {
    let open_box = OpenBox {
        contents: "public information",
    };
    
    println!("The open box contains: {}", open_box.contents);

    let _closed_box = ClosedBox::new("classified information");

    ClosedBox::print_content(&_closed_box);
    _closed_box.print_content();
}
