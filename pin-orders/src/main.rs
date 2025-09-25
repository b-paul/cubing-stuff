mod matrix;
mod pins;

fn main() {
    for pin_set in pins::PinSet::all() {
        println!("{}", pin_set.into_pin_order());
    }
}
