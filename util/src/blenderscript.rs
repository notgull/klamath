// Apache 2.0 License

use std::path::PathBuf;

/// Use the "blender" command line tool to output a set of images.
#[inline]
pub fn render_blender(framestart: usize, frameend: usize, frameout: Vec<PathBuf>) -> crate::Result {
    Ok(())    
}
