use anyhow::*;

pub struct ShaderLoader;
impl ShaderLoader {
    /// A way of loading WGSL shaders inline using GFX-21 (newer than built in wgpu-rs version)
    pub fn load_wgsl(source: &'static str) -> Result<Vec<u32>> {
        use naga::{back::spv, front::wgsl, valid::Validator};
        
        let module = wgsl::parse_str(source)?;
        
        let info = Validator::new(naga::valid::ValidationFlags::all()).validate(&module)?;
        let options = spv::Options::default();
        
        let res = spv::write_vec(&module, &info, &options)?;
        
        Ok(res)
    }
}