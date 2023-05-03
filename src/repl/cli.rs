use crate::ast::{Module, TExpr, Pat, InfixOp, Expr, Variable};
use crate::transform::{compile, collect_module_variables, FieldOps};

use ark_bls12_381::{Bls12_381, Fr as BlsScalar};
use halo2_proofs::pasta::{EqAffine, Fp};
use crate::plonk::synth::{PrimeFieldOps as PlonkPrimeFieldOps};
use crate::halo2::synth::{PrimeFieldOps as Halo2PrimeFieldOps};

use std::collections::HashMap;
use serde_json::Map;

use std::fs; 
use std::fs::File;
use std::io::{BufWriter, BufReader, BufRead, Write};
use std::fs::OpenOptions;

use clap::{Args, Subcommand};
use std::path::PathBuf;

use num_bigint::BigInt;


#[derive(Subcommand)]
pub enum REPLCommands {
    /// Run repl evaluated using Halo2 field.
    Halo2(Halo2),
    /// Run repl evaluated using Plonk field.
    Plonk(Plonk),
}


#[derive(Args)]
pub struct Halo2 {
    /// Path to source file that is being loaded into repl
    #[arg(short, long)]
    source: Option<PathBuf>,
}


#[derive(Args)]
pub struct Plonk {
    /// Path to source file that is being loaded into repl
    #[arg(short, long)]
    source: Option<PathBuf>,
}

pub fn repl_cmd(source: &Option<PathBuf>, field_ops: &dyn FieldOps) {
    let mut module = Module::default();

    if let Some(path) = source {
        let unparsed_file = fs::read_to_string(path).expect("cannot read file");
        module = Module::parse(&unparsed_file).unwrap();
        println!("Entering REPL with module loaded from file.");
        // println!("{}", module);
    } else {
        println!("Entering REPL with no module loaded.");
    }

    loop {
        print!("> ");
        std::io::stdout().flush().expect("Error flushing stdout");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Error reading from stdin");

        if input.trim() == "quit" || input.trim() == "exit" {
            break;
        }

        println!("{}", input);
    }
}


/* Implements the subcommand that writes witnesses to a JSON file. */
pub fn halo2_repl_cmd(Halo2 { source }: &Halo2) {
    println!("** Halo2 Repl");
    repl_cmd(&source, &Halo2PrimeFieldOps::<Fp>::default());
}

/* Implements the subcommand that writes circuit into comma separated, three-address file. */
pub fn plonk_repl_cmd(Plonk { source }: &Plonk) {
    println!("** Plonk Repl");
    repl_cmd(&source, &PlonkPrimeFieldOps::<BlsScalar>::default());
}

pub fn repl(repl_commands: &REPLCommands) {
    match repl_commands {
        REPLCommands::Halo2(args) => halo2_repl_cmd(args),
        REPLCommands::Plonk(args) => plonk_repl_cmd(args),
    }
}

