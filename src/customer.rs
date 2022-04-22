#[derive(Debug)]
pub struct Customer {
    pub name: String,
    pub phone_number: String,
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
    // TODO pub fn new(name: &str, phone_number: &str, last_order: Order) -> Self

    pub fn get_last_order(&self) -> &Order {
        &self.last_order
    }
}

impl Order {
    // TODO pub fn new(customer: &str, total: u32, pieces: u32, due_date: &str) -> Self
    // TODO pub fn void_order(&mut self)
}