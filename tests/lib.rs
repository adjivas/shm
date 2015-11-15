// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/sem
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate shm;

type Box = [i32; 100];

#[allow(unused_unsafe)]
#[test]
fn the_box () {
    let id: i32 = shmget_id! (
        ftok!().expect("ftok! fail"),
        std::mem::size_of::<Box>()
    ).expect("shmget! fail");
    let addr = shmat!(id).expect("shmat! fail");

    assert_eq!(shmdt!(addr), true);
    assert_eq!(shmctl!(id, shm::ffi::Ipc::RMID), true);
}
