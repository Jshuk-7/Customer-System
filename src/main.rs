mod customer;

use customer::*;

fn main() {
    let mut cus = Customer::default();
    cus.set_name("Jack");

    let mut order = Order::new(
        "Jack",
        25,
        10,
        "12/22/22"
    );

    let order2 = Order::new(
        "Jack",
        25,
        10,
        "12/22/22"
    );

    cus.set_last_order(order2);
    order.void_order();
    println!("{:#?}", order);
}
