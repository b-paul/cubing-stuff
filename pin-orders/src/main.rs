pub mod matrix;
pub mod pins;
pub mod z12;

fn main() {
    use itertools::Itertools;
    use rayon::prelude::*;
    let sols = pins::PinSet::all().map(|pin_set| {
        let mut all = Vec::new();
        pin_set
            .0
            .into_iter()
            .permutations(7)
            .collect_vec()
            .into_par_iter()
            .map(pins::PinOrder)
            .map(|o| (o.gen_memo(), o.count_transitions() as i32, o))
            .collect_into_vec(&mut all);
        let (memo, _, order) = all
            .into_iter()
            .sorted_by_key(|(memo, n, _)| {
                (
                    *n,
                    -(memo
                        .iter()
                        .filter(|m| !matches!(m, pins::MoveSolution::Memo(_)))
                        .count() as isize),
                    memo.iter()
                        .map(|m| match m {
                            pins::MoveSolution::Memo(m) => m.iter().filter(|&&n| n != 0).count(),
                            _ => 0,
                        })
                        .sum::<usize>(),
                )
            })
            .next()
            .unwrap();
        (memo, order)
    });

    for (memo, order) in sols {
        let mut lock = std::io::stdout().lock();
        order.make_tutorial(&mut lock, memo).unwrap();
    }
}
