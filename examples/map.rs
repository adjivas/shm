// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/shm
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate shm;

type Map = [i32; 10];

fn main () {
    let size:usize = std::mem::size_of::<Map>();

    if let Some(key) = ftok!() {
        if let Some(id) = match shmget! (
            key,
            0o0666 | shm::ffi::Ipc::CREAT as i32 | shm::ffi::Ipc::EXCL as i32,
            size
        ) {
            Some(id) => Some(id),
            None => shmget! (
                key,
                0o0666,
                size
            ),
        } {
            if let Some(addr) = shmat!(id) {
                let map: &mut Map = unsafe {
                    std::mem::transmute(addr)
                };

                loop {
                    for x in map.iter() {
                        print!("{} ", x);
                    }
                    println!("");
                }
            }
        }
    }
}
