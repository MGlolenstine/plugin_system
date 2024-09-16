use bypar::prelude::*;
use bypar_derive::{FromBytes, ToBytes};
use std::path::Path;
use wasmtime::*;

type StoreData = ();

#[derive(Debug, Clone, FromBytes, ToBytes)]
#[enum_index_type(u16)]
pub enum Msg {
    #[enum_index(0)]
    Version(i32, i32, i32),
    #[enum_index(1)]
    StringData(UnsizedString),
}

pub struct Plugin {
    #[allow(dead_code)]
    engine: Engine,
    module: Module,
    store: Store<StoreData>,
    instance: Option<Instance>,

    _run_func: Option<TypedFunc<(), ()>>,
    _get_version_func: Option<TypedFunc<(), (i32, i32, i32)>>,
    _get_bytes_func: Option<TypedFunc<(), (i32, i32)>>,
}

impl Plugin {
    pub fn new(engine: &Engine, path: impl AsRef<Path>) -> Self {
        let store = Store::new(engine, ());
        let module = Module::from_file(engine, path).unwrap();
        Self {
            engine: engine.clone(),
            module,
            store,
            instance: None,
            _run_func: None,
            _get_version_func: None,
            _get_bytes_func: None,
        }
    }

    pub fn init(&mut self) {
        // This defines a Rust function that a WASM module can call.
        let hello_func = Func::wrap(&mut self.store, |pointer: i32, length: i32| {
            println!("Pointer: {}, length: {}", pointer, length);
            println!("Calling back...");
        });

        // Import Rust function into WASM module
        let imports = [hello_func.into()];

        // Create a new module instance
        let instance = Instance::new(&mut self.store, &self.module, &imports).unwrap();

        // Our WASM module doesn't have `main` function, but we expect `run`.
        // Find the function by type and name.
        let run = instance
            .get_typed_func::<(), ()>(&mut self.store, "run")
            .unwrap();
        self._run_func = Some(run);

        let get_version = instance
            .get_typed_func::<(), (i32, i32, i32)>(&mut self.store, "get_version")
            .unwrap();
        self._get_version_func = Some(get_version);

        let get_bytes = instance
            .get_typed_func::<(), (i32, i32)>(&mut self.store, "get_bytes")
            .unwrap();
        self._get_bytes_func = Some(get_bytes);

        self.instance = Some(instance);
    }
}

macro_rules! handle_none {
    ($e:expr) => {
        if let Some(e) = $e {
            e
        } else {
            panic!("Plugin needs to be inited before it can be used!");
        }
    };
}

impl Plugin {
    pub fn run(&mut self) {
        handle_none!(&self._run_func)
            .call(&mut self.store, ())
            .expect("Failed to run the `run` function!")
    }

    pub fn get_version(&mut self) -> String {
        let (major, minor, patch) = handle_none!(&self._get_version_func)
            .call(&mut self.store, ())
            .expect("Failed to run the `get_version` function!");
        let serialized_version = format!("{major}.{minor}.{patch}");
        println!("Version: {}", serialized_version);
        serialized_version
    }

    pub fn recv(&mut self) -> Option<Msg> {
        let (pointer, length) = handle_none!(&self._get_bytes_func)
            .call(&mut self.store, ())
            .expect("Failed to run the `get_bytes` function!");

        let memory = self
            .instance
            .unwrap()
            .get_memory(&mut self.store, "memory")
            .unwrap();

        let msg = {
            let mut slice = vec![0u8; length as usize];
            memory
                .read(&self.store, pointer as usize, &mut slice)
                .expect("Failed to read the memory!");

            slice

            // let msg = Msg::from_bytes(&slice).unwrap();
            // msg
        };

        dbg!(msg);

        // let mut caller = Caller::new(&self.store);
        // let memory = self.instance.exports.get_memory("memory")?;
        // let data = memory.data(&caller, 0, array_len as usize)?;
        // let extracted_array: &[u8] = &data[0..array_len as usize];

        None
    }
}

#[test]
fn test_basic_struct() {
    let engine = Engine::default();
    let mut plugin = Plugin::new(&engine, "tests/hello.wat");
    plugin.init();

    assert_eq!(&plugin.get_version(), "1.2.3");
    plugin.run();
}

#[test]
fn test_communication_parse() {
    let engine = Engine::default();
    let mut plugin = Plugin::new(&engine, "tests/hello.wat");
    plugin.init();

    assert!(plugin.recv().is_none());
    plugin.run();
}
