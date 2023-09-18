trait Show {
    fn show(&self) -> String;
}

trait Location {
    fn location(&self) -> String;
}

trait ShowTell: Show + Location {}

/*========================================================================*/

#[derive(Debug)]
struct Foo {
    name: String,
    location: String
}

impl Foo {
    fn new(name: &str, location: &str) -> Foo {
        Foo{
            name: name.to_string(),
            location: location.to_string()
        }
    }
}

impl Show for Foo {
    fn show(&self) -> String {
        self.name.clone()
    }
}

impl Location for Foo {
    fn location(&self) -> String {
        self.location.clone()
    }
}

impl ShowTell for Foo {}

/*=============================================*/
macro_rules! dbg {
    ($x:expr) => {
        println!("{} = {:?}",stringify!($x),$x);
    }
}
/*========================================================================*/

pub fn rust_oop(){
    let foo = Foo::new("Pete","bathroom");
    dbg!(foo.show());
    dbg!(foo.location());
    
    let st: &dyn ShowTell = &foo;
    
    dbg!(st.show());
    dbg!(st.location());
    
    fn show_it_all(r: &dyn ShowTell) {
        dbg!(r.show());
        dbg!(r.location());
    }
    
    let boo = Foo::new("Alice","cupboard");
    show_it_all(&boo);
    
    fn show(s: &dyn Show) {
        dbg!(s.show());
    }
    
    show(&boo);
}

// foo.show() = "Pete"
// foo.location() = "bathroom"
// st.show() = "Pete"
// st.location() = "bathroom"
// r.show() = "Alice"
// r.location() = "cupboard"
// s.show() = "Alice"