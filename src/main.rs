mod circuit_traits;
mod hadamard_circuit;
mod addition_circuit;
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
use marlin_v4::Marlin as MarlinV4;

use circuit_traits::BenchCircuit;
use addition_circuit::AdditionCircuit;
use hadamard_circuit::HadamardCircuit;
use multiple_addition_circuit::MultipleAdditionCircuit;

use ark_groth16::Groth16;
use ark_snark::SNARK;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    version: usize,

    #[arg(short, long, default_value = "hadamard")]
    circuit: String,

    #[arg(short, long, default_value_t = 1)]
    constraints: usize,

    #[arg(short, long, default_value = "false")]
    groth16: String,
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

        let mut total_duration = std::time::Duration::new(0, 0);

        for _ in 0..10 {
            let start_time = Instant::now();
            
            let proof = MarlinInst::prove(&index_pk, circuit_instance, rng).unwrap();

            assert!(MarlinInst::verify(&index_vk, &[circuit_instance.get_result()], &proof, rng).unwrap());

            let end_time = Instant::now();
            let duration = end_time - start_time;
            total_duration += duration;
        }
        println!("\n{} {:?}", Colorize::bold(Colorize::cyan("Time spended proving and verifying in Marlin:")), total_duration/10);
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

        let mut total_duration = std::time::Duration::new(0, 0);

        for _ in 0..10 {
            let start_time = Instant::now();
            
            let (proof, t_poly) = MarlinInst::prove(&index_pk, circuit_instance, rng).unwrap();

            assert!(MarlinInst::verify(&index_vk, &[circuit_instance.get_result()], &proof, rng, &t_poly).unwrap());

            let end_time = Instant::now();
            let duration = end_time - start_time;
            total_duration += duration;
        }
        println!("\n{} {:?}", Colorize::bold(Colorize::cyan("Time spended proving and verifying in Marlin:")), total_duration/10);
    }
}

macro_rules! bench_groth {
    ($circuit:ident, $constraints:expr) => {
        let rng = &mut ark_std::test_rng();
        
        let circuit_r = $circuit::new_random(rng, $constraints, true);

        let (index_pk, index_vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit_r, rng).unwrap();      
        
        let circuit_instance = $circuit::new_random(rng, $constraints, false);

        let mut total_duration = std::time::Duration::new(0, 0);

        for _ in 0..10 {
            let start_time = Instant::now();

            let proof = Groth16::<Bls12_381>::prove(&index_pk, circuit_instance, rng).unwrap();

            assert!(Groth16::<Bls12_381>::verify(&index_vk, &[circuit_instance.get_result()], &proof).unwrap());

            let end_time = Instant::now();
            let duration = end_time - start_time;
            total_duration += duration;
        }
        println!("\n{} {:?}", Colorize::bold(Colorize::cyan("Time spended proving and verifying in Groth16:")), total_duration/10);
    }
}

fn main() {
    let args = Args::parse();
    let version = args.version;
    let circuit = args.circuit.as_str();
    let constraints = args.constraints;
    let groth16 = args.groth16.as_str();

    let start_time = Instant::now();
    match (version, circuit){
        (1, "hadamard") => {bench!(OriginalMarlin, version, HadamardCircuit, constraints);}
        (2, "hadamard") => {bench!(MarlinV2, version, HadamardCircuit, constraints);}
        (3, "hadamard") => {bench_t!(MarlinV3, version, HadamardCircuit, constraints);}
        (4, "hadamard") => {bench_t!(MarlinV4, version, HadamardCircuit, constraints);}
        (1, "addition") => {bench!(OriginalMarlin, version, AdditionCircuit, constraints);}
        (2, "addition") => {bench!(MarlinV2, version, AdditionCircuit, constraints);}
        (3, "addition") => {bench_t!(MarlinV3, version, AdditionCircuit, constraints);}
        (4, "addition") => {bench_t!(MarlinV4, version, AdditionCircuit, constraints);}
        (1, "multiple_addition") => {bench!(OriginalMarlin, version, MultipleAdditionCircuit, constraints);}
        (2, "multiple_addition") => {bench!(MarlinV2, version, MultipleAdditionCircuit, constraints);}
        (3, "multiple_addition") => {bench_t!(MarlinV3, version, MultipleAdditionCircuit, constraints);}
        (4, "multiple_addition") => {bench_t!(MarlinV4, version, MultipleAdditionCircuit, constraints);}
        _ => println!("Invalid version"),
    }
    let end_time = Instant::now();
    let duration = end_time - start_time;
    println!("{} {} {} {}{} {:?}", Colorize::bold(Colorize::cyan("Time spended in")), 
        Colorize::bold(Colorize::cyan(circuit)), Colorize::bold(Colorize::cyan("circuit with Marlin version")), 
        Colorize::bold(version.to_string().cyan()), Colorize::bold(Colorize::cyan(":")), duration);

    if groth16 == "true"{
        println!("\n");
        let start_time = Instant::now();
        match circuit{
            "hadamard" => {bench_groth!(HadamardCircuit, constraints);}
            "addition" => {bench_groth!(AdditionCircuit, constraints);}
            "multiple_addition" => {bench_groth!(MultipleAdditionCircuit, constraints);}
            _ => println!(""),
        }
        let end_time = Instant::now();
        let duration = end_time - start_time;
        println!("{} {} {}{} {:?}", Colorize::bold(Colorize::cyan("Time spended in")), 
            Colorize::bold(Colorize::cyan(circuit)), Colorize::bold(Colorize::cyan("circuit with Groth16 version")), 
            Colorize::bold(Colorize::cyan(":")), duration);
    }
}
