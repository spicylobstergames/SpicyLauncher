use crate::constant::ARCHIVE_PREFIX;
use crate::error::Result;
use crate::tracker::{Progress, ProgressEvent, ProgressTracker};
use flate2::read::GzDecoder;
use std::fs::File;
use std::path::Path;
use tar::Archive as TarArchive;

pub fn extract<Tracker: ProgressTracker>(
    target: &Path,
    destination: &Path,
    file_name: &str,
    tracker: &mut Tracker,
) -> Result<()> {
    let tar_gz = File::open(target)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = TarArchive::new(tar);
    let total = archive.entries()?.count().try_into()?;
    tracker.set_total_progress(total, ProgressEvent::Extract);
    tracker.update_progress(Progress {
        event: ProgressEvent::Extract,
        received: 0,
        total,
    });

    let tar_gz = File::open(target)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = TarArchive::new(tar);
    for (i, entry) in archive.entries()?.into_iter().enumerate() {
        let mut entry = entry?;
        let entry_path = match entry
            .path()?
            .strip_prefix(format!("{}{}", ARCHIVE_PREFIX, file_name))
            .ok()
        {
            Some(v) => v.to_path_buf(),
            None => entry.path()?.to_path_buf(),
        };
        tracker.update_progress(Progress {
            event: ProgressEvent::Extract,
            received: i.try_into()?,
            total,
        });
        let extract_path = destination.join(file_name).join(entry_path);
        log::trace!("Extracting to: {:?}", extract_path);
        entry.unpack(&extract_path)?;
    }

    Ok(())
}
