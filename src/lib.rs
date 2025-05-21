use std::collections::{HashMap, HashSet};
use russcip::{Constraint, VarId};

pub mod ryanfoster;
pub mod pricing;


pub struct PatternForVar(pub HashMap<VarId, Vec<usize>>);
pub struct ItemToConstraint(pub Vec<Constraint>);

pub struct BinPackingInstance {
    pub item_sizes: Vec<f64>,
    pub capacity: f64,
}

pub type BBNodeId = usize;

#[derive(Debug, Clone, Hash, Default, Eq, PartialEq)]
pub struct Pair(usize, usize);

#[derive(Debug, Clone, Default)]
pub struct BranchingDecisions {
    together: HashSet<Pair>,
    apart: HashSet<Pair>,
}

pub struct BranchingDecisionMap(HashMap<BBNodeId, BranchingDecisions>);

impl Default for BranchingDecisionMap {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(1, BranchingDecisions::default());
        BranchingDecisionMap(map)
    }
}