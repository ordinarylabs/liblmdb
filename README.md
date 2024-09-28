# liblmdb

up to date Rust bindings for LMDB.

version matches LMDB version for which the bindings were generated.

```sh
## setup
cargo install bindgen-cli && git submodule update --init

## checkout mdb.master and pull latest
git submodule switch mdb.master; git pull

## generate bindings
bindgen lmdb/libraries/liblmdb/lmdb.h -o src/lmdb.rs

## build
cargo build
```

update the version to match what is reflected in `/lmdb/libraries/liblmdb/lmdb.h` for `MDB_VERSION_(MAJOR | MINOR | PATCH)` before deploying.