//! Apply Python extension linker arguments only to this crate.

fn main() {
    pyo3_build_config::add_extension_module_link_args();
}
