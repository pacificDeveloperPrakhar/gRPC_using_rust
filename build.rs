fn main() {
    // println!("cargo:rerun-if-changed=build.rs");

    // They are Cargo build-script directives, 
    
    // =>telling Cargo how to:
    // =>re-run the build script
    // =>show warnings
    // =>link native libraries
    // =>pass cfg flags
    // =>set compile-time variables

    let proto_file = "./users.proto";
    tonic_prost_build::configure()
        .build_server(true)
        .out_dir("./src")
        .compile_protos(&[proto_file], &["."]).expect("unable to compile the file");
    println!("cargo:rerun-if-changed={}", proto_file);
}
