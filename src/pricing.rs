use russcip::{Constraint, Model, Pricer, PricerResult, PricerResultState, ProblemOrSolving, SCIPPricer, Solving, VarType, WithSolutions};
use russcip::prelude::{cons, var};
use crate::{BinPackingInstance, BranchingDecisionMap, BranchingDecisions, ItemToConstraint, PatternForVar};

pub struct KnapsackPricer;

fn get_duals(item_constraints: &[Constraint], farkas: bool) -> Vec<f64> {
    let mut duals = vec![0.0; item_constraints.len()];
    for (item, cons) in item_constraints.iter().enumerate() {
        let c = cons
            .transformed()
            .expect("Could not get transformed constraint");

        duals[item] = if farkas {
            c.farkas_dual_sol().expect("Could not get dual solution")
        } else {
            c.dual_sol().expect("Could not get farkas solution")
        };
    }

    duals
}

impl Pricer for KnapsackPricer {
    fn generate_columns(
        &mut self,
        mut model: Model<Solving>,
        _pricer: SCIPPricer,
        farkas: bool,
    ) -> PricerResult {
        let item_constraints = model.get_data::<ItemToConstraint>().unwrap().0.clone();

        let duals = get_duals(&item_constraints, farkas);

        let instance = model.get_data::<BinPackingInstance>().unwrap();

        let branching_decisions = model
            .get_data::<BranchingDecisionMap>()
            .unwrap()
            .0
            .get(&model.focus_node().number())
            .unwrap();

        let res = solve_knapsack(
            &instance.item_sizes,
            &duals,
            instance.capacity,
            branching_decisions,
        );

        if res.is_none() {
            return PricerResult {
                state: PricerResultState::NoColumns,
                lower_bound: None,
            };
        }

        let (sol_items, sol_value) = res.unwrap();

        let obj_coef = if farkas { 0.0 } else { 1.0 };
        let redcost = obj_coef - sol_value;

        if redcost < -model.eps() {
            // println!("-- Adding new pattern {sol_items:?} with reduced cost {redcost}");
            let new_var = model.add_priced_var(
                0.0,
                f64::INFINITY,
                1.0,
                format!("{sol_items:?}").as_str(),
                VarType::Integer,
            );

            for item in sol_items.iter() {
                model.add_cons_coef(&item_constraints[*item], &new_var, 1.0);
            }

            model
                .get_data_mut::<PatternForVar>()
                .unwrap()
                .0
                .insert(new_var.index(), sol_items.clone());

            PricerResult {
                state: PricerResultState::FoundColumns,
                lower_bound: None,
            }
        } else {
            PricerResult {
                state: PricerResultState::NoColumns,
                lower_bound: None,
            }
        }
    }
}

/// Solve the knapsack problem and return the selected items and the total profit.
fn solve_knapsack(
    sizes: &[f64],
    profits: &Vec<f64>,
    capacity: f64,
    branching_decision: &BranchingDecisions,
) -> Option<(Vec<usize>, f64)> {
    let mut model = Model::default().hide_output().maximize();

    let mut vars = Vec::with_capacity(sizes.len());
    for profit in profits {
        vars.push(model.add(var().bin().obj(*profit)));
    }

    let mut capacity_cons = cons().le(capacity);
    for (i, var) in vars.iter().enumerate() {
        capacity_cons = capacity_cons.coef(var, sizes[i]);
    }
    model.add(capacity_cons);

    // add branching decisions
    // together constraints
    for pair in branching_decision.together.iter() {
        let var1 = &vars[pair.0];
        let var2 = &vars[pair.1];
        model.add(cons().eq(1.0).coef(var1, 1.0).coef(var2, -1.0));
    }

    // apart constraints
    for pair in branching_decision.apart.iter() {
        let var1 = &vars[pair.0];
        let var2 = &vars[pair.1];
        model.add(cons().le(1.0).coef(var1, 1.0).coef(var2, 1.0));
    }

    let solved_model = model.solve();

    let sol = solved_model.best_sol()?;
    let mut items = vec![];
    for (i, var) in vars.iter().enumerate() {
        if sol.val(var) > 0.5 {
            items.push(i);
        }
    }
    let value = sol.obj_val();

    assert!(items.iter().map(|i| sizes[*i]).sum::<f64>() <= capacity);

    Some((items, value))
}