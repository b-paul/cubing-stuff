mod matrix;
mod pins;

fn main() {
    for order in pins::PinSet::all().map(|s| s.into_pin_order()) {
        println!("{} {}", order, order.as_matrix().try_inverse().unwrap());
    }
}
