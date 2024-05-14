use std::path::Path;
use wasmtime::*;

type StoreData = ();

pub struct Plugin {
    #[allow(dead_code)]
    engine: Engine,
    module: Module,
    store: Store<StoreData>,
    instance: Option<Instance>,

    _run_func: Option<TypedFunc<(), ()>>,
    _get_version_func: Option<Func>,
}

impl Plugin {
    pub fn new(engine: &Engine, path: impl AsRef<Path>) -> Self {
        let store = Store::new(engine, ());
        let module = Module::from_file(engine, path)
            .map_err(|_| panic!())
            .unwrap();
        Self {
            engine: engine.clone(),
            module,
            store,
            instance: None,
            _run_func: None,
            _get_version_func: None,
        }
    }

    pub fn init(&mut self) {
        // This defines a Rust function that a WASM module can call.
        let hello_func = Func::wrap(&mut self.store, |_caller: Caller<'_, ()>| {
            println!("Calling back...");
        });

        // Import Rust function into WASM module
        let imports = [hello_func.into()];

        // Create a new module instance
        let instance = Instance::new(&mut self.store, &self.module, &imports)
            .map_err(|_| panic!())
            .unwrap();

        // Our WASM module doesn't have `main` function, but we expect `run`.
        // Find the function by type and name.
        let run = instance
            .get_typed_func::<(), ()>(&mut self.store, "run")
            .unwrap();
        self._run_func = Some(run);

        let get_version = instance.get_func(&mut self.store, "get_version").unwrap();
        self._get_version_func = Some(get_version);

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
        let version = &mut [Val::I32(0), Val::I32(0), Val::I32(0)];
        handle_none!(&self._get_version_func)
            .call(&mut self.store, &[], version)
            .expect("Failed to run the `get_version` function!");
        let serialized_version = version
            .iter()
            .flat_map(|a| {
                if let Val::I32(a) = a {
                    Some(a.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join(".");
        println!("Version: {:?} -> {}", version, serialized_version);
        serialized_version
    }
}

#[test]
fn test_basic_struct() {
    tracing_subscriber::fmt::init();

    let engine = Engine::default();
    let mut plugin = Plugin::new(&engine, "tests/hello.wat");
    plugin.init();

    assert_eq!(&plugin.get_version(), "1.2.3");
    plugin.run();
}
