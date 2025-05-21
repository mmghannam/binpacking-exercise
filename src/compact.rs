use russcip::{Model, ProblemCreated};

fn bin_packing_compact(sizes: &[f64], capacity: f64) -> Model<ProblemCreated> {
    todo!("Implement the compact formulation");
}

#[cfg(test)]
mod tests {
    use super::*;
    use russcip::{Model, Status};

    #[test]
    fn test_bin_packing_compact() {
        let sizes = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let capacity = 5.0;
        let model: Model<ProblemCreated> = bin_packing_compact(&sizes, capacity);

        let solved = model.solve();
        assert_eq!(solved.status(), Status::Optimal);
        assert!(solved.eq(solved.obj_val(), 3.0))
    }
}
