use tracing::debug;
use wasmtime::*;

#[test]
fn test_basic_wasmtime_runtime() {
    debug!("Compiling module...");
    let engine = Engine::default();
    let module = Module::from_file(&engine, "tests/hello.wat")
        .map_err(|_| panic!())
        .unwrap();

    debug!("Initializing...");
    let mut store = Store::new(&engine, ());

    debug!("Creating callback...");
    let hello_func = Func::wrap(&mut store, |pointer: i32, length: i32| {
        println!("Pointer: {}, length: {}", pointer, length);
        println!("Calling back...");
    });

    debug!("Instantiating module...");
    let imports = [hello_func.into()];
    let instance = Instance::new(&mut store, &module, &imports).unwrap();

    debug!("Extracting export...");
    let run = instance
        .get_typed_func::<(), ()>(&mut store, "run")
        .unwrap();

    debug!("Calling export...");
    run.call(&mut store, ()).map_err(|_| panic!()).unwrap();

    debug!("Done.");
}
