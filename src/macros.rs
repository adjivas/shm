// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/shm
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `ftok` macro returns the System-V'IPC
/// key from pathname.

#[macro_export]
macro_rules! ftok {
    () => ({
        extern crate shm;
        ftok!(shm::ffi::TOK_PATHNAME)
    });
    ($pathname: expr) => ({
        extern crate shm;
        match unsafe {
            shm::ffi::ftok (
                $pathname.as_ptr() as *mut i8,
                shm::ffi::TOK_PROJ_ID as i32
            )
        } {
            -1 => None,
            key => Some(key as u64),
        }
    });
}

#[macro_export]
macro_rules! shmget {
    ($key: expr, $flag: expr, $size: expr) => ({
        extern crate shm;
        match unsafe {
            shm::ffi::shmget (
                $key as i32,
                $size as u64,
                $flag as i32
            )
        } {
            -1 => None,
            key => Some(key as i32),
        }
    });
}

#[macro_export]
macro_rules! shmat {
    ($id: expr) => ({
        extern crate std;
        shmat!($id, std::ptr::null_mut(), 0)
    });
    ($id: expr, $addr: expr) => ({
        shmat!($id, $addr, 0)
    });
    ($id: expr, $addr: expr, $flag: expr) => ({
        extern crate shm;
        match unsafe {
            shm::ffi::shmat (
                $id as i32,
                $addr,
                $flag as i32,
            )
        } {
            addr if addr.is_null() => None,
            addr => Some(addr),
        }
    });
}

#[macro_export]
macro_rules! shmget_id {
    ($key: expr, $size: expr) => ({
        match shmget! (
            $key,
            0o0666 | shm::ffi::Ipc::CREAT as i32
                   | shm::ffi::Ipc::EXCL as i32,
            $size
        ) {
            Some(id) => Some(id),
            None => shmget! (
                $key,
                0o0666,
                $size
            ),
        }
    });
}

#[macro_export]
macro_rules! shmdt {
    ($addr: expr) => ({
        extern crate shm;
        match unsafe {
            shm::ffi::shmdt (
                $addr
            )
        } {
            -1 => false,
            _ => true,
        }
    });
}

#[macro_export]
macro_rules! shmctl {
    ($id: expr, $cmd: expr) => ({
        extern crate std;
        shmctl!($id, $cmd, std::ptr::null_mut())
    });
    ($id: expr, $cmd: expr, $info: expr) => ({
        extern crate shm;
        match unsafe {
            shm::ffi::shmctl (
                $id,
                $cmd as i32,
                $info
            )
        } {
            -1 => false,
            _ => true,
        }
    });
}
