# Unsafe Rust Notes

## Basics
- unsafe rust used for 
    - deref raw ptr
    - call unsafe fn
    - impl unsafe trait
    - access/modify mut static var
    - access fields of union

## Deref Raw Ptr
- not bound by borrow rules
    - mut and non mut allowed at same time
- can be null/invalid memory
- not auto cleaned up
- init like
```rust
let mut s = "Hello".to_owned();
let r1 = &s as *const String;
let r2 = &mut s as *mut String;
```
- always safe to set var to raw ptr
- UNSAFE to deref raw ptr
- deref ptr
```rust
(*raw2).push_str("!!");
println!("raw1 is: {}", *raw1);
```
- raw ptr is useful for 
    - interop with C code
    - greater performance
    - build safe abstractions that borrow checker can't check
    - interop with HW

## Call unsafe Fn
- fn created unsafe using `unsafe fn my_fn()`
- if unsafe, need to enforce invariance
- body of unsafe fn treated like unsafe block -> can write unsafe code there
    - could be changing in future

## Impl Unsafe trait
- when trait marked unsafe, all impl of trait must be marked unsafe as well
- not common but good example is std::alloc::GlobalAlloc
    - define mem allocator

## Inline Assembly
- unsafe block allows for writing assembly code
- can increase performance at cost of portability & useability

## FFI (Foreign Function Interface)
- From C to Rust
    - define fn that can be called from other lang
    - call from rust
    ```rust
    #[link(name = "adder", kind="static")]
    extern "C" { fn add(a: i32, b: i32) -> i32;}

    fn main() {
        let x: i32;
        unsafe {
            x = add(1,2);
            println!("x is {}", x);
        }
    }
    ```
    - where
        - `"C"` ref the ABI (Application Binary Interface) to be used for the link
        - `#[link]` -> defines linker props (name and type of binary)
        - `fn add(...)` is the interface code including arg types
- From Rust to C
    - create pub fn in rust library crate
    ```rust
    #[no_mangle]
    pub extern "C" fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    ```
    - where
        - `extern "C"` -> defines ABI to use and that fn will be called externally
        - `#[no_mangle]` -> tells rust compiler to not change fn sig
    - also need to to add `[lib] crate-type=["cdylib"]` in cargo.toml file
        - indicates to cargo that it should compile to c lib
- Rust from Python
    - setup for Python
        - create new venv
        - `pip install maturin`
        - run `maturin init`
        - select `pyo3`
    - above will create dirs/files for rust and python
        - creates cargo.toml for control of rust
        - creates src/lib.rs file with example code for how pyo3 builds & exposes modules/fns to python
    - once done with rust code
        - run `maturin develop` to create python modules from rust code
        - also installs packages into venv
    - create py files as usual & import lib/package as usual