# Builder

A Rust project for building something.

## Coverage

This project uses `cargo-llvm-cov` for code coverage.

### Local Coverage

To run coverage locally and open the HTML report:

```bash
./run-coverage.sh
```

Alternatively, if you have `cargo-llvm-cov` installed:

```bash
cargo llvm-cov --html --open
```

### CI Coverage

Coverage is automatically calculated and uploaded to Codecov on every push to the `main` branch and on pull requests.
