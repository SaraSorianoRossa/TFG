mod circuit_traits;
mod hadamard_circuit;
mod addition_circuit;
mod fibonacci_circuit;
mod multiple_addition_circuit;

use std::time::Instant;
use clap::{Parser, arg, command};
use colored::Colorize;

use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_poly::univariate::DensePolynomial;
use ark_poly_commit::marlin_pc::MarlinKZG10;
use blake2::Blake2s;

use marlin::Marlin as OriginalMarlin;
use marlin_v2::Marlin as MarlinV2;
use marlin_v3::Marlin as MarlinV3;

use circuit_traits::BenchCircuit;
use addition_circuit::AdditionCircuit;
use hadamard_circuit::HadamardCircuit;
use fibonacci_circuit::FibonacciCircuit;
use multiple_addition_circuit::MultipleAdditionCircuit;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    version: usize,

    #[arg(short, long, default_value = "basic")]
    circuit: String,

    #[arg(short, long, default_value_t = 1)]
    constraints: usize,
}

macro_rules! bench {
    ($marlin:ident, $version:expr, $circuit:ident, $constraints:expr) =>{
        type MultiPC = MarlinKZG10<Bls12_381, DensePolynomial<BlsFr>>;
        type MarlinInst = $marlin::<BlsFr, MultiPC, Blake2s>;

        let num_constraints: usize = 1 << 20;
        let num_variables: usize = 1 << 20;
    
        let rng = &mut ark_std::test_rng();     

        let universal_srs = MarlinInst::universal_setup(num_constraints, num_variables, num_variables, rng).unwrap();

        let circuit_r = $circuit::new_random(rng, $constraints, true);

        let (index_pk, index_vk) = MarlinInst::index(&universal_srs, circuit_r).unwrap();      

        let circuit_instance = $circuit::new_random(rng, $constraints, false);

        let start_time = Instant::now();

        let proof = MarlinInst::prove(&index_pk, circuit_instance, rng).unwrap();
        
        assert!(MarlinInst::verify(&index_vk, &[circuit_instance.get_result()], &proof, rng).unwrap());

        let end_time = Instant::now();
        let duration = end_time - start_time;
        println!("\n{} {:?}", Colorize::bold(Colorize::cyan("Time spended doing Marlin:")), duration);
    }
}

macro_rules! bench_t {
    ($marlin:ident, $version:expr, $circuit:ident, $constraints:expr) =>{
        
        type MultiPC = MarlinKZG10<Bls12_381, DensePolynomial<BlsFr>>;
        type MarlinInst = $marlin::<BlsFr, MultiPC, Blake2s>;

        let num_constraints: usize = 1 << 20;
        let num_variables: usize = 1 << 20;
    
        let rng = &mut ark_std::test_rng();
        let universal_srs = MarlinInst::universal_setup(num_constraints, num_variables, num_variables, rng).unwrap();
    
        let circuit_r = $circuit::new_random(rng, $constraints, true);

        let (index_pk, index_vk) = MarlinInst::index(&universal_srs, circuit_r).unwrap();
        
        let circuit_instance = $circuit::new_random(rng, $constraints, false);

        let start_time = Instant::now();
        
        let (proof, t_poly) = MarlinInst::prove(&index_pk, circuit_instance, rng).unwrap();

        assert!(MarlinInst::verify(&index_vk, &[circuit_instance.get_result()], &proof, rng, &t_poly).unwrap());

        let end_time = Instant::now();
        let duration = end_time - start_time;
        println!("\n{} {:?}", Colorize::bold(Colorize::cyan("Time spended doing Marlin:")), duration);
    }
}

fn main() {
    let args = Args::parse();
    let version = args.version;
    let circuit = args.circuit.as_str();
    let constraints = args.constraints;

    let start_time = Instant::now();
    match (version, circuit){
        (0, "hadamard") => {bench!(OriginalMarlin, version, HadamardCircuit, constraints);}
        (1, "hadamard") => {bench!(MarlinV2, version, HadamardCircuit, constraints);}
        (2, "hadamard") => {bench_t!(MarlinV3, version, HadamardCircuit, constraints);}
        (0, "addition") => {bench!(OriginalMarlin, version, AdditionCircuit, constraints);}
        (1, "addition") => {bench!(MarlinV2, version, AdditionCircuit, constraints);}
        (2, "addition") => {bench_t!(MarlinV3, version, AdditionCircuit, constraints);}
        (0, "fibonacci") => {bench!(OriginalMarlin, version, FibonacciCircuit, constraints);}
        (1, "fibonacci") => {bench!(MarlinV2, version, FibonacciCircuit, constraints);}
        (2, "fibonacci") => {bench_t!(MarlinV3, version, FibonacciCircuit, constraints);}
        (0, "multiple_addition") => {bench!(OriginalMarlin, version, MultipleAdditionCircuit, constraints);}
        (1, "multiple_addition") => {bench!(MarlinV2, version, MultipleAdditionCircuit, constraints);}
        (2, "multiple_addition") => {bench_t!(MarlinV3, version, MultipleAdditionCircuit, constraints);}
        _ => println!("Invalid version"),
    }
    let end_time = Instant::now();
    let duration = end_time - start_time;
    println!("{} {} {} {}{} {:?}", Colorize::bold(Colorize::cyan("Time spended doing everything in")), 
        Colorize::bold(Colorize::cyan(circuit)), Colorize::bold(Colorize::cyan("circuit with Marlin version")), 
        Colorize::bold(version.to_string().cyan()), Colorize::bold(Colorize::cyan(":")), duration);
}
