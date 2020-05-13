# flatbuffers-poc
POC with flatbuffers using Java and Rust programs to comunicate each-other

# About Flatbuffers
* https://google.github.io/flatbuffers

# Install Compiler
```bash
sudo apt install flatbuffers-compiler
```

# Script
* Started following the Tutorial in https://google.github.io/flatbuffers/flatbuffers_guide_tutorial.html
* Create schema file in flatbuffer/monster.fbs
* Compile schema to destination language
```bash
flatc --rust monster.fbs  # Generate monster_generate.rs
flatc --java monster.fbs  # Generate MyGame package
```
* Compile and run Rust project
```bash
cd flatbuffer_rs
cargo run flatbuffer_rs
```
* It should generate protobufer file in /tmp/flatbuffer.dat
* Compile and run Java project
```bash
cd flatbuffer_java
gradle run
```
* It should show you some of the values of the created monster 

# References
* https://google.github.io/flatbuffers/
* https://codeburst.io/json-vs-protocol-buffers-vs-flatbuffers-a4247f8bda6f
