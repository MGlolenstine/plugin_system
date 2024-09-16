use plugin_system::plugin::{Msg, Plugin};
use wasmtime::*;

fn main() {
    let engine = Engine::default();
    let mut plugin = Plugin::new(&engine, "tests/hello.wat");
    plugin.init();

    plugin.run();

    for _ in 0..3 {
        let msg = plugin.recv();

        match msg {
            Some(Msg::StringData(string)) => {
                println!("Received the string data! {}", &*string);
            }
            Some(Msg::Version(major, minor, patch)) => {
                println!("Received version: {}.{}.{}", major, minor, patch);
            }
            None => {
                println!("Received no callback!");
            }
        }
    }
}
