use pyo3::prelude::*;

// To do:
// Eliminate recursion in .advance()

mod xz_models;
use xz_models::{PoolIteratorConglomerate, XZGoal};

mod x_or_zmodels;
use x_or_zmodels::{SingleAxisPoolIteratorConglomerate, SingleAxisGoal};

mod iterators;

#[pymodule]
mod brute_forcer {

    use pyo3::prelude::*;
    use pyo3::FromPyObject;

    use crate::{PoolIteratorConglomerate, XZGoal, SingleAxisPoolIteratorConglomerate, SingleAxisGoal, iterators::MaxCountIter};

    // No facing range, single goal, XZ
    fn nfr_sg_xz_bf(n: usize, max_counts: Vec<usize>, pools: Vec<Vec<(f32, f32)>>, is_reversible: Vec<bool>, goal_args: ((f32, f32), (f32, f32))) -> (Vec<Vec<Vec<isize>>>, Vec<(f32, f32)>, Vec<(f32, f32, f32, f32)>) {
        let goal: XZGoal = XZGoal::new(goal_args.0, goal_args.1);

        let mut successes: Vec<Vec<Vec<isize>>> = Vec::with_capacity(5);
        let mut strat_offsets: Vec<(f32, f32)> = Vec::with_capacity(5);
        let mut dists: Vec<(f32, f32, f32, f32)> = Vec::with_capacity(5);

        for counts in MaxCountIter::new(max_counts, n) {
            let mut iterator = PoolIteratorConglomerate::new(counts, &pools, &is_reversible);

            loop {
                let curr_pos = std::mem::take(&mut iterator.pos);

                if goal.is_satisfied(&curr_pos) {
                    successes.push(iterator.weight_state().iter().map(|s| s.to_vec()).collect());
                    strat_offsets.push(curr_pos);
                    dists.push(goal.get_dists(&curr_pos))
                }

                if iterator.advance() {
                    break;
                }
            }
        }

        return (successes, strat_offsets, dists)
    }

    // No facing range, single goal, single axis
    fn nfr_sg_sa_bf(n: usize, max_counts: Vec<usize>, pools: Vec<Vec<f32>>, is_reversible: Vec<bool>, goal_args: (f32, f32)) -> (Vec<Vec<Vec<isize>>>, Vec<f32>, Vec<(f32, f32)>) {
        let goal: SingleAxisGoal = SingleAxisGoal::new(goal_args.0, goal_args.1);

        let mut successes: Vec<Vec<Vec<isize>>> = Vec::with_capacity(5);
        let mut strat_offsets: Vec<f32> = Vec::with_capacity(5);
        let mut dists: Vec<(f32, f32)> = Vec::with_capacity(5);

        for counts in MaxCountIter::new(max_counts, n) {
            let mut iterator = SingleAxisPoolIteratorConglomerate::new(counts, &pools, &is_reversible);

            loop {
                let curr_pos = std::mem::take(&mut iterator.pos);
                
                if goal.is_satisfied(curr_pos) {
                    successes.push(iterator.weight_state().iter().map(|s| s.to_vec()).collect());
                    strat_offsets.push(curr_pos);
                    dists.push(goal.get_dists(curr_pos))
                }
                

                if iterator.advance() {
                    break;
                }
            }
        }

        return (successes, strat_offsets, dists)
    }

