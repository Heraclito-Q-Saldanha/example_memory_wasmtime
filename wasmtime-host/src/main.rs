fn main() {
    let engine = wasmtime::Engine::default();
    let module = wasmtime::Module::from_file(&engine, "../wasmtime-client/target/wasm32-unknown-unknown/release/wasmtime_client.wasm").unwrap();
    let mut store = wasmtime::Store::new(&engine, ());
    let instance = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
    let hello_fn = instance.get_typed_func::<(), u32>(&mut store, "hello").unwrap();
    let pointer = hello_fn.call(&mut store, ()).unwrap();
    let memory = instance.get_memory(&mut store, "memory").unwrap();
    let mut buffer = [0u8;5];
    memory.read(&store, pointer as usize, &mut buffer).unwrap();
    let string = String::from_utf8_lossy(&buffer);
    println!("{string:?}");
}