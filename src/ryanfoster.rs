use std::collections::{BTreeMap, HashSet};
use russcip::{BranchRule, BranchingCandidate, BranchingResult, Model, ModelWithProblem, SCIPBranchRule, Solving};
use crate::{BranchingDecisionMap, Pair, PatternForVar};

pub struct RyanFoster;

impl BranchRule for RyanFoster {
    fn execute(
        &mut self,
        mut model: Model<Solving>,
        _branchrule: SCIPBranchRule,
        candidates: Vec<BranchingCandidate>,
    ) -> BranchingResult {
        let fractional_pair = RyanFoster::find_fractional_pair(
            &model,
            model.get_data::<PatternForVar>().unwrap(),
            &candidates,
        );
        // println!("-- Branching on fractional pair: {:?}", fractional_pair);

        let current_bb_node = model.focus_node().number();
        let current_decisions = model
            .get_data::<BranchingDecisionMap>()
            .unwrap()
            .0
            .get(&current_bb_node)
            .unwrap()
            .clone();

        // save branching decisions (for the pricer)
        let down_child = model.create_child();
        let up_child = model.create_child();
        let mut down_decisions = current_decisions.clone();
        down_decisions.apart.insert(fractional_pair.clone());
        let mut up_decisions = current_decisions.clone();
        up_decisions.together.insert(fractional_pair.clone());
        model
            .get_data_mut::<BranchingDecisionMap>()
            .unwrap()
            .0
            .insert(down_child.number(), down_decisions);
        model
            .get_data_mut::<BranchingDecisionMap>()
            .unwrap()
            .0
            .insert(up_child.number(), up_decisions);

        // fix infeasible variables
        let (i, j) = (fractional_pair.0, fractional_pair.1);
        for var in model.vars().iter() {
            // skip fixed vars
            if var.ub_local() < model.eps() {
                continue;
            }

            let pattern = model
                .get_data::<PatternForVar>()
                .unwrap()
                .0
                .get(&var.index())
                .unwrap()
                .clone();

            let pattern_set = HashSet::<&usize>::from_iter(pattern.iter());

            // down child: fix any variable that uses both nodes of the pair
            if pattern_set.contains(&i) && pattern_set.contains(&j) {
                model.set_ub_node(&down_child, var, 0.0);
            }

            // up child: fix any variable that uses neither node of the pair
            let neither_is_in_pattern = !pattern_set.contains(&i) && !pattern_set.contains(&j);
            let both_are_in_pattern = pattern_set.contains(&i) && pattern_set.contains(&j);
            if !(both_are_in_pattern || neither_is_in_pattern) {
                model.set_ub_node(&up_child, var, 0.0);
            }
        }

        BranchingResult::CustomBranching
    }
}

impl RyanFoster {
    fn find_fractional_pair(
        model: &Model<Solving>,
        pattern_for_var: &PatternForVar,
        candidates: &Vec<BranchingCandidate>,
    ) -> Pair {
        let mut pair_vals = BTreeMap::new();
        for candidate in candidates {
            let var = model.var_in_prob(candidate.var_prob_id).unwrap();
            let pattern = pattern_for_var.0.get(&var.index()).unwrap();

            for i in 0..pattern.len() - 1 {
                for j in i + 1..pattern.len() {
                    let item_i = pattern[i];
                    let item_j = pattern[j];

                    if item_i != item_j {
                        let pair = (item_i, item_j);
                        let val = pair_vals.entry(pair).or_insert(0.0);
                        *val += candidate.lp_sol_val;
                    }
                }
            }
        }

        // find the pair with the largest fractional value
        let pair = pair_vals
            .iter()
            .filter(|&(_, &val)| val.fract() > model.eps() && val < 1.0 - model.eps())
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;

        Pair(pair.0, pair.1)
    }
}