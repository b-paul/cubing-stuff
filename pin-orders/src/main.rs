pub mod matrix;
pub mod pins;
pub mod z12;

fn main() {
    for pin_set in pins::PinSet::all() {
        use itertools::Itertools;
        let (_, order) = pin_set
            .0
            .into_iter()
            .permutations(7)
            .map(pins::PinOrder)
            .map(|o| (o.count_transitions() as i32, o))
            .sorted_by_key(|(n, _)| -(*n))
            .next()
            .unwrap();
        let mut lock = std::io::stdout().lock();
        order.make_tutorial(&mut lock, order.gen_memo()).unwrap();
    }
    /*
    use pins::PinConfiguration as P;
    let order = pins::PinOrder(vec![P::NDL, P::R, P::DR, P::NUR, P::L, P::UL, P::BSLASH]);
    let mut lock = std::io::stdout().lock();
    order.make_tutorial(&mut lock, order.gen_memo()).unwrap();
    */
}
