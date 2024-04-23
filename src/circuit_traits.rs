use ark_ff::Field;
use ark_std::rand::RngCore;

pub trait BenchCircuit<F: Field> {
    fn new_random<R: RngCore>(rng: &mut R, constraints: usize, printing: bool) -> Self;
    fn get_result(&self) -> F;
}