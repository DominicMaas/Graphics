use anyhow::*;
use glob::glob;
use naga::back::spv;
use std::fs::{read_to_string, write};
use std::path::PathBuf;

struct ShaderData {
    src: String,
    src_path: PathBuf,
    spv_path: PathBuf,
}

impl ShaderData {
    pub fn load(src_path: PathBuf) -> Result<Self> {
        let src = read_to_string(src_path.clone())?;
        let spv_path = src_path.with_extension("spv");

        Ok(Self {
            src,
            src_path,
            spv_path,
        })
    }
}

fn main() -> Result<()> {
    // Collect all shaders recursively within /src/
    let mut shader_paths = [glob("./src/**/*.wgsl")?];

    // This could be parallelized
    let shaders = shader_paths
        .iter_mut()
        .flatten()
        .map(|glob_result| ShaderData::load(glob_result?))
        .collect::<Vec<Result<_>>>()
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

    for shader in shaders {
        // This tells cargo to rerun this script if something in /src/ changes.
        println!(
            "cargo:rerun-if-changed={}",
            shader.src_path.as_os_str().to_str().unwrap()
        );

        let module = naga::front::wgsl::parse_str(&shader.src)
            .map_err(|e| {
                println!("{:#?}", e);
                e
            })
            .unwrap();

        // Output to SPIR-V
        let info = naga::valid::Validator::new(naga::valid::ValidationFlags::all())
            .validate(&module)
            .unwrap();
        let options = naga::back::spv::Options::default();
        let spv = spv::write_vec(&module, &info, &options).unwrap();

        let bytes = spv
            .iter()
            .fold(Vec::with_capacity(spv.len() * 4), |mut v, w| {
                v.extend_from_slice(&w.to_le_bytes());
                v
            });

        write(shader.spv_path, bytes.as_slice()).expect("Couldn't write SPIR-V shader file");
    }

    Ok(())
}
