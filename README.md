# Shm-XSI

[![Crate][crate-badge]][crate] [![docs-badge][]][docs] [![license-badge][]][license] [![travis-badge][]][travis] [![circle-badge][]][circle]

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
|__ example
|   |__ map.rs
|   |__ box.rs
|   \__ clr.rs
\__ src
    |__ ffi.rs
    |__ lib.rs
    \__ macros.rs
```

#### License:
*shm*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license][license].

[crate-badge]: https://img.shields.io/badge/crates.io-v0.1.0-orange.svg?style=flat-square
[crate]: https://crates.io/crates/shm
[docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
[docs]: http://adjivas.github.io/shm/shm
[license-badge]: http://img.shields.io/badge/license-GPLv3-blue.svg?style=flat-square
[license]: https://github.com/adjivas/shm/blob/master/LICENSE
[travis-badge]: https://travis-ci.org/adjivas/shm.svg?style=flat-square
[travis]: https://travis-ci.org/adjivas/shm
[circle-badge]: https://circleci.com/gh/adjivas/shm/tree/master.svg?style=svg
[circle]: https://circleci.com/gh/adjivas/shm/tree/master
