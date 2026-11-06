# Create a rust crate 

- There are two types of crate, binary crate and library crate

```bash
cargo new 1-helloworld --name demo
```

```bash
# run rust application
cargo run 
# build rust application to the debug build which is target/debug/<nameofthecrate>
cargo build 
# build rust application to the debug build which is target/release/<nameofthecrate>
# relase builds are generated to release the project which are generally small in size
cargo build --release 
```
