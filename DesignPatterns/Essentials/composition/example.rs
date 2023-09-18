/*
interface Animal {
    void tell();
    void pet();
    void feed(Food food);
}

class Cat implements Animal {
    public void tell() { System.out.println("Meow"); }
    public void pet() { System.out.println("purr"); }
    public void feed(Food food) { System.out.println("lick"); }
}

Here: Implementation Inheritance [ Lion inherits Cat's impl of interface functions ]
class Lion extends Cat {
    public void tell() { System.out.println("Roar"); }
}
*/

/*================================ DIVISION ==============================*/


// Part that can be impl in Rust
trait Animal {
    fn tell(&self);
    fn pet(&mut self);
    fn feed(&mut self, food: Food);
}

struct Cat;

impl Animal for Cat {
    fn tell(&self) { println!("Meow"); }
    fn pet(&mut self) { println!("purr");
    fn feed(&mut self, food: Food) { println!("lick"); }
}

/*=============================================*/

// Not easy to impl in Rust
struct Lion;

impl Animal for Lion {
    fn tell(&self) { println!("Roar"); }
    // Error: Missing methods pet and feed
}

/*========================================================================*/

struct Lion;

impl Animal for Lion {
    fn tell(&self) { println!("Roar"); }
    // Error: Missing methods pet and feed
}
The simplest way is, obviously, to duplicate the methods. Yes, duplication is bad. So is complexity. Create a free method and call that from the Cat and Lion impl if you need to deduplicate the code.

But wait, what about the polymorphism part of the equation? That’s where it gets complicated. Where OO languages usually give you dynamic dispatch, Rust makes you choose between static and dynamic dispatch, and both have their costs and benefits.

// static dispatch
let cat = Cat;
cat.tell();

let lion = Lion;
lion.tell();

// dynamic dispatch via enum
enum AnyAnimal {
   Cat(Cat),
   Lion(Lion),
}

// `impl Animal for AnyAnimal` left as an exercise for the reader

let animals = [AnyAnimal::Cat(cat), AnyAnimal::Lion(lion)];
for animal in animals.iter() {
   animal.tell();
}

// dynamic dispatch via "fat" pointer including vtable
let animals = [&cat as &dyn Animal, &lion as &dyn Animal];
for animal in animals.iter() {
   animal.tell();
}