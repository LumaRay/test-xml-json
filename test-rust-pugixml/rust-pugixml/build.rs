extern crate cc;
// sudo apt-get install g++-multilib
fn main() {
    cc::Build::new()
        .file("wrapper/wrapper.cpp")
        .file("pugixml/src/pugixml.cpp")
//        .define("FOO", Some("bar"))
        .include("pugixml/src")
        .cpp_set_stdlib(Some("c++"))
//        .flag("--sysroot=/home/ebarreto/x-tools/i686-unknown-linux-gnu/i686-unknown-linux-gnu/sysroot")
//        .flag("--enable-multiarch")
        .compile("libpugixml.a");
}
