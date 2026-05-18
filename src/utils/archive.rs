use std::{
    fs::{self},
    io::{self},
    path::{Path, PathBuf},
};

/// Creates a zip archive at `zip_path` containing all files recursively under `dir_path`.
pub fn zip_create_from_directory(zip_path: &PathBuf, dir_path: &PathBuf) -> io::Result<()> {
    let file = fs::File::create(zip_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    add_dir_to_zip(&mut zip, dir_path, dir_path, options)?;
    zip.finish().map_err(io::Error::other)?;
    Ok(())
}

fn add_dir_to_zip(
    zip: &mut zip::ZipWriter<fs::File>,
    base_dir: &PathBuf,
    dir: &PathBuf,
    options: zip::write::SimpleFileOptions,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)?.flatten() {
        let path = entry.path();
        let relative = path.strip_prefix(base_dir).unwrap();
        let name = relative.to_string_lossy().replace('\\', "/");

        if path.is_dir() {
            zip.add_directory(&name, options)
                .map_err(io::Error::other)?;

            add_dir_to_zip(zip, base_dir, &path, options)?;
        } else {
            zip.start_file(&name, options).map_err(io::Error::other)?;

            let mut f = fs::File::open(&path)?;

            io::copy(&mut f, zip)?;
        }
    }
    Ok(())
}

/// Extracts the zip archive at `zip_path` into `output_dir`.
pub fn zip_extract(zip_path: &PathBuf, output_dir: &Path) -> io::Result<()> {
    let file = fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file).map_err(io::Error::other)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(io::Error::other)?;
        let out_path = output_dir.join(entry.name());

        if entry.is_dir() {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut out_file = fs::File::create(&out_path)?;
            io::copy(&mut entry, &mut out_file)?;
        }
    }
    Ok(())
}
