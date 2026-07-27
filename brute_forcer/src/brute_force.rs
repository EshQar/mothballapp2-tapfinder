
use crate::goals::{RotationGoal, NoRotationGoal};
use crate::conglomerate::{Conglomerate, RotationConglomerate, NoRotationConglomerate};
use crate::iterators::MaxCountIter;

pub fn no_facing_bf<C, G>(
    n: usize,
    strat_count: usize,
    max_counts: Vec<usize>,
    mut conglomerate: C,
    mut goals: Vec<G>,
) -> (Vec<Vec<Vec<Vec<isize>>>>, Vec<Vec<C::Offset>>, Vec<Vec<G::Dist>>)
where
    C: NoRotationConglomerate,
    G: NoRotationGoal<C::Offset>,
//        <C as Conglomerate>::Offset: std::fmt::Debug
{

    let len_goals = goals.len();
    let mut successes = vec![Vec::new(); len_goals];
    let mut strat_offsets: Vec<Vec<C::Offset>> = vec![Vec::new(); len_goals];
    let mut dists: Vec<Vec<G::Dist>> = (0..len_goals).map(|_| Vec::new()).collect();
    let mut found_strats = 0_usize;

    'outer: for (sum, counts) in MaxCountIter::new(&max_counts, n) {
        conglomerate.update_max_counts(&counts);
        loop {
            let curr_offset = conglomerate.offset();
            for (i, goal) in goals.iter_mut().enumerate() {
                if goal.is_satisfied(&curr_offset) {
                    successes[i].push(conglomerate.weight_state().iter().map(|s| s.to_vec()).collect());
                    dists[i].push(goal.get_dists(&curr_offset));
                    
                    strat_offsets[i].push(curr_offset.clone());

                    found_strats += 1;
                    if strat_count <= found_strats {
                        break 'outer;
                    }
                }
            }

            if conglomerate.advance() {
                break;
            }
        }
    }
    (successes, strat_offsets, dists)
}

pub fn facing_bf<C, G>(
    n: usize,
    strat_count: usize,
    max_counts: Vec<usize>,
    mut conglomerate: C,
    mut goals: Vec<G>,
) -> (Vec<Vec<Vec<Vec<isize>>>>, Vec<Vec<G::Pos>>, Vec<Vec<G::Dist>>, Vec<Vec<Vec<usize>>>)
where
    C: RotationConglomerate,
    G: RotationGoal<C::Offset>,
    G::Pos: Clone + Copy,
        <C as Conglomerate>::Offset: std::fmt::Debug
{

    let len_goals = goals.len();
    let mut successes = vec![Vec::new(); len_goals];
    let mut strat_offsets: Vec<Vec<G::Pos>> = vec![Vec::new(); len_goals];
    let mut dists: Vec<Vec<G::Dist>> = (0..len_goals).map(|_| Vec::new()).collect();
    let mut facings: Vec<Vec<Vec<usize>>> = vec![Vec::new(); len_goals];
    let mut found_strats = 0_usize;

    'outer: for (sum, counts) in MaxCountIter::new(&max_counts, n) {
        conglomerate.update_max_counts(&counts);
        loop {
                let curr_offset = conglomerate.offset();

                for (i, goal) in goals.iter_mut().enumerate() {
                    if goal.is_satisfied(&curr_offset) {
                        let data = goal.fetch_best_tap_data(&curr_offset);
                        successes[i].push(conglomerate.weight_state().iter().map(|s| s.to_vec()).collect());
                        dists[i].push(data.0);
                        
                        strat_offsets[i].push(data.1);
                        facings[i].push(data.2);

                        found_strats += 1;
                        if strat_count <= found_strats {
                            break 'outer;
                        }
                    }
                }

            if conglomerate.advance() {
                break;
            }
        }
    }
    (successes, strat_offsets, dists, facings)
}