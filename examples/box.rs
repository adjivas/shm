// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/shm
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate shm;

type Box = [i32; 80];

fn main () {
    let size:usize = std::mem::size_of::<Box>();

    if let Some(key) = ftok!() {
        if let Some(id) = shmget_id!(key, size) {
            if let Some(addr) = shmat!(id) {
                let mut data: &mut Box = unsafe {
                    std::mem::transmute(addr)
                };

                data[0] += 1;
                println!("{:?}", data[0]);
            }
        }
    }
}
