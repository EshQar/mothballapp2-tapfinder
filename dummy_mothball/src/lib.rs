use pyo3::prelude::*;
use pyo3;
mod expr_eval;
mod player;
mod math;
mod parser;
mod counter;
mod errors;
mod sort;
mod functions;
mod args;

/// A Python module implemented in Rust.
#[pymodule]
mod dummy_mothball {
    use pyo3;
    use std::collections::HashMap;
    use indexmap::IndexMap;

    fn hashmap_to_indexmap<K, V>(map: HashMap<K, V>) -> IndexMap<K, V>
    where
        K: std::hash::Hash + Eq,
    {
        map.into_iter().collect()
    }

    use crate::{player::PlayerSimulationXZ};
    use pyo3::prelude::*;
    use crate::player;
    use crate::parser;
    /// Erm
    #[pyfunction]
    fn mothball(sequence: String, return_defaults: bool, locals: Option<HashMap<String, parser::Data>>, suppress_exception: bool) -> Vec<Vec<String>> {
        let index_locals = match locals {
            Some(hashmap) => { Some(hashmap_to_indexmap(hashmap)) }
            None => None,
        };

        let mut sim_player: PlayerSimulationXZ = player::PlayerSimulationXZ::new();
        sim_player.simulate(sequence, return_defaults, index_locals, suppress_exception);
        return sim_player.return_output()
    }
    use crate::expr_eval;
    #[pyfunction]
    fn expression_eval(expr: &str, vars: HashMap<String, parser::Data>) -> f64 {
        let index_locals = hashmap_to_indexmap(vars);
        expr_eval::evaluate(expr, index_locals)
    }
}
