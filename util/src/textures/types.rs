// Apache 2.0 License

use std::{
    collections::{BTreeMap, HashMap},
    convert::TryInto,
    io::prelude::*,
    path::PathBuf,
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc,
    },
};

/// All information relating to the materials, patches, and textures.
#[derive(Debug)]
pub struct Matinfo {
    pub mats: Box<[Arc<Material>]>,
    pub flats: Vec<Flat>,
    pub patches: Vec<Arc<Patch>>,
    pub textures: Vec<Texture>,
}

impl Matinfo {
    #[inline]
    pub fn load<R: Read>(r: &mut R) -> crate::Result<Matinfo> {
        let m: MatinfoSD = serde_yaml::from_reader(r)?;
        Ok(m.into())
    }
}

#[derive(Debug)]
pub struct Material {
    pub name: Arc<str>,
    pub file: PathBuf,
    pub spdx: String,
    pub source: String,
}

#[derive(Debug)]
pub struct Flat {
    pub name: String,
    pub mat: Arc<Material>,
}

#[derive(Debug)]
pub struct Patch {
    pub name: Arc<str>,
    pub mat: Arc<Material>,
    pub transforms: Vec<Transform>,
    index: AtomicU16,
}

impl Patch {
    #[inline]
    pub fn assign_index(&self, i: usize) {
        self.index
            .store(i.try_into().expect("Too many patches!"), Ordering::SeqCst);
    }

    #[inline]
    pub fn index(&self) -> u16 {
        self.index.load(Ordering::SeqCst)
    }
}

#[derive(Debug)]
pub struct Texture {
    pub name: String,
    pub width: u16,
    pub height: u16,
    pub patches: Vec<(Arc<Patch>, i16, i16)>,
    pub hscale: Option<f32>,
    pub vscale: Option<f32>,
}

impl From<MatinfoSD> for Matinfo {
    #[inline]
    fn from(m: MatinfoSD) -> Matinfo {
        let MatinfoSD {
            mats,
            flats,
            patches,
            textures,
        } = m;

        let mats = mats
            .into_iter()
            .map(|(name, MaterialSD { file, spdx, source })| {
                let name: Arc<str> = name.into_boxed_str().into();
                (
                    name.clone(),
                    Arc::new(Material {
                        name,
                        file,
                        spdx,
                        source,
                    }),
                )
            })
            .collect::<HashMap<Arc<str>, _>>();

        let flats = flats
            .into_iter()
            .map(|(name, FlatSD { mat })| Flat {
                mat: mats
                    .get(mat.as_str())
                    .cloned()
                    .unwrap_or_else(|| panic!("flat: cannot find material with name {}", &mat)),
                name,
            })
            .collect::<Vec<_>>();

        let patches = patches
            .into_iter()
            .map(|(name, PatchSD { mat, transforms })| {
                let name: Arc<str> = name.into_boxed_str().into();

                (
                    name.clone(),
                    Arc::new(Patch {
                        name,
                        mat: mats.get(mat.as_str()).cloned().unwrap_or_else(|| {
                            panic!("patch: cannot find material with name {}", &mat)
                        }),
                        transforms: transforms.unwrap_or(vec![]),
                        index: AtomicU16::new(std::u16::MAX),
                    }),
                )
            })
            .collect::<HashMap<_, _>>();

        let textures = textures
            .into_iter()
            .map(
                |(
                    name,
                    TextureSD {
                        size: (width, height),
                        patches: texture_patches,
                        scale,
                    },
                )| {
                    let (hscale, vscale) = match scale {
                        None => (None, None),
                        Some((h, v)) => (Some(h), Some(v)),
                    };

                    Texture {
                        name,
                        width,
                        height,
                        hscale,
                        vscale,
                        patches: texture_patches
                            .into_iter()
                            .map(|(n, w, h)| {
                                (
                                    patches.get(&*n).cloned().unwrap_or_else(|| {
                                        panic!("texture: cannot find patch with name {}", &n)
                                    }),
                                    w,
                                    h,
                                )
                            })
                            .collect(),
                    }
                },
            )
            .collect::<Vec<_>>();

        Matinfo {
            textures,
            flats,
            mats: mats.into_iter().map(|(_, v)| v).collect(),
            patches: patches.into_iter().map(|(_, v)| v).collect(),
        }
    }
}

/// All information relating to the materials, patches, and textures, in a format easy to deserialize.
#[derive(Debug, serde::Deserialize)]
struct MatinfoSD {
    mats: BTreeMap<String, MaterialSD>,
    flats: BTreeMap<String, FlatSD>,
    patches: BTreeMap<String, PatchSD>,
    textures: BTreeMap<String, TextureSD>,
}

#[derive(Debug, serde::Deserialize)]
struct MaterialSD {
    file: PathBuf,
    spdx: String,
    source: String,
}

#[derive(Debug, serde::Deserialize)]
struct FlatSD {
    mat: String,
}

#[derive(Debug, serde::Deserialize)]
struct PatchSD {
    mat: String,
    transforms: Option<Vec<Transform>>,
}

#[derive(Debug, serde::Deserialize)]
struct TextureSD {
    size: (u16, u16),
    scale: Option<(f32, f32)>,
    patches: Vec<(String, i16, i16)>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Transform(pub String);
