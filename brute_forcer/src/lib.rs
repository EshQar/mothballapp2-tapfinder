use pyo3::prelude::*;

mod models;

mod xz_models;
use xz_models::{PoolIteratorConglomerate, RotationPoolIteratorConglomerate, XZGoal};

mod x_or_zmodels;
use x_or_zmodels::{SingleAxisPoolIteratorConglomerate, SingleAxisGoal, AxisProjectedGoal};

mod iterators;

#[pymodule]
mod brute_forcer {

    use pyo3::prelude::*;
    use pyo3::FromPyObject;

    use crate::{PoolIteratorConglomerate, RotationPoolIteratorConglomerate, XZGoal, SingleAxisPoolIteratorConglomerate, SingleAxisGoal, AxisProjectedGoal, iterators::MaxCountIter, models::Conglomerate, models::Goal};

    
    pub fn no_facing_range_bf<C, G>(
        n: usize,
        max_counts: Vec<usize>,
        make_conglomerate: impl Fn(Vec<usize>) -> C,
        goals: &[G],
    ) -> (Vec<Vec<Vec<Vec<isize>>>>, Vec<Vec<C::Pos>>, Vec<Vec<G::Dist>>)
    where
        C: Conglomerate,
        G: Goal<C::Pos>,
    {
        let len_goals = goals.len();
        let mut successes = vec![Vec::new(); len_goals];
        let mut strat_offsets: Vec<Vec<C::Pos>> = vec![Vec::new(); len_goals];
        let mut dists: Vec<Vec<G::Dist>> = (0..len_goals).map(|_| Vec::new()).collect();

        for counts in MaxCountIter::new(max_counts.clone(), n) {
            let mut iterator = make_conglomerate(counts);
            loop {
                let curr_pos = iterator.pos();
                for (i, goal) in goals.iter().enumerate() {
                    if goal.is_satisfied(curr_pos) {
                        successes[i].push(iterator.weight_state().iter().map(|s| s.to_vec()).collect());
                        strat_offsets[i].push(curr_pos);
                        dists[i].push(goal.get_dists(curr_pos));
                    }
                }
                if iterator.advance() { break; }
            }
        }
        (successes, strat_offsets, dists)
    }

    pub fn facing_range_bf<G>(
        n: usize,
        max_counts: Vec<usize>,
        pools: Vec<Vec<Vec<(f32, f32)>>>,
        is_reversible: Vec<bool>,
        goals: &[Vec<G>],
        fstart: f32,
        fstep: f32,
        len_facings: usize,
    ) -> (Vec<Vec<Vec<Vec<isize>>>>, Vec<Vec<(f32, f32)>>, Vec<Vec<G::Dist>>, Vec<Vec<Vec<f32>>>)
    where
        G: Goal<(f32, f32)>,
    {
        let len_goals = goals.len();
        let mut successes = vec![Vec::new(); len_goals];
        let mut strat_offsets: Vec<Vec<(f32, f32)>> = vec![Vec::new(); len_goals];
        let mut dists: Vec<Vec<G::Dist>> = (0..len_goals).map(|_| Vec::new()).collect();

        let mut facings: Vec<Vec<usize>> = vec![Vec::with_capacity(5 * len_facings); len_goals];

        let mut quality: Vec<f32> = vec![f32::INFINITY; len_goals];
        let mut prev_quality: Vec<f32> = vec![f32::INFINITY; len_goals];

        for counts in MaxCountIter::new(max_counts.clone(), n) {
            let mut iterator = RotationPoolIteratorConglomerate::new(counts, &pools, &is_reversible);
            loop {
                let mut is_first_time = vec![true; len_goals];
                for fi in 0..=len_facings {
                    iterator.facing_index = fi;
                    let curr_pos = iterator.pos();
                    for (i, goal) in goals.iter().enumerate() {
                        if goal[fi].is_satisfied(curr_pos) {
                            if is_first_time[i] {
                                is_first_time[i] = false;
                                successes[i].push(iterator.weight_state().iter().map(|s| s.to_vec()).collect());
                                strat_offsets[i].push(curr_pos);
                                dists[i].push(goal[fi].get_dists(curr_pos));
                                prev_quality[i] = f32::INFINITY;
                                facings[i].push(len_facings + 1);
                            }

                            quality[i] = goal[fi].get_quality(curr_pos);
                            if quality[i] < prev_quality[i] {
                                let last_offset = strat_offsets[i].last_mut().unwrap();
                                let last_dist = dists[i].last_mut().unwrap();
                                *last_offset = curr_pos.clone();
                                *last_dist = goal[fi].get_dists(curr_pos);
                                prev_quality[i] = std::mem::take(&mut quality[i]);
                            }

                            facings[i].push(fi);
                        }
                    }
                }
                if iterator.advance() { break; }
            }
        }

        let mut success_facings: Vec<Vec<Vec<f32>>> = vec![Vec::new(); len_goals];
        let mut strat_facings: Vec<f32> = Vec::with_capacity(len_facings/2);
        for j in 0..len_goals {
            for i in 1..facings[j].len() {
                if facings[j][i] != len_facings + 1 {
                    strat_facings.push(fstart + fstep * facings[j][i] as f32);
                } 
                else {
                    success_facings[j].push(strat_facings);
                    strat_facings = Vec::with_capacity(len_facings/2);
                }
            }

            success_facings[j].push(strat_facings);
            strat_facings = Vec::with_capacity(len_facings/2);
        }

        (successes, strat_offsets, dists, success_facings)
    }


