use pyo3::prelude::*;

mod models;
mod goals;
mod position;
mod wall_and_edges;
mod iterators;
mod brute_force;
mod conglomerate;

#[pymodule]
mod brute_forcer {

use pyo3::prelude::*;
    use pyo3::FromPyObject;
    use std::ops::Mul;

    use crate::brute_force::{no_facing_bf, facing_bf};
    use crate::position::Position2D;
    use crate::goals::{MultiAxisGoal, MultiAxisRotationGoal, SingleAxisGoal, XRotationGoal, ZRotationGoal};
    use crate::conglomerate::{NoRotationPoolIteratorConglomerate, RotationPoolIteratorConglomerate, Conglomerate};
    use crate::iterators::MaxCountIter;

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
    fn brute_force(n: usize, strat_count: usize, max_counts: Vec<usize>, pools: PoolInput, is_reversible: Vec<bool>, goals_args: GoalInput, axis: usize) -> Vec<(Vec<Vec<isize>>, (Option<f32>, Option<f32>), (Option<f32>, Option<f32>, Option<f32>, Option<f32>), Option<usize>, Option<Vec<usize>>)> {

        match goals_args {
            GoalInput::MultiAxisGoal(goals_args) => {
                match pools {
                    PoolInput::MultiAxisPool(pools) => {
                        let goals: Vec<MultiAxisGoal> = goals_args.into_iter().map(|(start, end)| MultiAxisGoal::new(start, end)).collect();
                        let pools 
                        = pools.into_iter().map(
                            |pool| 
                            pool.into_iter().map(
                                |offset| Position2D { x: offset.0, z: offset.1 }
                                ).collect()
                            ).collect();

                        let output = no_facing_bf(n, strat_count, max_counts, NoRotationPoolIteratorConglomerate::new(&pools, &is_reversible), goals);
                        let mut sanitized_output = Vec::new();

                        for goal_index in 0..output.0.len() {
                            for i in 0..output.0[goal_index].len() {
                                let weights = output.0[goal_index][i].clone();
                                let offset = (Some(output.1[goal_index][i].x), Some(output.1[goal_index][i].z));
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
                        let output = no_facing_bf(n, strat_count, max_counts, NoRotationPoolIteratorConglomerate::new(&pools, &is_reversible), goals);
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
                        let goals: Vec<MultiAxisRotationGoal> = goals_args.into_iter().map(|args| MultiAxisRotationGoal::new(args)).collect();
                        let pools 
                        = pools.into_iter().map(
                            |pool| 
                            pool.into_iter().map(
                                |offset| offset.into_iter().map(|pos| Position2D {x: pos.0, z: pos.1 }).collect()
                                ).collect()
                            ).collect();

                        let output = facing_bf(n, strat_count, max_counts, RotationPoolIteratorConglomerate::new(&pools, &is_reversible), goals);
                        let mut sanitized_output = Vec::new();

                        for goal_index in 0..output.0.len() {
                            for i in 0..output.0[goal_index].len() {
                                let weights = output.0[goal_index][i].clone();
                                let offset = (Some(output.1[goal_index][i].x), Some(output.1[goal_index][i].z));
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
                        let pools 
                        = pools.into_iter().map(
                            |pool| 
                            pool.into_iter().map(
                                |offset| offset.into_iter().map(|pos| Position2D {x: pos.0, z: pos.1 }).collect()
                                ).collect()
                            ).collect();

                        let output: (Vec<Vec<Vec<Vec<isize>>>>, Vec<Vec<f32>>, Vec<Vec<(f32, f32)>>, Vec<Vec<Vec<usize>>>);
                        if axis == 0 {
                            let goals: Vec<XRotationGoal> = goals_args.into_iter().map(|args| XRotationGoal::new(args)).collect();
                            output = facing_bf(n, strat_count, max_counts, RotationPoolIteratorConglomerate::new(&pools, &is_reversible), goals);
                        } else if axis == 1 {
                            let goals: Vec<ZRotationGoal> = goals_args.into_iter().map(|args| ZRotationGoal::new(args)).collect();
                            output = facing_bf(n, strat_count, max_counts, RotationPoolIteratorConglomerate::new(&pools, &is_reversible), goals);
                        } else {
                            panic!("Axis is SingleAxis, but also MultiAxis!")
                        }
                        
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
            }
        }
    }
}
