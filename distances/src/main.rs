// main.rs
// :PROPERTIES:
// :header-args: :tangle src/main.rs
// :END:

// [[file:~/Workspace/Programming/rust-scratch/distances/distances.note::*main.rs][main.rs:1]]
use quicli::main;
use quicli::prelude::*;

use distances::*;

/// Calculate distance matrix from atoms in pdb file
#[derive(Debug, StructOpt)]
struct Cli {
    /// Input file name to a pdb file
    inpfile: String,

    /// Output file name for storing distance matrix
    #[structopt(long = "out", short = "o")]
    outfile: String,

    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    verbosity: Verbosity,
}

main!(|args: Cli, log_level: verbosity| {
    info!("Hello world from rust!");

    let positions = read_positions_from_pdb(&args.inpfile)?;
    let ds = get_distances(&positions, 5.0);
    info!("done!");
});
// main.rs:1 ends here
