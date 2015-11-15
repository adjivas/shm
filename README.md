# Shm-XSI

[![GPLv3 License](http://img.shields.io/badge/license-GPLv3-blue.svg)](https://www.gnu.org/copyleft/gpl.html)
[![Build Status](https://travis-ci.org/adjivas/shm.svg)](https://travis-ci.org/adjivas/shm)
[![Circle CI](https://circleci.com/gh/adjivas/shm/tree/master.svg?style=svg)](https://circleci.com/gh/adjivas/shm/tree/master)

#### How to build:
```shell
git clone https://github.com/adjivas/shm.git shmxsi && cd shmxsi
cargo build
```

#### How to use:
```shell
cargo run --example box
1
cargo run --example box
2
cargo run --example box
3
```

#### Directory-Tree:
```shell
.
|__ Cargo.toml
|__ LICENSE
|__ README.md
|__ example/box.rs
\__ src
    |__ ffi.rs
    |__ lib.rs
    \__ macros.rs
```

#### License:
*shm*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license](https://github.com/adjivas/shm/blob/master/LICENSE).
