use std::{
    fs,
    fs::File,
    io::{self, Read, Write},
    path::{Path, PathBuf}
};

use crate::grid::Grid;

const MAGIC: [u8; 4] = *b"CAGR";
const VERSION: u8 = 1;

pub const GRIDS_DIR: &str = "grids";
pub const EXT: &str = "cagr";

pub fn ensure_dir() -> Result<(), GridIoError> {
    fs::create_dir_all(GRIDS_DIR)?;
    Ok(())
}

pub fn list_grids() -> Result<Vec<String>, GridIoError> {
    ensure_dir()?;

    let mut out = Vec::new();
    for entry in fs::read_dir(GRIDS_DIR)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some(EXT) {
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                out.push(name.to_string());
            }
        }
    }
    out.sort();
    Ok(out)
}

pub fn path_in_dir(file_name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(GRIDS_DIR).join(file_name)
}


#[derive(Debug)]
pub enum GridIoError {
    Io(io::Error),
    BadMagic,
    UnsupportedVersion(u8),
    BadSize,
}

impl From<io::Error> for GridIoError {
    fn from(e: io::Error) -> Self { GridIoError::Io(e) }
}

pub fn save(path: impl AsRef<Path>, grid: &Grid) -> Result<(), GridIoError> {
    ensure_dir()?;

    let mut f = File::create(path)?;
    f.write_all(&MAGIC)?;
    f.write_all(&[VERSION])?;

    let w = grid.width() as u32;
    let h = grid.height() as u32;
    f.write_all(&w.to_le_bytes())?;
    f.write_all(&h.to_le_bytes())?;

    f.write_all(grid.as_slice())?;
    Ok(())
}

pub fn load(path: impl AsRef<Path>) -> Result<Grid, GridIoError> {
    ensure_dir()?;

    let mut f = File::open(path)?;

    let mut magic = [0u8; 4];
    f.read_exact(&mut magic)?;
    if magic != MAGIC { return Err(GridIoError::BadMagic); }

    let mut ver = [0u8; 1];
    f.read_exact(&mut ver)?;
    if ver[0] != VERSION { return Err(GridIoError::UnsupportedVersion(ver[0])); }

    let mut wb = [0u8; 4];
    let mut hb = [0u8; 4];
    f.read_exact(&mut wb)?;
    f.read_exact(&mut hb)?;
    let w = u32::from_le_bytes(wb) as usize;
    let h = u32::from_le_bytes(hb) as usize;

    if w == 0 || h == 0 { return Err(GridIoError::BadSize); }

    let mut g = Grid::new(w, h);
    f.read_exact(g.as_mut_slice())?;
    Ok(g)
}
