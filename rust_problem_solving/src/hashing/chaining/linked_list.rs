
// #[derive(Clone, PartialEq, Debug)]
// pub enum Element{
//     Some(u32),
//     Deleted,
//     None
// }

#[derive(Default, Debug)]
struct MyHashTable {
    table: Vec<u32>,
    size: usize
}

trait HashTable {

    fn new(capacity:usize) -> Self;
    fn size(&self)-> usize;
    fn capacity(&self)->usize;
    fn hash(&self, target: u32) -> usize;
    fn get(&self, index: usize)-> Option<u32>;
    fn search(&self, target: u32) -> bool;
    fn insert(&mut self, item: u32) -> bool;
    fn delete(&mut self, item: u32) -> bool;
}

trait Chaining: HashTable{

    fn insert(&mut self, item: u32) -> bool;
    fn get(&self, index: usize)-> Option<u32>;
    fn search(&self, target: u32) -> bool;
    fn insert(&mut self, item: u32) -> bool;
    fn delete(&mut self, item: u32) -> bool;
}

impl HashTable for MyHashTable{

    fn new(capacity:usize) -> Self{
        // let table: Vec<Element> = Vec::with_capacity(capacity);
        let table = vec![None;capacity];
        Self{
            table,
            size: 0
        }
    }

    fn capacity(&self)->usize {
        self.table.capacity()
    }

    fn size(&self)->usize {
        self.size
    }

    fn hash(&self, target: u32) -> usize {
        target as usize % self.capacity()
    }
}