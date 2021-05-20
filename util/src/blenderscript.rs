// Apache 2.0 License

use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

/// Use the "blender" command line tool to output a set of images.
#[inline]
pub fn render_blender<'a, I: Iterator<Item = &'a OsStr>>(
    model: &Path,
    framestart: usize,
    frameend: usize,
    frameout: I,
) -> crate::Result {
    // get the temp directory
    let outdir = dirs::download_dir().expect("No download dir?");

    if !Command::new("blender")
        // render in background
        .arg("-b")
        // render file model
        .arg(model)
        .arg("-E")
        .arg("CYCLES")
        .arg("-s")
        .arg(framestart.to_string())
        .arg("-e")
        .arg(frameend.to_string())
        .arg("-o")
        .arg(outdir.join("render_####.png"))
        .output()?
        .status
        .success()
    {
        return Err(crate::Error::StaticMsg("Render failed"));
    }

    (framestart..=frameend)
        .map(|f| outdir.join(format!("render_{:04}.png", f)))
        .zip(frameout)
        .try_for_each::<_, io::Result<()>>(|(src, dst)| fs::rename(src, dst))?;

    Ok(())
}
