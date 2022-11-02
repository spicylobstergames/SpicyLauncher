use crate::error::{Error, Result};
use crate::tracker::{Progress, ProgressEvent, ProgressTracker};
use std::fs::{self, File};
use std::io::{self, Read, Seek};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

pub fn extract<Tracker: ProgressTracker>(
    target: &Path,
    target_dir: &Path,
    strip_toplevel: bool,
    tracker: &mut Tracker,
) -> Result<()> {
    let source = File::open(target)?;
    if !target_dir.exists() {
        fs::create_dir(target_dir)?;
    }
    let mut archive = ZipArchive::new(source)?;
    let total = archive.len().try_into()?;
    tracker.set_total_progress(total, ProgressEvent::Extract);
    let do_strip_toplevel = strip_toplevel && has_toplevel(&mut archive)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let mut relative_path = file.mangled_name();
        if do_strip_toplevel {
            let base = relative_path
                .components()
                .take(1)
                .fold(PathBuf::new(), |mut p, c| {
                    p.push(c);
                    p
                });
            relative_path = relative_path
                .strip_prefix(&base)
                .map_err(|error| Error::StripToplevel {
                    toplevel: base,
                    path: relative_path.clone(),
                    error,
                })?
                .to_path_buf()
        }
        if relative_path.to_string_lossy().is_empty() {
            continue;
        }
        let mut output_path = target_dir.to_path_buf();
        output_path.push(relative_path);
        log::trace!("Extracting to: {:#?}", output_path);
        if file.name().ends_with('/') {
            fs::create_dir_all(&output_path)?;
        } else {
            if let Some(parent) = output_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            let mut outfile = File::create(&output_path)?;
            io::copy(&mut file, &mut outfile)?;
        }
        #[cfg(unix)]
        set_unix_mode(&file, &output_path)?;
        tracker.update_progress(Progress {
            event: ProgressEvent::Extract,
            received: i.try_into()?,
            total,
        });
    }
    Ok(())
}

fn has_toplevel<S: Read + Seek>(archive: &mut ZipArchive<S>) -> Result<bool> {
    let mut top_level_dir: Option<PathBuf> = None;
    if archive.len() < 2 {
        return Ok(false);
    }
    for i in 0..archive.len() {
        let file = archive.by_index(i)?.mangled_name();
        if let Some(top_level_dir) = &top_level_dir {
            if !file.starts_with(top_level_dir) {
                log::trace!("Found different toplevel directory");
                return Ok(false);
            }
        } else {
            let components: PathBuf = file.components().take(1).collect();
            log::trace!(
                "Checking if path component {:?} is the only toplevel directory",
                components
            );
            top_level_dir = Some(components);
        }
    }
    log::trace!("Found no other toplevel directory");
    Ok(true)
}

#[cfg(unix)]
fn set_unix_mode(file: &zip::read::ZipFile, output_path: &Path) -> io::Result<()> {
    if let Some(mode) = file.unix_mode() {
        fs::set_permissions(
            output_path,
            std::os::unix::fs::PermissionsExt::from_mode(mode),
        )?
    }
    Ok(())
}
