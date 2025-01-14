use spirv_builder::{Capability, SpirvBuilder};
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

fn build_shader(
    path_to_crate: &str,
    codegen_names: bool,
    caps: &[Capability],
) -> Result<(), Box<dyn Error>> {
    let builder_dir = &Path::new(env!("CARGO_MANIFEST_DIR"));
    let path_to_crate = builder_dir.join(path_to_crate);
    let mut builder = SpirvBuilder::new(path_to_crate, "spirv-unknown-vulkan1.1");
    for &cap in caps {
        builder = builder.capability(cap);
    }
    let result = builder.build()?;
    if codegen_names {
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("entry_points.rs");
        fs::create_dir_all(&out_dir).unwrap();
        fs::write(&dest_path, result.codegen_entry_point_strings()).unwrap();
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    build_shader("../../../shaders/sky-shader", true, &[])?;
    build_shader("../../../shaders/simplest-shader", false, &[])?;
    // We need the int8 capability for using `Option`
    build_shader(
        "../../../shaders/compute-shader",
        false,
        &[Capability::Int8],
    )?;
    build_shader("../../../shaders/mouse-shader", false, &[])?;
    Ok(())
}