    #[derive(FromPyObject)]
    enum PoolInput {
        MultiAxisPool(Vec<Vec<(f32, f32)>>),
        SingleAxisPool(Vec<Vec<f32>>),
        RotationPool(Vec<Vec<Vec<(f32, f32)>>>),
    }

    #[derive(FromPyObject)]
    enum GoalInput {
        MultiAxisGoal(Vec<((f32, f32), (f32, f32))>),
        SingleAxisGoal(Vec<(f32, f32)>),
        RotationMultiAxisGoal(Vec<Vec<((f32, f32), (f32, f32))>>),
        RotationSingleAxisGoal(Vec<Vec<(f32, f32)>>),
    }

//    enum Conglomerates<'g> {
//        SingleAxis(SingleAxisPoolIteratorConglomerate<'g>),
//        MultiAxis(PoolIteratorConglomerate<'g>),
//    }

    #[pyfunction]
    fn brute_force(n: usize, max_counts: Vec<usize>, pools: PoolInput, is_reversible: Vec<bool>, goals_args: GoalInput, axis: usize, fstart: Option<f32>, fstep: Option<f32>, len_facings: Option<usize>) -> Vec<(Vec<Vec<isize>>, (Option<f32>, Option<f32>), (Option<f32>, Option<f32>, Option<f32>, Option<f32>), Option<usize>, Option<Vec<f32>>)> {

        match goals_args {
            GoalInput::MultiAxisGoal(goals_args) => {
                match pools {
                    PoolInput::MultiAxisPool(pools) => {
                        let goals: Vec<XZGoal> = goals_args.into_iter().map(|(start, end)| XZGoal::new(start, end)).collect();
                        let output = no_facing_range_bf(n, max_counts, |counts| -> PoolIteratorConglomerate { PoolIteratorConglomerate::new(counts, &pools, &is_reversible) }, &goals);
                        let mut sanitized_output = Vec::new();

                        for goal_index in 0..output.0.len() {
                            for i in 0..output.0[goal_index].len() {
                                let weights = output.0[goal_index][i].clone();
                                let offset = (Some(output.1[goal_index][i].0), Some(output.1[goal_index][i].1));
                                let dist = (Some(output.2[goal_index][i].0), Some(output.2[goal_index][i].1), Some(output.2[goal_index][i].2), Some(output.2[goal_index][i].3));
                                let for_goal = Some(goal_index);
                                let facings = None;

                                sanitized_output.push((weights, offset, dist, for_goal, facings))
                            }
                        }

                        return sanitized_output
                    }
                    PoolInput::SingleAxisPool(_pools) => {
                        panic!("MultiAxis but also SingleAxis!");
                    }
                    PoolInput::RotationPool(_pools) => {
                        panic!("Rotation but also SingleAxis!")
                    }
                }
            }
            
            GoalInput::SingleAxisGoal(goals_args) => {
                match pools {
                    PoolInput::MultiAxisPool(_pools) => {
                        panic!("SingleAxis but also MultiAxis!")
                    }
                    PoolInput::SingleAxisPool(pools) => {
                        let goals: Vec<SingleAxisGoal> = goals_args.into_iter().map(|(start, end)| SingleAxisGoal::new(start, end)).collect();
                        let output = no_facing_range_bf(n, max_counts, |counts| -> SingleAxisPoolIteratorConglomerate { SingleAxisPoolIteratorConglomerate::new(counts, &pools, &is_reversible) }, &goals);
                        let mut sanitized_output = Vec::new();

                        for goal_index in 0..output.0.len() {
                            for i in 0..output.0[goal_index].len() {
                                let weights = output.0[goal_index][i].clone();
                                let offset = if axis == 0 {(Some(output.1[goal_index][i]), None)} else {(None, Some(output.1[goal_index][i]))};
                                let dist = if axis == 0 {(Some(output.2[goal_index][i].0), None, Some(output.2[goal_index][i].1), None)} else {(None, Some(output.2[goal_index][i].0), None, Some(output.2[goal_index][i].1))};
                                let for_goal = Some(goal_index);
                                let facings = None;

                                sanitized_output.push((weights, offset, dist, for_goal, facings))
                            }
                        }

                        return sanitized_output
                    }
                    PoolInput::RotationPool(_pools) => {
                        panic!("Non-Rotation goal but RotationPool!")
                    }
                }
            }

            GoalInput::RotationMultiAxisGoal(goals_args) => {
                match pools {
                    PoolInput::MultiAxisPool(_pools) => {
                        panic!("Rotation goal but non-rotation pool!")
                    }
                    PoolInput::SingleAxisPool(_pools) => {
                        panic!("Rotation goal but non-rotation pool!")
                    }
                    PoolInput::RotationPool(pools) => {
                        let goals: Vec<Vec<XZGoal>> = goals_args.into_iter().map(|goals_inner_args| goals_inner_args.into_iter().map(|(start, end)| XZGoal::new(start, end)).collect()).collect();
                        let output = facing_range_bf(n, max_counts, pools, is_reversible, &goals, fstart.unwrap(), fstep.unwrap(), len_facings.unwrap());
                        let mut sanitized_output = Vec::new();

                        for goal_index in 0..output.0.len() {
                            for i in 0..output.0[goal_index].len() {
                                let weights = output.0[goal_index][i].clone();
                                let offset = (Some(output.1[goal_index][i].0), Some(output.1[goal_index][i].1));
                                let dist = (Some(output.2[goal_index][i].0), Some(output.2[goal_index][i].1), Some(output.2[goal_index][i].2), Some(output.2[goal_index][i].3));
                                let for_goal = Some(goal_index);
                                let facings = Some(output.3[goal_index][i].clone());

                                sanitized_output.push((weights, offset, dist, for_goal, facings))
                            }
                        }

                        return sanitized_output
                    }
                }
            }

            GoalInput::RotationSingleAxisGoal(goals_args) => {
                match pools {
                    PoolInput::MultiAxisPool(_pools) => {
                        panic!("Rotation goal but non-rotation pool!")
                    }
                    PoolInput::SingleAxisPool(_pools) => {
                        panic!("Rotation goal but non-rotation pool!")
                    }
                    PoolInput::RotationPool(pools) => {
                        let goals: Vec<Vec<AxisProjectedGoal>> = goals_args.into_iter().map(|goals_inner_args| goals_inner_args.into_iter().map(|(start, end)| AxisProjectedGoal::new(start, end, axis)).collect()).collect();
                        let output = facing_range_bf(n, max_counts, pools, is_reversible, &goals, fstart.unwrap(), fstep.unwrap(), len_facings.unwrap());
                        let mut sanitized_output = Vec::new();

                        for goal_index in 0..output.0.len() {
                            for i in 0..output.0[goal_index].len() {
                                let weights = output.0[goal_index][i].clone();
                                let offset = if axis == 0 {(Some(output.1[goal_index][i].0), None)} else {(None, Some(output.1[goal_index][i].1))};
                                let dist = if axis == 0 {(Some(output.2[goal_index][i].0), None, Some(output.2[goal_index][i].1), None)} else {(None, Some(output.2[goal_index][i].0), None, Some(output.2[goal_index][i].1))};
                                let for_goal = Some(goal_index);
                                let facings = Some(output.3[goal_index][i].clone());

                                sanitized_output.push((weights, offset, dist, for_goal, facings))
                            }
                        }

                        return sanitized_output
                    }
                }
            }
        }
    }
}