    // No facing range, multi goal, XZ
    fn nfr_mg_xz_bf(n: usize, max_counts: Vec<usize>, pools: Vec<Vec<(f32, f32)>>, is_reversible: Vec<bool>, goals_args: Vec<((f32, f32), (f32, f32))>) -> (Vec<Vec<Vec<Vec<isize>>>>, Vec<Vec<(f32, f32)>>, Vec<Vec<(f32, f32, f32, f32)>>) {
        let len_goals = goals_args.len();
        let goals: Vec<XZGoal> = goals_args.into_iter().map(|(start, end)| XZGoal::new(start, end)).collect();

        let mut successes: Vec<Vec<Vec<Vec<isize>>>> = vec![Vec::with_capacity(5); len_goals];
        let mut strat_offsets: Vec<Vec<(f32, f32)>> = vec![Vec::with_capacity(5); len_goals];
        let mut dists: Vec<Vec<(f32, f32, f32, f32)>> = vec![Vec::with_capacity(5); len_goals];

        for counts in MaxCountIter::new(max_counts, n) {
            let mut iterator = PoolIteratorConglomerate::new(counts, &pools, &is_reversible);

            loop {
                let curr_pos = std::mem::take(&mut iterator.pos);
                for i in 0..len_goals {
                    if goals[i].is_satisfied(&curr_pos) {
                        successes[i].push(iterator.weight_state().iter().map(|s| s.to_vec()).collect());
                        strat_offsets[i].push(curr_pos);
                        dists[i].push(goals[i].get_dists(&curr_pos))
                    }

                }

                if iterator.advance() {
                    break;
                }
            }
        }

        return (successes, strat_offsets, dists)
    }

    // No facing range, multi goal, single axis
    fn nfr_mg_sa_bf(n: usize, max_counts: Vec<usize>, pools: Vec<Vec<f32>>, is_reversible: Vec<bool>, goals_args: Vec<(f32, f32)>) -> (Vec<Vec<Vec<Vec<isize>>>>, Vec<Vec<f32>>, Vec<Vec<(f32, f32)>>) {
        let len_goals = goals_args.len();
        let goals: Vec<SingleAxisGoal> = goals_args.into_iter().map(|(start, end)| SingleAxisGoal::new(start, end)).collect();

        let mut successes: Vec<Vec<Vec<Vec<isize>>>> = vec![Vec::with_capacity(5); len_goals];
        let mut strat_offsets: Vec<Vec<f32>> = Vec::with_capacity(5);
        let mut dists: Vec<Vec<(f32, f32)>> = Vec::with_capacity(5);

        for counts in MaxCountIter::new(max_counts, n) {
            let mut iterator = SingleAxisPoolIteratorConglomerate::new(counts, &pools, &is_reversible);

            loop {
                let curr_pos = std::mem::take(&mut iterator.pos);
                for i in 0..len_goals {
                    if goals[i].is_satisfied(curr_pos) {
                        successes[i].push(iterator.weight_state().iter().map(|s| s.to_vec()).collect());
                        strat_offsets[i].push(curr_pos);
                        dists[i].push(goals[i].get_dists(curr_pos))
                    }

                }

                if iterator.advance() {
                    break;
                }
            }
        }

        return (successes, strat_offsets, dists)
    }

    // Facing range, single goal, XZ
    fn fr_sg_xz_bf(n: usize, max_counts: Vec<usize>, pools: Vec<Vec<(f32, f32)>>, is_reversible: Vec<bool>, goals_args: Vec<((f32, f32), (f32, f32))>, fstart: f32, fstep: f32, fend: f32) -> (Vec<Vec<Vec<isize>>>, Vec<(f32, f32)>, Vec<(f32, f32, f32, f32)>, Vec<Vec<f32>>) {
        let goals: Vec<XZGoal> = goals_args.into_iter().map(|(start, end)| XZGoal::new(start, end)).collect();

        let mut successes: Vec<Vec<Vec<isize>>> = Vec::with_capacity(5);
        let mut strat_offsets: Vec<(f32, f32)> = Vec::with_capacity(5);
        let mut dists: Vec<(f32, f32, f32, f32)> = Vec::with_capacity(5);
        let mut success_facings: Vec<Vec<f32>> = Vec::with_capacity(5);

        assert!(fstart < fend);
        let len_facings = (((fend - fstep - fstart)/fstep).ceil() + 1.0f32) as usize;

        let sin: f32 = fstep.to_radians().sin();
        let cos: f32 = fstep.to_radians().cos();

        let mut facings: Vec<usize> = Vec::with_capacity(5 * len_facings);

        let mut quality: f32;
        let mut prev_quality: f32 = f32::INFINITY;

        for counts in MaxCountIter::new(max_counts, n) {
            let mut iterator = PoolIteratorConglomerate::new(counts, &pools, &is_reversible);

            loop {
                let mut curr_pos = std::mem::take(&mut iterator.pos);
                let mut is_first_time = true;


                for i in 0..=len_facings {
                    if goals[i].is_satisfied(&curr_pos) {
                        if is_first_time {
                            is_first_time = false;
                            successes.push(iterator.weight_state().iter().map(|s| s.to_vec()).collect());

                            prev_quality = f32::INFINITY;

                            strat_offsets.push((0.0f32, 0.0f32));
                            dists.push((0.0f32, 0.0f32, 0.0f32, 0.0f32));

                            facings.push(len_facings)
                        }

                        quality = goals[i].get_quality(&curr_pos);
                        if quality < prev_quality {
                            let last_offset = strat_offsets.last_mut().unwrap();
                            let last_dist = dists.last_mut().unwrap();
                            *last_offset = curr_pos.clone();
                            *last_dist = goals[i].get_dists(&curr_pos);
                            prev_quality = std::mem::take(&mut quality);
                        }
                        facings.push(i);
                    }

                    curr_pos = (curr_pos.0 * cos - curr_pos.1 * sin, curr_pos.0 * sin + curr_pos.1 * cos);
                }

                if iterator.advance() {
                    break;
                }
            }
        }

        let mut strat_facings: Vec<f32> = Vec::with_capacity(len_facings/2);
        for i in 1..facings.len() {
            if facings[i] != len_facings {
                strat_facings.push(fstart + fstep * facings[i] as f32);
            } 
            else {
                success_facings.push(strat_facings);
                strat_facings = Vec::with_capacity(len_facings/2);
            }
        }

        success_facings.push(strat_facings);

        return (successes, strat_offsets, dists, success_facings)
    }

