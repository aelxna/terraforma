# terraforma: building worlds with fBm

## Dependencies
- cargo
- numpy
- matplotlib

### Nix flake
A flake is provided containing Cargo and the Python dependencies.

### Pip

A `requirements.txt` file is provided if you desire to manage the Python dependencies with Pip.

## Building

To build, run the following:
```bash
cargo build --release
```
This will place the Rust binary at `./target/release/terraforma`.

## Running

Execute the following script to run the program:
```bash
./run.sh
```
If you want to use a previously generated `config.txt`, run the following:
```bash
cat config.txt | ./run.sh
```
