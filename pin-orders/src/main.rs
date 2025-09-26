pub mod matrix;
pub mod pins;
pub mod z12;

fn main() {
    use itertools::Itertools;
    use rayon::prelude::*;
    let sols = pins::PinSet::all()
        .flat_map(|pin_set| {
            let mut all = Vec::new();
            pin_set
                .0
                .into_iter()
                .permutations(7)
                .map(pins::PinOrder)
                .flat_map(|po| (0..7).map(move |i| pins::FlipPinOrder(po.clone(), i)))
                //.filter(|fpo| fpo.no_d_moves())
                .collect_vec()
                .into_par_iter()
                .map(|o| (o.gen_memo(), o.0.count_transitions() as i32, o))
                .collect_into_vec(&mut all);
            all.into_iter()
        })
        .sorted_by_key(|(memo, n, _)| {
            (
                memo.iter()
                    .map(|m| match m {
                        pins::MoveSolution::Memo(m) => m.iter().filter(|&&n| n != 0).count(),
                        _ => 0,
                    })
                    .sum::<usize>(),
                -(memo
                    .iter()
                    .filter(|m| !matches!(m, pins::MoveSolution::Memo(_)))
                    .count() as isize),
                *n,
            )
        });

    for (memo, _, order) in sols {
        let mut lock = std::io::stdout().lock();
        order.make_tutorial(&mut lock, memo).unwrap();
    }
}
