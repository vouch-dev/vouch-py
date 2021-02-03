use vouch_lib::extension::Extension;
use vouch_py_lib;

fn main() {
    let mut extension = vouch_py_lib::PyExtension::new();
    vouch_lib::extension::commands::run(&mut extension).unwrap();
}
