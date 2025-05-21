use russcip::{Constraint, Model, Pricer, PricerResult, PricerResultState, ProblemOrSolving, SCIPPricer, Solving, VarType, WithSolutions};
use russcip::prelude::{cons, var};
use crate::{BinPackingInstance, BranchingDecisionMap, BranchingDecisions, ItemToConstraint, PatternForVar};
use crate::knapsack::solve_knapsack;

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


