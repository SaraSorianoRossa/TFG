use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use colored::Colorize;
use ark_std::rand::RngCore;

use super::circuit_traits::BenchCircuit;
use ark_ff::Field;

#[derive(Copy, Clone)]
pub struct HadamardCircuit<F: Field> {
    a: Option<F>,
    b: Option<F>,
    num_constraints: usize,
    num_variables: usize,
    print: bool,
}

impl<F: Field> BenchCircuit<F> for HadamardCircuit<F> {
    fn new_random<R: RngCore>(rng: &mut R, constraints: usize, printing: bool) -> Self {
        HadamardCircuit { 
            a: Some(<F>::rand(rng)), 
            b: Some(<F>::rand(rng)), 
            num_constraints: if constraints < 3 { 3 } else { constraints },
            num_variables: if constraints < 3 { 3 } else { constraints },
            print: printing,
        }
    }

    fn get_result(&self) -> F {
        if let (Some(a_val), Some(b_val)) = (self.a, self.b) {
            return a_val * b_val;
        } else {
            return <F>::zero();
        }
    }
} 

impl<ConstraintF: Field> ConstraintSynthesizer<ConstraintF> for HadamardCircuit<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        let a = cs.new_witness_variable(|| self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let b = cs.new_witness_variable(|| self.b.ok_or(SynthesisError::AssignmentMissing))?;
        let c = cs.new_input_variable(|| {
            let mut a = self.a.ok_or(SynthesisError::AssignmentMissing)?;
            let b = self.b.ok_or(SynthesisError::AssignmentMissing)?;

            a.mul_assign(&b);
            Ok(a)
        })?;

        for _ in 0..(self.num_variables - 3) { 
            let _ = cs.new_witness_variable(|| self.a.ok_or(SynthesisError::AssignmentMissing))?;
        }

        for _ in 0..(self.num_constraints) {
            cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)?;
        }
        if self.print == true{
            println!("{} {}", Colorize::green("Constraints:"), cs.num_constraints());
            println!("{} {}", Colorize::green("Variables:"), cs.num_constraints());
            let matrices = cs.to_matrices().unwrap();
            println!("{} {}", Colorize::green("Num witness variables:"), cs.num_witness_variables() + 1);
            println!(
                "{} A: {}, B: {}, C: {}", 
                Colorize::blue("R1CS non-zeros -"),
                matrices.a_num_non_zero,
                matrices.b_num_non_zero,
                matrices.c_num_non_zero,
            );
            let matrix_num_values = cs.num_constraints() * (cs.num_witness_variables() + 1);
            println!(
                "{} A: {}, B: {}, C: {}", 
                Colorize::blue("R1CS zeros -"),
                matrix_num_values - matrices.a_num_non_zero,
                matrix_num_values - matrices.b_num_non_zero,
                matrix_num_values - matrices.c_num_non_zero,
            );
            println!(
                "{} A: {}%, B: {}%, C: {}%", 
                Colorize::blue("R1CS sparsity -"),
                ((matrix_num_values - matrices.a_num_non_zero) * 100) / matrix_num_values,
                (matrix_num_values - matrices.b_num_non_zero) * 100 / matrix_num_values,
                (matrix_num_values - matrices.c_num_non_zero) * 100 / matrix_num_values,
            );
        }
        Ok(())
    }
}