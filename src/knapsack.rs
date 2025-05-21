use russcip::{Model, WithSolutions};
use russcip::prelude::{cons, var};
use crate::BranchingDecisions;

/// Solve the knapsack problem and return the selected items and the total profit.
pub fn solve_knapsack(
    sizes: &[f64],
    profits: &[f64],
    capacity: f64,
    branching_decision: &BranchingDecisions,
) -> Option<(Vec<usize>, f64)> {
    todo!("Implement the knapsack solver");
}

#[cfg(test)]
mod tests {
    use crate::knapsack::solve_knapsack;
    use crate::{BranchingDecisions, Pair};

    #[test]
    fn test_knapsack() {
        let sizes = vec![1.0, 2.0, 3.0];
        let profits = vec![10.0, 20.0, 30.0];
        let capacity = 5.0;

        let branching_decision = BranchingDecisions::default();

        let res = solve_knapsack(&sizes, &profits, capacity, &branching_decision);
        assert!(res.is_some());
        let (items, value) = res.unwrap();
        assert_eq!(items, vec![1, 2]);
        assert_eq!(value, 50.0);
    }

    #[test]
    fn test_knapsack_branching_togheter() {
        let sizes = vec![1.0, 2.0, 3.0];
        let profits = vec![10.0, 20.0, 40.0];
        let capacity = 5.0;

        let mut branching_decision = BranchingDecisions::default();
        branching_decision.together.insert(Pair(0, 1));

        let res = solve_knapsack(&sizes, &profits, capacity, &branching_decision);
        assert!(res.is_some());
        let (items, value) = res.unwrap();
        assert_eq!(items, vec![2]);
        assert_eq!(value, 40.0);
    }

    #[test]
    fn test_knapsack_branching_apart() {
        let sizes = vec![1.0, 2.0, 3.0];
        let profits = vec![10.0, 20.0, 30.0];
        let capacity = 5.0;

        let mut branching_decision = BranchingDecisions::default();
        branching_decision.apart.insert(Pair(0, 1));

        let res = solve_knapsack(&sizes, &profits, capacity, &branching_decision);
        assert!(res.is_some());
        let (items, value) = res.unwrap();
        assert_eq!(items, vec![1, 2]);
        assert_eq!(value, 50.0);
    }
}