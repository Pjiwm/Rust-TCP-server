# Rust-TCP-server
Simple TCP server built in rust.

### Setup and connecting to/hosting server:

Using Docker and remote containers on WSL no software needs to be installed.
Inside the container compile the code and connect with `nc 127.0.0.1 {Your port}`.
Compiling and running can be done using: `cargo run 12345` Any valid port will work.
Using the win_compile.sh outside the containers terminal the code can be compiled to an exe for windows.

### Running tests:

Run tests:`cargo test`.
Tests will run in the pipeline as well (both nightly and stable build)
