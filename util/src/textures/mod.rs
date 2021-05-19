// Apache 2.0 License

mod types;

//use rayon::prelude::*;
use std::{
    array::IntoIter as ArrayIter,
    collections::HashMap,
    fs::{self, File},
    io::{self, prelude::*, BufReader, BufWriter, ErrorKind},
    iter, mem,
    path::{Path, PathBuf},
};
use tinyvec::ArrayVec;
use types::{Flat, Material, Matinfo, Patch, Texture};

#[inline]
pub fn process_textures(
    pnames_path: &Path,
    texture1_path: &Path,
    patchfolder: PathBuf,
    flatfolder: PathBuf,
    wadinfo_in: &Path,
    wadinfo_out: &Path,
    matdir: &Path,
) -> crate::Result {
    // read in the matinfo from stdin
    let stdin = io::stdin();
    let mut cin = stdin.lock();
    let Matinfo {
        flats,
        patches,
        textures,
        ..
    } = Matinfo::load(&mut cin)?;
    mem::drop(cin);

    let mut pnames = Pnames::new();
    let mut texture1 = Texture1::new();

    //    eprintln!("Read matinfo");

    let mut wadinfo_in = BufReader::new(File::open(wadinfo_in)?);
    let mut wadinfo_out = BufWriter::new(File::create(wadinfo_out)?);
    io::copy(&mut wadinfo_in, &mut wadinfo_out)?;

    //    eprintln!("copied wadinfo_in to wadinfo_out");

    // first, write all the files we can to the flat directory
    fs::create_dir_all(&flatfolder)?;
    writeln!(wadinfo_out, "[flats]")?;
    flats
        .into_iter()
        .map(|Flat { name, mat }| {
            let fname = flatfolder.join(format!("{}.png", &name));
            // use a hard link to avoid copying the file
            replace_hard_link(&matdir.join(&mat.file), &fname)?;

            wadinfo_out.write_all(format!("{}\n", name).as_bytes())?;
            Ok(())
        })
        .collect::<crate::Result>()?;

    //    eprintln!("Wrote flats");

    // then, write all the patch names to a file. the patch lump consists of the number of patches (4 bytes)
    // followed by the eight-byte name for each patch name
    fs::create_dir_all(&patchfolder)?;
    writeln!(wadinfo_out, "[patches]")?;
    patches
        .into_iter()
        .map(|patch| {
            let fname = patchfolder.join(format!("{}.png", &patch.name));
            // TODO: transforms
            replace_hard_link(&matdir.join(&patch.mat.file), &fname)?;
            writeln!(wadinfo_out, "{}", &patch.name)?;
            let i = pnames.add_name(&patch.name);
            patch.assign_index(i);
            Ok(())
        })
        .collect::<crate::Result>()?;
    pnames.write(&mut BufWriter::new(File::create(pnames_path)?))?;

    // finally, write the textures lump
    textures
        .into_iter()
        .for_each(|tex| texture1.push_texture(tex));
    texture1.write(&mut BufWriter::new(File::create(texture1_path)?))?;

    Ok(())
}

#[inline]
fn replace_hard_link<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> crate::Result {
    let dstp: &Path = dst.as_ref();
    match fs::remove_file(dstp) {
        Ok(()) => (),
        Err(e) if e.kind() == ErrorKind::NotFound => (),
        Err(e) => return Err(e.into()),
    }

    fs::hard_link(src, dstp)?;
    Ok(())
}

struct Pnames {
    names: Vec<u8>,
}

impl Pnames {
    #[inline]
    fn new() -> Self {
        Self { names: vec![] }
    }

    #[inline]
    fn add_name(&mut self, name: &str) -> usize {
        self.names
            .extend(name.bytes().chain(iter::repeat(0)).take(8));
        (self.names.len() / 8) - 1
    }

    #[inline]
    fn write<W: Write>(self, out: &mut W) -> crate::Result {
        // 1st 4 bytes: # of entries
        let len = ((self.names.len() / 8) as u32).to_le_bytes();
        // other bytes: nmaes
        out.write_all(&len)?;
        out.write_all(&self.names)?;
        Ok(())
    }
}

struct Texture1 {
    textures: Vec<Vec<u8>>,
}

impl Texture1 {
    #[inline]
    fn new() -> Self {
        Self { textures: vec![] }
    }

    #[inline]
    fn push_texture(&mut self, texture: Texture) {
        let Texture {
            mut name,
            width,
            height,
            patches,
            hscale,
            vscale,
        } = texture;
        let mut bytes = vec![];

        // push texture name
        name.make_ascii_uppercase();
        bytes.extend(name.into_bytes().into_iter().chain(iter::repeat(0)).take(8));

        // "flag" is unused
        bytes.extend(ArrayIter::new([0, 0]));

        // horizontal and vertical scaling
        let hscale = (hscale.unwrap_or(1.0) * 8.0) as u8;
        let vscale = (vscale.unwrap_or(1.0) * 8.0) as u8;
        bytes.extend(ArrayIter::new([hscale, vscale]));

        // width and height
        let (width, height) = (width.to_le_bytes(), height.to_le_bytes());
        bytes.extend(&width);
        bytes.extend(&height);

        // number of patches
        let num_patches = patches.len() as u16;
        bytes.extend(&num_patches.to_le_bytes());

        // patches
        patches
            .into_iter()
            .for_each(|(patch, x, y)| apply_patch(&mut bytes, &patch, x, y));

        self.textures.push(bytes);
    }

    #[inline]
    fn write<W: Write>(self, w: &mut W) -> crate::Result {
        // first, write the 4-byte num of textures
        w.write_all(&(self.textures.len() as u32).to_le_bytes())?;
        // then, write the offsets for each texture
        self.textures
            .iter()
            .scan((self.textures.len() * 4) + 4, |cur_offset, texture| {
                let my_offset = *cur_offset;
                *cur_offset += texture.len();
                Some(my_offset)
            })
            .try_for_each::<_, crate::Result>(|offset| {
                let offset = offset as u32;
                w.write_all(&offset.to_le_bytes())?;
                Ok(())
            })?;

        // finally, write each texture
        self.textures
            .into_iter()
            .try_for_each::<_, crate::Result>(|texture| {
                w.write_all(&texture)?;
                Ok(())
            })
    }
}

#[inline]
fn apply_patch(bytes: &mut Vec<u8>, patch: &Patch, x: i16, y: i16) {
    bytes.extend(&x.to_le_bytes());
    bytes.extend(&y.to_le_bytes());
    bytes.extend(&patch.index().to_le_bytes());
    bytes.extend(ArrayIter::new([0, 0, 0, 0]));
}
