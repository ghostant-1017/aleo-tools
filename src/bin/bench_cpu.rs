use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use rand::{RngCore, thread_rng};
use snarkvm::circuit::AleoV0;
use snarkvm::prelude::{Address, MainnetV0, Network};
use snarkvm::prelude::puzzle::{Puzzle, PuzzleTrait};
use snarkvm_ledger_puzzle_epoch::SynthesisPuzzle;

fn main() {
    let num_cpus = num_cpus::get();
    let puzzle = Puzzle::new::<SynthesisPuzzle<MainnetV0, AleoV0>>();
    let acc = Arc::new(AtomicU64::new(0));
    let address = Address::from_str("aleo15qwecrrmvf53x4npnz6aj5xlcs0a7524ukz7hr9cvaxw5av98uxqdq0ula").unwrap();
    let epoch_hash = std::env::var("EPOCH_HASH").unwrap();
    let epoch_hash = <MainnetV0 as Network>::BlockHash::from_str(&epoch_hash).unwrap();
    for _ in 0..num_cpus {
        let puzzle = puzzle.clone();
        let acc = acc.clone();
        std::thread::spawn(move || {
            loop {
                let counter = &mut thread_rng();
                let _ = puzzle.prove(epoch_hash, address, counter.next_u64(), None).unwrap();
                acc.fetch_add(1, Ordering::Relaxed);
            }
        });
    }

    loop {
        let before = acc.load(Ordering::Relaxed);
        std::thread::sleep(std::time::Duration::from_secs(10));
        let after = acc.load(Ordering::Relaxed);
        println!("Proofs per second: {}", (before - after) as f64 / 10.0);
    }
}



fn prove(puzzle: Puzzle<MainnetV0>, epoch_hash: <MainnetV0 as Network>::BlockHash, address: Address<MainnetV0>){
    let counter = &mut thread_rng();
    let _ = puzzle.prove(epoch_hash, address, counter.next_u64(), None).unwrap();
}