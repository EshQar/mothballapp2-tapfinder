use pyo3::prelude::*;
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

    use std::collections::HashMap;
    use indexmap::IndexMap;

    fn hashmap_to_indexmap<K, V>(map: HashMap<K, V>) -> IndexMap<K, V>
    where
        K: std::hash::Hash + Eq,
    {
        map.into_iter().collect()
    }

    use crate::{expr_eval::evaluate, player::PlayerSimulationXZ};
    use pyo3::prelude::*;
    use crate::player;
    use crate::parser;

    /// Erm
    #[pyfunction]
    fn mothball(sequence: String, return_defaults: bool, locals: Option<HashMap<String, parser::Data>>, suppress_exception: bool) -> String {
        let index_locals = match locals {
            Some(hashmap) => { Some(hashmap_to_indexmap(hashmap)) }
            None => None,
        };

        let mut sim_player: PlayerSimulationXZ = player::PlayerSimulationXZ::new();
        sim_player.simulate(sequence, return_defaults, index_locals, suppress_exception);
        sim_player.show_output()
    }
}
