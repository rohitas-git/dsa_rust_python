use std::io::Empty;

use Element::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Element{
    Some(u32),
    Deleted,
    None
}

#[derive(Default, Debug)]
struct MyHash {
    table: Vec<Element>,
    size: usize
}

trait HashTable {

    fn new(capacity:usize) -> Self;
    fn size(&self)-> usize;
    fn capacity(&self)->usize;
    fn hash(&self, target: u32) -> usize;
    fn get(&self, index: usize)-> Option<&Element>;
    fn search(&self, target: u32) -> bool;
    fn insert(&mut self, item: u32) -> bool;
    fn delete(&mut self, item: u32) -> bool;
}

impl HashTable for MyHash{

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

    fn get(&self, index: usize)->Option<&Element> {
        let cap = self.capacity();
        let index = index % cap;
        self.table.get(index)
    }

    fn search(&self, target: u32) -> bool {
        let cap = self.capacity();
        let table = &self.table;
        let hash = self.hash(target);
        let mut index = hash;

        while table[index] != None{
            if table[index] == Some(target){
                return true;
            }
            index = (index + 1)% cap; // Linear Probing
            if index == hash{
                return false;
            } 
        }   
        false
    }

    fn insert(&mut self, item: u32) -> bool {
        if self.capacity() == self.size() {
            return false;
        }
        if self.search(item) == true{
            return false;
        }
        let mut i = self.hash(item);
        let cap = self.capacity();
        let mut table = &mut self.table;

        while table[i] != Deleted && table[i] != None {
            i = (i+1) % cap; // Linear Probing
        }
        table[i] = Some(item);
        self.size +=1;
        true
    }

    fn delete(&mut self, item: u32) -> bool {
        let hash = self.hash(item);
        let size = self.size;
        let cap = self.capacity();
        let table = &mut self.table;
        let mut i = hash;

        while table[i] != None {
            if table[i] == Some(item){
                table[i] = Deleted;
                self.size = size - 1;
                return true;
            }
            i = (i+1) % cap;
            if i == hash{
                return false;
            }
        }
        false
    }

}

impl PartialEq for MyHash{

    fn eq(&self, other: &Self) -> bool {
        self.table == other.table
    }

}

#[cfg(test)]
mod test_my_hashtable {
    use super::*;

    #[test]
    fn test_new_impl() {
        let t = MyHash::new(7);
        let r = MyHash{table: vec![None;7], size:0};
        assert_eq!(t, r);
        assert_eq!(t.capacity(), r.capacity());
        assert_eq!(t.size(), r.size());
    }

    #[test]
    fn test_search_impl() {
        let t = MyHash{table: vec![None,None,None,Some(10),None,None,None], size:0};
        
        let res = t.search(10);
        assert_eq!(res, true);

        let res = t.search(17);
        assert_eq!(res, false);
    }

    #[test]
    fn test_insert_impl() {
        let mut t = MyHash::new(7);
        
        assert_eq!(t.insert(10), true);
        let r = MyHash{table: vec![None,None,None,Some(10),None,None,None],size:0};
        assert_eq!(t, r);

        assert_eq!(t.insert(17), true);
        let r = MyHash{table: vec![None,None,None,Some(10),Some(17),None,None],size:0};
        assert_eq!(t, r);

        assert_eq!(t.insert(1), true);
        assert_eq!(t.insert(21), true);
        assert_eq!(t.insert(21), false); // Already inserted
        assert_eq!(t.insert(14), true);
        assert_eq!(t.insert(15), true);
        assert_eq!(t.insert(27), true);
        assert_eq!(t.insert(12), false); // Filled
    }

    #[test]
    fn test_delete_impl() {
        let mut t = MyHash::new(7);
        t.insert(10);
        t.insert(2);
        t.insert(3);         
        t.insert(4);
        t.insert(5); 
        t.insert(6);
        t.insert(7);

        assert_eq!(t.delete(10), true);
        assert_eq!(t.delete(2), true);
        assert_eq!(t.delete(2), false);
        assert_eq!(t.delete(99), false);
        println!("{:?}", t);
    }
}