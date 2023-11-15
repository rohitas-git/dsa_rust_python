trait Deque {
    type Data;
    fn insert_front(&mut self, key: Data);
    fn insert_rear(&mut self, key: Data);
    
    fn get_front(&self);
    fn get_rear(&self);

    fn delete_front(&mut self);
    fn delete_rear(&mut self);
}


mod list_impl {

    struct Node {}
}