    // Facing range, single goal, single axis
    fn fr_sg_sa_bf(n: usize, max_counts: Vec<usize>, pools: Vec<Vec<(f32, f32)>>, is_reversible: Vec<bool>, goals_args: Vec<(f32, f32)>, axis: usize, fstart: f32, fstep: f32, fend: f32) -> (Vec<Vec<Vec<isize>>>, Vec<f32>, Vec<(f32, f32)>, Vec<Vec<f32>>) {
        let goals: Vec<SingleAxisGoal> = goals_args.into_iter().map(|(start, end)| SingleAxisGoal::new(start, end)).collect();

        let mut successes: Vec<Vec<Vec<isize>>> = Vec::with_capacity(5);
        let mut strat_offsets: Vec<f32> = Vec::with_capacity(5);
        let mut dists: Vec<(f32, f32)> = Vec::with_capacity(5);
        let mut success_facings: Vec<Vec<f32>> = Vec::with_capacity(5);

        assert!(fstart < fend);
        let len_facings = (((fend - fstep - fstart)/fstep).ceil() + 1.0f32) as usize;

        let sin: f32 = fstep.to_radians().sin();
        let cos: f32 = fstep.to_radians().cos();

        let mut facings: Vec<usize> = Vec::with_capacity(5 * len_facings);


        let mut quality: f32;
        let mut prev_quality: f32 = f32::INFINITY;

        for counts in MaxCountIter::new(max_counts, n) {
            let mut iterator = PoolIteratorConglomerate::new(counts, &pools, &is_reversible);

            loop {
                let mut curr_pos = std::mem::take(&mut iterator.pos);
                let mut is_first_time = true;

                for i in 0..=len_facings {

                    if goals[i].is_satisfied(if axis == 0 {curr_pos.0} else {curr_pos.1}) {
                        if is_first_time {
                            is_first_time = false;
                            successes.push(iterator.weight_state().iter().map(|s| s.to_vec()).collect());

                            prev_quality = f32::INFINITY;

                            strat_offsets.push(0.0f32);
                            dists.push((0.0f32, 0.0f32));

                            facings.push(len_facings)
                        }

                        quality = goals[i].get_quality(if axis == 0 {curr_pos.0} else {curr_pos.1});
                        if quality < prev_quality {
                            let last_offset = strat_offsets.last_mut().unwrap();
                            let last_dist = dists.last_mut().unwrap();
                            *last_offset = if axis == 0 {curr_pos.0} else {curr_pos.1};
                            *last_dist = goals[i].get_dists(if axis == 0 {curr_pos.0} else {curr_pos.1});
                            prev_quality = std::mem::take(&mut quality);
                        }
                        facings.push(i);
                    }

                    curr_pos = (curr_pos.0 * cos - curr_pos.1 * sin, curr_pos.0 * sin + curr_pos.1 * cos);
                }

                if iterator.advance() {
                    break;
                }
            }
        }

        let mut strat_facings: Vec<f32> = Vec::with_capacity(len_facings/2);
        for i in 1..facings.len() {
            if facings[i] != len_facings {
                strat_facings.push(fstart + fstep * facings[i] as f32);
            } 
            else {
                success_facings.push(strat_facings);
                strat_facings = Vec::with_capacity(len_facings/2);
            }
        }

        success_facings.push(strat_facings);

        return (successes, strat_offsets, dists, success_facings)
    }

