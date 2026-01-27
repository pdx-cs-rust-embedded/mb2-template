# mb2-template: MicroBit 2 Rust embedded project template

*[A version of this template as a [Github
Template](https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-repository-from-a-template)
is available on the `main` branch of this repo.]*

To use this [`cargo-generate` template
repository](https://cargo-generate.github.io/cargo-generate/):

1. Follow the instructions linked above to make a Git repo
   for your project.
   
2. If needed, do the following to set up your tools:

       rustup target add thumbv7em-none-eabihf
       rustup component add llvm-tools
       cargo install cargo-binutils
       cargo install --locked probe-rs-tools

3. Edit this `README.md`, the `Cargo.toml` and the stuff in
   `src/` to get the names right and the code to do what you
   need.
