# Artifact for the Lola to Rust Compilation

The repository contains an artifact for the [Verified Rust Monitors for Lola Specifications](https://www.react.uni-saarland.de/publications/FOPS20.html) paper to appear at RV'20.

## Structure
There are artifacts for two specifications: a simple altimeter sensor validation and a network monitoring specification.
* The `altimeter` directory contains randomly generated input data `altitude_data.txt`, a functional Rust project, the verifiable `provable/motivating_example_proof.rs` file, and the specification `altimeter.lola`.
* The `network` directory contains randomly generated input data `network_data.txt`, a functional Rust project, the verifiable `provable/network_proof.rs` file, and the specification `network.lola`.

The difference between the executable and provable files is that the Viper code annotations are removed and the I/O functions were adapted accordingly.  You can compile an run the monitors by switching into the respective folder an run `cargo run --release`.  Note that you need to install Rust first, check it out [here](https://www.rust-lang.org/tools/install).