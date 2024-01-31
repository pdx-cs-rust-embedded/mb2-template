# mb2-template: MicroBit 2 Rust embedded project template

To use this [Github template
repository](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template):

1. Follow the instructions linked above to make a Github repo
   for your project.
   
2. If needed, do the following to set up your tools:

      rustup target add thumbv7em-none-eabihf
      rustup component add llvm-tools
      cargo install cargo-binutils
      cargo install probe-rs --features cli

3. Edit this `README.md`, the `Cargo.toml` and the stuff in
   `src/` to get the names right and the template to what
   you need.
