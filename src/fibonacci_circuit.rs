use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use colored::Colorize;
use ark_std::rand::RngCore;
use super::circuit_traits::BenchCircuit;
use ark_ff::Field;

fn fibonacci_recursive(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2);
    }
}

#[derive(Copy, Clone)]
pub struct FibonacciCircuit<F: Field> {
    x1: Option<F>,
    x2: Option<F>,
    num_constraints: usize,
    num_variables: usize,
    print: bool,
}

impl<F: Field> BenchCircuit<F> for FibonacciCircuit<F> {
    fn new_random<R: RngCore>(rng: &mut R, constraints: usize, printing: bool) -> Self {
        FibonacciCircuit { 
            x1: Some(<F>::rand(rng)), 
            x2: Some(<F>::rand(rng)),
            num_constraints: if constraints < 4 { 4 } else { constraints },
            num_variables: if constraints < 4 { 4 } else { constraints },
            print: printing,
        }
    }

    fn get_result(&self) -> F {
        if let (Some(x1_val), Some(x2_val)) = (self.x1, self.x2) {
            let exponent: u64 = 2 * fibonacci_recursive((self.num_constraints - 3) as u64);
            let n_minus_3 = x1_val.pow([exponent]);

            let exponent2: u64 = fibonacci_recursive((self.num_constraints - 4) as u64);
            let n_minus_4 = x2_val.pow([exponent2]);

            return n_minus_3 * n_minus_4;
        } else {
            return <F>::zero();
        }
    }
} 

impl<ConstraintF: Field> ConstraintSynthesizer<ConstraintF> for FibonacciCircuit<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        let _x1 = cs.new_witness_variable(|| self.x1.ok_or(SynthesisError::AssignmentMissing))?;
        let _x2 = cs.new_witness_variable(|| self.x2.ok_or(SynthesisError::AssignmentMissing))?;
        let x3 = cs.new_input_variable(|| {
            let x1_val = self.x1.ok_or(SynthesisError::AssignmentMissing)?;
            let x2_val = self.x2.ok_or(SynthesisError::AssignmentMissing)?;

            let exponent: u64 = 2 * fibonacci_recursive((self.num_constraints - 3) as u64);
            let mut n_minus_3 = x1_val.pow([exponent]);
            
            let exponent2: u64 = fibonacci_recursive((self.num_constraints - 4) as u64);
            let n_minus_4 = x2_val.pow([exponent2]);

            n_minus_3.mul_assign(n_minus_4);
            Ok(n_minus_3)
        })?;

        for _ in 0..(self.num_variables - 4) { 
            let _ = cs.new_witness_variable(|| self.x1.ok_or(SynthesisError::AssignmentMissing))?;
        }

        for _ in 0..(self.num_constraints) {
            let x1_val = self.x1.ok_or(SynthesisError::AssignmentMissing)?;
            let x2_val = self.x2.ok_or(SynthesisError::AssignmentMissing)?;

            let exponent: u64 = 2 * fibonacci_recursive((self.num_constraints - 3) as u64);
            let n_minus_3 = x1_val.pow([exponent]);
            let first = cs.new_witness_variable(|| Some(n_minus_3).ok_or(SynthesisError::AssignmentMissing))?;

            let exponent2: u64 = fibonacci_recursive((self.num_constraints - 4) as u64);
            let n_minus_4 = x2_val.pow([exponent2]);
            let second = cs.new_witness_variable(|| Some(n_minus_4).ok_or(SynthesisError::AssignmentMissing))?;

            cs.enforce_constraint(lc!() + first, lc!() + second, lc!() + x3)?;
        }

        if self.print {
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
