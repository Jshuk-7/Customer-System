mod customer;

use customer::*;

fn main() {
    let cus = Customer{
        name: String::from("Jack"),
        phone_number: String::from("248-298-2367"),
        last_order: Order{
            due_date: String::from("12/22/22"),
            customer: String::from("Jack"),
            total: 25,
            pieces: 10,
        }
    };

    println!("{:#?}", cus);
    println!("{:#?}", cus.get_last_order());
}
