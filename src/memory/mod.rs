#![cfg(windows)]

// imports
use sysinfo::{ProcessExt, System, SystemExt};

#[cfg(feature = "memory")]
pub mod structures;

// Process
struct Process {
    _pid: i32
}

impl Process {

}

// Public Functions
pub fn pid(pname: &str) -> usize {
    let s = System::new_all();

    let mut _pid: &usize = &0;

    // TODO: replace with normal winapi functions later
    for (pid, process) in s.get_processes() {
        if process.name() == pname {
            _pid = pid;
        }
    }

    *_pid
}