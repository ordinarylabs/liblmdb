use cc;

fn main() {
    let mut build = cc::Build::new();

    build
        .file("lmdb/libraries/liblmdb/mdb.c")
        .file("lmdb/libraries/liblmdb/midl.c")
        .opt_level(2)
        .warnings(false)
        .compile("liblmdb.a");
}
