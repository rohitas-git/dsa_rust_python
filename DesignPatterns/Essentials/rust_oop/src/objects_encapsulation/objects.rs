/**========================================================================
*                           Objects and Encapsulation
*========================================================================**/
// Objects Contain Data and Behavior

// Object-oriented programs are made up of objects. 
// An object packages both data and the procedures that operate on that data.
// The procedures are typically called methods or operations.

// AveragedCollection is an object -> data (struct) and procedures (impl block)
// Data is encapsulated with struct 
// as its fields of struct are private and can only be operated through its public methods

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

