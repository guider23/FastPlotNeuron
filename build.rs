use pyo3_build_config::pyo3_build_script_impl::{cargo_env_var, errors::Result};

fn main() -> Result<()> {
    
    pyo3_build_config::add_extension_module_link_args();
    Ok(())
}
