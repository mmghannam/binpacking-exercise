use std::collections::HashMap;
use russcip::{Model, ModelWithProblem, ParamSetting, WithSolutions};
use russcip::prelude::{branchrule, cons, pricer};
use crate::{BinPackingInstance, BranchingDecisionMap, ItemToConstraint, PatternForVar};

pub fn solve_binpacking(
    sizes: &[f64],
    capacity: f64,
) -> (Vec<Vec<usize>>, f64) {
    let mut model = Model::default()
        .set_presolving(ParamSetting::Off)
        .set_separating(ParamSetting::Off)
        .set_param("display/freq", 1)
        .minimize();

    model.set_data(PatternForVar(HashMap::new()));
    model.set_data(ItemToConstraint(Vec::new()));
    model.set_data(BinPackingInstance {
        item_sizes: sizes.to_vec(),
        capacity,
    });
    model.set_data(BranchingDecisionMap::default());

    for i in 0..sizes.len() {
        let item_constraint = model.add(cons().eq(1.0).modifiable(true).removable(false));

        model
            .get_data_mut::<ItemToConstraint>()
            .unwrap()
            .0
            .insert(i, item_constraint);
    }

    // attach pricer and branching rule plugins
    model.add(pricer(crate::pricing::KnapsackPricer {}));
    model.add(branchrule(crate::ryanfoster::RyanFoster {}));

    let solved_model = model.solve();

    let solution = solved_model.best_sol().unwrap();
    let pattern_for_var = &solved_model.get_data::<PatternForVar>().unwrap().0;
    let mut sol_patterns = vec![];
    for var in solved_model.vars().iter() {
        let value = solution.val(var);
        if value > 1e-6 {
            let pattern = pattern_for_var.get(&var.index()).unwrap();
            sol_patterns.push(pattern.clone());
        }
    }

    (sol_patterns, solved_model.obj_val())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_packing() {
        let sizes = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let capacity = 5.0;
        let (patterns, obj_val) = solve_binpacking(&sizes, capacity);

        assert_eq!(obj_val, 3.0);
        assert_eq!(patterns.len(), 3);
    }
}