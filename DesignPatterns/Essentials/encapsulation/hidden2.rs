#[path = "./hidden.rs"]
mod hidden;
use hidden::*;

fn main() {
    let open_box = OpenBox {
        contents: "public information",
    };
    
    println!("The open box contains: {}", open_box.contents);

    let mut _closed_box = ClosedBox::new("classified information");

    _closed_box.set_content("secret information");
    println!("The close box contains: {:?}", _closed_box.get_content());
}
