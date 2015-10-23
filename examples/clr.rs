// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/shm
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate shm;

type Example = [i32; 100];

fn main () {
    let size:usize = std::mem::size_of::<Example>();

    if let Some(key) = ftok!() {
        if let Some(id) = shmget_id!(key, size) {
            if let Some(addr) = shmat!(id) {
                shmdt!(addr);
                println!("work: {}", shmctl!(id, shm::ffi::Ipc::RMID));
            }
        }
    }
}
