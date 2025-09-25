pub mod matrix;
pub mod pins;

fn main() {
    for pin_set in pins::PinSet::all() {
        use itertools::Itertools;
        let (count, order) = pin_set
            .0
            .into_iter()
            .permutations(7)
            .map(pins::PinOrder)
            .map(|o| (o.count_transitions() as i32, o))
            .sorted_by_key(|(n, _)| -(*n))
            .next()
            .unwrap();
        println!("{order} {count}");
    }
}
