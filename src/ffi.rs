// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/shm
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `Ipc` enum is a POSIX Standard
/// for System V.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Shm {
  READ    = 0o0000400,
  WRITE   = 0o0000200,
  RDONLY  = 0o0010000,
  REMAP   = 0o0040000,
  EXEC    = 0o0100000,
  LOCK    = 0o0000011,
  UNLOCK  = 0o0000012,
  STAT    = 0o0000013,
  INFO    = 0o0000014,
  DEST    = 0o0001000,
  LOCKED  = 0o0002000,
  HUGETLB = 0o0004000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Ipc {
  CREAT  = 0o0001000,
  NOWAIT = 2048,
  EXCL   = 0o0002000,
  RMID   = 0o0000000,
  SET    = 0o0000001,
  STAT   = 0o0000002,
  INFO   = 0o0000003,
}

/// The `TOK_*, MSG_BUFF` const are default values
/// for macros.

#[allow(dead_code)]
pub const TOK_PATHNAME: &'static [u8; 4] = b"/tmp";
pub const TOK_PROJ_ID: u32 = 0;

/// The `C` extern is list of libc functions required
/// by the project.

#[cfg(any(unix))]
extern "C" {
  pub fn ftok(path: *mut i8, id: i32) -> i64;
  pub fn shmget(key: i32, size: u64, flag: i32) -> i32;
  pub fn shmat(id: i32, addr: *mut i32, flag: i32) -> *mut i32;
  pub fn shmdt(addr: *const i32) -> i32;
}
