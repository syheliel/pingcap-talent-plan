task website: https://github.com/pingcap/talent-plan/tree/master/courses/rust/projects/project-1#user-content-project-spec
# task 1(Done)
add [dev-dependencies] and struct in lib.rs
# task 2(Done)
```
kvs set <KEY> <VALUE>
```
Set the value of a string key to a string
```
kvs get <KEY>
```
Get the string value of a given string key
```
kvs rm <KEY>
```
Remove a given key
```
kvs -V
```
version of kvs
# task 3(Done)
Modify your clap setup to set these values(set the name, version, authors, and description (if not, do so)) from standard cargo environment variables.
# task 4:Store values in memory(Done)
Now that your command line scaffolding is done, let's turn to the implementation of KvStore, and make the remaining test cases pass.
The behavior of KvStore's methods are fully-defined through the test cases themselves — you don't need any further description to complete the code for this project.
Make the remaining test cases pass by implementing methods on KvStore.
# task 5: Documentation(Done)
Add #![deny(missing_docs)] to the top of src/lib.rs to enforce that all public items have doc comments. Then add doc comments to the types and methods in your library. Follow the documentation guidelines. Give each an example and make sure they pass cargo test --doc.
# task 6: Ensure good style with clippy and rustfmt(Done)
cargo clippy
cargo rustfmt

# Extension 1: structopt(Done)
Modify your program to use structopt for parsing command line arguments instead of using clap directly.