    // Facing range, multi goal, XZ
    fn fr_mg_xz_bf(n: usize, max_counts: Vec<usize>, pools: Vec<Vec<(f32, f32)>>, is_reversible: Vec<bool>, goals_args: Vec<Vec<((f32, f32), (f32, f32))>>, fstart: f32, fstep: f32, fend: f32) -> (Vec<Vec<Vec<Vec<isize>>>>, Vec<Vec<(f32, f32)>>, Vec<Vec<(f32, f32, f32, f32)>>, Vec<Vec<Vec<f32>>>) {
        let len_goals = goals_args.len();
        let goals: Vec<Vec<XZGoal>> = goals_args.into_iter().map(|sub_goals_args| sub_goals_args.into_iter().map(|(start, end)| XZGoal::new(start, end)).collect()).collect();

        let mut successes: Vec<Vec<Vec<Vec<isize>>>> = vec![Vec::with_capacity(5); len_goals];
        let mut strat_offsets: Vec<Vec<(f32, f32)>> = vec![Vec::with_capacity(5); len_goals];
        let mut dists: Vec<Vec<(f32, f32, f32, f32)>> = vec![Vec::with_capacity(5); len_goals];
        let mut success_facings: Vec<Vec<Vec<f32>>> = vec![Vec::with_capacity(5); len_goals];

        assert!(fstart < fend);
        let len_facings = (((fend - fstep - fstart)/fstep).ceil() + 1.0f32) as usize;

        let sin: f32 = fstep.to_radians().sin();
        let cos: f32 = fstep.to_radians().cos();

        let mut facings: Vec<Vec<usize>> = vec![Vec::with_capacity(5 * len_facings); len_goals];

        let mut quality: Vec<f32> = vec![f32::INFINITY; len_goals];
        let mut prev_quality: Vec<f32> = vec![f32::INFINITY; len_goals];

        for counts in MaxCountIter::new(max_counts, n) {
            let mut iterator = PoolIteratorConglomerate::new(counts, &pools, &is_reversible);

            loop {
                let mut curr_pos = std::mem::take(&mut iterator.pos);
                let mut is_first_time = vec![true; len_goals];

                for i in 0..=len_facings {
                    for j in 0..len_goals {
                        if goals[j][i].is_satisfied(&curr_pos) {
                            if is_first_time[j] {
                                successes[j].push(iterator.weight_state().iter().map(|s| s.to_vec()).collect());
                                is_first_time[j] = false;

                                dists[j].push((0.0f32, 0.0f32, 0.0f32, 0.0f32));
                                strat_offsets[j].push((0.0f32, 0.0f32));
                                
                                prev_quality[j] = f32::INFINITY;

                                facings[j].push(len_facings)
                                
                            }

                            quality[j] = goals[j][i].get_quality(&curr_pos);
                            if quality[j] < prev_quality[j] {
                                let last_offset = strat_offsets[j].last_mut().unwrap();
                                let last_dist = dists[j].last_mut().unwrap();
                                *last_offset = curr_pos.clone();
                                *last_dist = goals[j][i].get_dists(&curr_pos);
                                prev_quality[j] = std::mem::take(&mut quality[j]);
                            }

                            facings[j].push(i);
                        }
                    }

                    curr_pos = (curr_pos.0 * cos - curr_pos.1 * sin, curr_pos.0 * sin + curr_pos.1 * cos);
                }


                if iterator.advance() {
                    break;
                }
            }
        }

        let mut strat_facings: Vec<f32> = Vec::with_capacity(len_facings/2);
        for j in 0..len_goals {
            for i in 1..facings[j].len() {
                if facings[j][i] != len_facings {
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

        return (successes, strat_offsets, dists, success_facings)
    }

    // Facing range, multi goal, single axis
    fn fr_mg_sa_bf(n: usize, max_counts: Vec<usize>, pools: Vec<Vec<(f32, f32)>>, is_reversible: Vec<bool>, goals_args: Vec<Vec<(f32, f32)>>, axis: usize, fstart: f32, fstep: f32, fend: f32) -> (Vec<Vec<Vec<Vec<isize>>>>, Vec<Vec<f32>>, Vec<Vec<(f32, f32)>>, Vec<Vec<Vec<f32>>>) {
        let len_goals = goals_args.len();
        let goals: Vec<Vec<SingleAxisGoal>> = goals_args.into_iter().map(|sub_goals_args| sub_goals_args.into_iter().map(|(start, end)| SingleAxisGoal::new(start, end)).collect()).collect();

        let mut successes: Vec<Vec<Vec<Vec<isize>>>> = vec![Vec::with_capacity(5); len_goals];
        let mut strat_offsets: Vec<Vec<f32>> = vec![Vec::with_capacity(5); len_goals];
        let mut dists: Vec<Vec<(f32, f32)>> = vec![Vec::with_capacity(5); len_goals];
        let mut success_facings: Vec<Vec<Vec<f32>>> = vec![Vec::with_capacity(5); len_goals];

        assert!(fstart < fend);
        let len_facings = (((fend - fstep - fstart)/fstep).ceil() + 1.0f32) as usize;

        let sin: f32 = fstep.to_radians().sin();
        let cos: f32 = fstep.to_radians().cos();

        let mut facings: Vec<Vec<usize>> = Vec::with_capacity(5 * len_facings);

        let mut quality: Vec<f32> = vec![f32::INFINITY; len_goals];
        let mut prev_quality: Vec<f32> = vec![f32::INFINITY; len_goals];

        for counts in MaxCountIter::new(max_counts, n) {
            let mut iterator = PoolIteratorConglomerate::new(counts, &pools, &is_reversible);

            loop {
                let mut curr_pos = std::mem::take(&mut iterator.pos);
                let mut is_first_time = true;

                for i in 0..=len_facings {
                    for j in 0..len_goals {
                        if goals[j][i].is_satisfied(if axis == 0 {curr_pos.0} else {curr_pos.1}) {
                            if is_first_time {
                                is_first_time = false;
                                successes[j].push(iterator.weight_state().iter().map(|s| s.to_vec()).collect());

                                strat_offsets[j].push(0.0f32);
                                dists[j].push((0.0f32, 0.0f32));

                                prev_quality[j] = f32::INFINITY;

                                facings[j].push(len_facings)
                            }

                            quality[j] = goals[j][i].get_quality(if axis == 0 {curr_pos.0} else {curr_pos.1});
                            if quality[j] < prev_quality[j] {
                                let last_offset = strat_offsets[j].last_mut().unwrap();
                                let last_dist = dists[j].last_mut().unwrap();
                                *last_offset = if axis == 0 {curr_pos.0} else {curr_pos.1};
                                *last_dist = goals[j][i].get_dists(if axis == 0 {curr_pos.0} else {curr_pos.1});
                                prev_quality[j] = std::mem::take(&mut quality[j]);
                            }
                            facings[j].push(i);
                        }
                    }

                    curr_pos = (curr_pos.0 * cos - curr_pos.1 * sin, curr_pos.0 * sin + curr_pos.1 * cos);
                }

                if iterator.advance() {
                    break;
                }
            }
        }

        let mut strat_facings: Vec<f32> = Vec::with_capacity(len_facings/2);
        for j in 0..len_goals {
            for i in 1..facings[j].len() {
                if facings[j][i] != len_facings {
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


        return (successes, strat_offsets, dists, success_facings)
    }

    #[derive(FromPyObject)]
    enum PoolInput {
        MultiAxisPool(Vec<Vec<(f32, f32)>>),
        SingleAxisPool(Vec<Vec<f32>>),
    }

    #[derive(FromPyObject)]
    enum GoalInput {
        MultiAxisGoal(Vec<((f32, f32), (f32, f32))>),
        SingleAxisGoal(Vec<(f32, f32)>),
        NestedMultiAxisGoal(Vec<Vec<((f32, f32), (f32, f32))>>),
        NestedSingleAxisGoal(Vec<Vec<(f32, f32)>>),
    }

    #[pyfunction]
    fn brute_force(n: usize, max_counts: Vec<usize>, pools: PoolInput, is_reversible: Vec<bool>, goals_args: GoalInput, axis: usize, fstart: Option<f32>, fstep: Option<f32>, fend: Option<f32>) -> Vec<(Vec<Vec<isize>>, (Option<f32>, Option<f32>), (Option<f32>, Option<f32>, Option<f32>, Option<f32>), Option<usize>, Option<Vec<f32>>)> {
        
        match goals_args {
            GoalInput::MultiAxisGoal(goals_args) => {
                match pools {
                    PoolInput::MultiAxisPool(pools) => {
                        if goals_args.len() == 1 {
                            let output = nfr_sg_xz_bf(n, max_counts, pools, is_reversible, goals_args[0]);
                            let mut sanitized_output = Vec::new();

                            for i in 0..output.0.len() {
                                let weights = output.0[i].clone();
                                let offset = (Some(output.1[i].0), Some(output.1[i].1));
                                let dist = (Some(output.2[i].0), Some(output.2[i].1), Some(output.2[i].2), Some(output.2[i].3));
                                let for_goal = None;
                                let facings = None;

                                sanitized_output.push((weights, offset, dist, for_goal, facings))
                            }

                            return sanitized_output
                        } else {
                            let output = nfr_mg_xz_bf(n, max_counts, pools, is_reversible, goals_args);
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
                    }
                    PoolInput::SingleAxisPool(_) => {
                        panic!("MultiAxis but also SingleAxis!")
                    }
                }
            }
            GoalInput::SingleAxisGoal(goals_args) => {
                match pools {
                    PoolInput::MultiAxisPool(_) => {
                        panic!("MultiAxis but also SingleAxis!")
                    }
                    PoolInput::SingleAxisPool(pools) => {
                        assert!(axis == 0 || axis == 1);

                        if goals_args.len() == 1 {
                            let output = nfr_sg_sa_bf(n, max_counts, pools, is_reversible, goals_args[0]);
                            let mut sanitized_output = Vec::new();

                            for i in 0..output.0.len() {
                                let weights = output.0[i].clone();
                                let offset = if axis == 0 {(Some(output.1[i]), None)} else {(None, Some(output.1[i]))};
                                let dist = if axis == 0 {(Some(output.2[i].0), None, Some(output.2[i].1), None)} else {(None, Some(output.2[i].0), None, Some(output.2[i].1))};
                                let for_goal = None;
                                let facings = None;

                                sanitized_output.push((weights, offset, dist, for_goal, facings))
                            }

                            return sanitized_output
                        } else {
                            let output = nfr_mg_sa_bf(n, max_counts, pools, is_reversible, goals_args);
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
                    }
                }
            }
            GoalInput::NestedMultiAxisGoal(goals_args) => {
                match pools {
                    PoolInput::MultiAxisPool(pools) => {
                        if goals_args.len() == 1 {
                            let output = fr_sg_xz_bf(n, max_counts, pools, is_reversible, goals_args[0].clone(), fstart.unwrap(), fstep.unwrap(), fend.unwrap());
                            let mut sanitized_output = Vec::new();

                            for i in 0..output.0.len() {
                                let weights = output.0[i].clone();
                                let offset = (Some(output.1[i].0), Some(output.1[i].1));
                                let dist = (Some(output.2[i].0), Some(output.2[i].1), Some(output.2[i].2), Some(output.2[i].3));
                                let for_goal = None;
                                let facings = Some(output.3[i].clone());

                                sanitized_output.push((weights, offset, dist, for_goal, facings))
                            }

                            return sanitized_output
                        } else {
                            let output = fr_mg_xz_bf(n, max_counts, pools, is_reversible, goals_args, fstart.unwrap(), fstep.unwrap(), fend.unwrap());
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
                    PoolInput::SingleAxisPool(_) => {
                        panic!("Facing range requires both axes!")
                    }
                }
            }
            GoalInput::NestedSingleAxisGoal(goals_args) => {
                match pools {
                    PoolInput::MultiAxisPool(pools) => {
                        assert!(axis == 0 || axis == 1);
                        if goals_args.len() == 1 {
                            let output = fr_sg_sa_bf(n, max_counts, pools, is_reversible, goals_args[0].clone(), axis, fstart.unwrap(), fstep.unwrap(), fend.unwrap());
                            let mut sanitized_output = Vec::new();

                            for i in 0..output.0.len() {
                                let weights = output.0[i].clone();
                                let offset = if axis == 0 {(Some(output.1[i]), None)} else {(None, Some(output.1[i]))};
                                let dist = if axis == 0 {(Some(output.2[i].0), None, Some(output.2[i].1), None)} else {(None, Some(output.2[i].0), None, Some(output.2[i].1))};
                                let for_goal = None;
                                let facings = Some(output.3[i].clone());

                                sanitized_output.push((weights, offset, dist, for_goal, facings))
                            }

                            return sanitized_output
                        } else {
                            let output = fr_mg_sa_bf(n, max_counts, pools, is_reversible, goals_args, axis, fstart.unwrap(), fstep.unwrap(), fend.unwrap());
                            let mut sanitized_output = Vec::new();

                            for goal_index in 0..output.0.len() {
                                for i in 0..output.0[goal_index].len() {
                                    let weights = output.0[goal_index][i].clone();
                                    let offset = if axis == 0 {(Some(output.1[goal_index][i]), None)} else {(None, Some(output.1[goal_index][i]))};
                                    let dist = if axis == 0 {(Some(output.2[goal_index][i].0), None, Some(output.2[goal_index][i].1), None)} else {(None, Some(output.2[goal_index][i].0), None, Some(output.2[goal_index][i].1))};
                                    let for_goal = Some(goal_index);
                                    let facings = Some(output.3[goal_index][i].clone());

                                    sanitized_output.push((weights, offset, dist, for_goal, facings))
                                }
                            }

                            return sanitized_output
                        }
                    }
                    PoolInput::SingleAxisPool(_) => {
                        panic!("Facing range requires both axes!")
                    }
                }
            }
        }
    }



    #[pyfunction]
    fn benchmark(n: usize, max_counts: Vec<usize>, pools: Vec<Vec<(f32, f32)>>, is_reversible: Vec<bool>, goals_args: Vec<((f32, f32), (f32, f32))>) -> (Vec<Vec<Vec<Vec<isize>>>>, Vec<(f32, f32)>, Vec<(f32, f32, f32, f32)>) {
        let goals: Vec<XZGoal> = goals_args.into_iter().map(|(start, end)| XZGoal::new(start, end)).collect();

        let mut successes: Vec<Vec<Vec<Vec<isize>>>> = vec![Vec::new(); goals.len()];
        let mut strat_offsets: Vec<(f32, f32)> = Vec::with_capacity(5);
        let mut dists: Vec<(f32, f32, f32, f32)> = Vec::with_capacity(5);

        for counts in MaxCountIter::new(max_counts, n) {
            let mut iterator = PoolIteratorConglomerate::new(counts, &pools, &is_reversible);

            loop {

                let curr_pos = std::mem::take(&mut iterator.pos);
                for i in 0..goals.len() {
                    if goals[i].is_satisfied(&curr_pos) {
                        successes[i].push(iterator.weight_state().iter().map(|s| s.to_vec()).collect());
                        strat_offsets.push(curr_pos);
                        dists.push(goals[i].get_dists(&curr_pos))
                    }

                }

                if iterator.advance() {
                    break;
                }
            }
        }

        return (successes, strat_offsets, dists)
    }
}
