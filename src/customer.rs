#![allow(dead_code)]

#[derive(Debug)]
pub struct Customer {
    pub name: String,
    pub phone_number: String,
    pub special: bool,
    pub last_order: Order,
}

#[derive(Debug)]
pub struct Order {
    pub customer: String,
    pub total: u32,
    pub pieces: u32,
    pub due_date: String,
}

impl Customer {
    pub fn new(name: &str, phone_number: &str, special: bool, last_order: Order) -> Self {
        Self {
            name: String::from(name),
            phone_number: String::from(phone_number),
            special: special,
            last_order: last_order,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_phone_number(&self) -> &str {
        &self.phone_number
    }

    pub fn get_last_order(&self) -> &Order {
        &self.last_order
    }

    pub fn set_name(&mut self, new_value: &str) {
        self.name = String::from(new_value);
    }

    pub fn set_phone_number(&mut self, new_value: &str) {
        self.phone_number = String::from(new_value);
    } 

    pub fn set_last_order(&mut self, new_value: Order) {
        self.last_order = new_value
    }
}

impl Default for Customer {
    fn default() -> Self {
        Self {
            name: String::new(),
            phone_number: String::new(),
            special: false,
            last_order: Order::default(),
        }
    }
}

impl Order {
    pub fn new(customer: &str, total: u32, pieces: u32, due_date: &str) -> Self {
        Self {
            customer: String::from(customer),
            total: total,
            pieces: pieces,
            due_date: String::from(due_date),
        }
    }

    pub fn void_order(&mut self) {
        drop(self);
    }
}

impl Default for Order {
    fn default() -> Self {
        Self {
            customer: String::new(),
            total: 0,
            pieces: 0,
            due_date: String::new(),
        }
    }
}