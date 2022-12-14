# Rust-async-training

Simple multi-threaded application with 2 engines connected to one fuel tank.
There is also refiller, which refills fuel tank if there is no fuel. Each engine is working in separate thread and consuming the fuel. Refiller is working in another thread and checking if fuel tank is empty, if yes then refilling it.
Application created for training of "tokio" async library and usage of Arc<T> with Mutex<T> types.