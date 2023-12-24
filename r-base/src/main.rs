use std::{thread::sleep, time::Duration};

use r_base::SecurityAgent;

extern crate libloading as lib;

type ExtentAgent = unsafe fn() -> *mut dyn SecurityAgent;

const LIBRARYIES: [&str; 2] = ["/home/hao/projects/rust-demo/lib/libr_dynamic.so", "/home/hao/projects/rust-demo/lib/1/libr_dynamic.so"];

fn main() {
    println!("Hello, world!");
    for path in LIBRARYIES {
        unsafe {
            let lib = lib::Library::new(path).unwrap();
            let func: lib::Symbol<ExtentAgent> = lib.get(b"_load_agent").unwrap();
            let box_raw = func();
            let agent = Box::from_raw(box_raw);
            agent.on_load();
            drop(agent);
            let _ = lib.close().unwrap();
        }
        println!("------------------------");
        sleep(Duration::from_secs(3));
    }
    
}
