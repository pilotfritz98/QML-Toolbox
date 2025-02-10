pub mod sim;
pub mod variance;
pub mod iqp;
pub mod toy;
pub mod qasmcircuit;

use std::env;
use quizx::circuit::*;


fn circuit_from_qasm(qasm: &String) -> Circuit {
    
    quizx::circuit::Circuit::from_qasm(qasm).expect(&format!("Circuit could not be parsed: {}", qasm))

}


fn main() {

    let args: Vec<String> = env::args().collect();
    let qasm = &args[1];
    let layers: usize = args[2].parse().unwrap();
    let h = &args[3];

    let mut param = 0;
    if args.len() > 4 {
        param = args[4].parse().unwrap();
    }

    let mut c = circuit_from_qasm(qasm);
    let mut qubits = c.num_qubits();
    variance::select_target(&mut c, param);

    let var = variance::compute_variance(&mut c, &h);
    println!("{var}");

}
