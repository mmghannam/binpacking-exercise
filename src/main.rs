use binpacking_exercise::ryanfoster::RyanFoster;
use russcip::prelude::*;
use russcip::*;
use std::collections::HashMap;
use std::hash::Hash;
use binpacking_exercise::{BinPackingInstance, BranchingDecisionMap, ItemToConstraint, PatternForVar};
use binpacking_exercise::pricing::KnapsackPricer;

fn main() {
    let capacity = 15.0;
    let item_sizes = &[6.0, 5.0, 4.0, 2.0, 3.0, 7.0, 5.0, 8.0, 4.0, 5.0];

    let mut model = Model::default()
        .set_presolving(ParamSetting::Off)
        .set_separating(ParamSetting::Off)
        .set_param("display/freq", 1)
        .minimize();

    model.set_data(PatternForVar(HashMap::new()));
    model.set_data(ItemToConstraint(Vec::new()));
    model.set_data(BinPackingInstance {
        item_sizes: item_sizes.to_vec(),
        capacity,
    });
    model.set_data(BranchingDecisionMap::default());

    for i in 0..item_sizes.len() {
        let item_constraint = model.add(cons().eq(1.0).modifiable(true).removable(false));
        
        model
            .get_data_mut::<ItemToConstraint>()
            .unwrap()
            .0
            .insert(i, item_constraint);
    }

    // attach pricer and branching rule plugins
    model.add(pricer(KnapsackPricer {}));
    model.add(branchrule(RyanFoster {}));

    let solved_model = model.solve();

    println!("\nSolution:");
    let solution = solved_model.best_sol().unwrap();
    let pattern_for_var = &solved_model.get_data::<PatternForVar>().unwrap().0;
    for var in solved_model.vars().iter() {
        let value = solution.val(var);
        if value > 1e-6 {
            let pattern = pattern_for_var.get(&var.index()).unwrap();
            println!("{:?} = {value}", pattern);
        }
    }

    assert!(solved_model.eq(solution.obj_val(), 4.0));
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_packing() {
        main();
    }
}
