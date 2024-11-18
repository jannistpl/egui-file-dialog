use std::path::{Path, PathBuf};

/// Wrapper above the `sysinfo::Disk` struct.
/// Used for helper functions and so that more flexibility is guaranteed in the future if
/// the names of the disks are generated dynamically.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Disk {
    mount_point: PathBuf,
    display_name: String,
    is_removable: bool,
}

impl Disk {
    pub fn new(
        name: Option<&str>,
        mount_point: &Path,
        is_removable: bool,
        canonicalize_paths: bool,
    ) -> Self {
        Self {
            mount_point: canonicalize(mount_point, canonicalize_paths),
            display_name: gen_display_name(
                name.unwrap_or_default(),
                mount_point.to_str().unwrap_or_default(),
            ),
            is_removable,
        }
    }

    /// Create a new Disk object based on the data of a `sysinfo::Disk`.
    pub fn from_sysinfo_disk(disk: &sysinfo::Disk, canonicalize_paths: bool) -> Self {
        Self::new(
            disk.name().to_str(),
            disk.mount_point(),
            disk.is_removable(),
            canonicalize_paths,
        )
    }

    /// Returns the mount point of the disk
    pub fn mount_point(&self) -> &Path {
        &self.mount_point
    }

    /// Returns the display name of the disk
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub const fn is_removable(&self) -> bool {
        self.is_removable
    }
}

/// Wrapper above the `sysinfo::Disks` struct
#[derive(Default, Debug)]
pub struct Disks {
    disks: Vec<Disk>,
}

impl Disks {
    /// Creates a new Disks object with a refreshed list of the system disks.
    pub fn new_with_refreshed_list(canonicalize_paths: bool) -> Self {
        Self {
            disks: load_disks(canonicalize_paths),
        }
    }

    /// Very simple wrapper method of the disks `.iter()` method.
    /// No trait is implemented since this is currently only used internal.
    pub fn iter(&self) -> std::slice::Iter<'_, Disk> {
        self.disks.iter()
    }
}

/// Canonicalizes the given path.
/// Returns the input path in case of an error.
fn canonicalize(path: &Path, canonicalize: bool) -> PathBuf {
    if canonicalize {
        dunce::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
    } else {
        path.to_path_buf()
    }
}

#[cfg(windows)]
fn gen_display_name(name: &str, mount_point: &str) -> String {
    let mount_point = mount_point.replace('\\', "");

    // Try using the mount point as the display name if the specified name
    // from sysinfo::Disk is empty or contains invalid characters
    if name.is_empty() {
        return mount_point;
    }

    format!("{name} ({mount_point})")
}

#[cfg(not(windows))]
fn gen_display_name(name: &str, mount_point: &str) -> String {
    // Try using the mount point as the display name if the specified name
    // from sysinfo::Disk is empty or contains invalid characters
    if name.is_empty() {
        return mount_point.to_string();
    }

    name.to_string()
}

#[cfg(windows)]
fn load_disks(canonicalize_paths: bool) -> Vec<Disk> {
    let mut disks: Vec<Disk> = sysinfo::Disks::new_with_refreshed_list()
        .iter()
        .map(|d| Disk::from_sysinfo_disk(d, canonicalize_paths))
        .collect();

    // `sysinfo::Disks` currently do not include mapped network drives on Windows.
    // We will load all other available drives using the Windows API.
    // However, the sysinfo disks have priority, we are just adding to the list.
    #[allow(unsafe_code)]
    let mut drives = unsafe { GetLogicalDrives() };
    let mut letter = b'A';

    while drives > 0 {
        if drives & 1 != 0 {
            let path = PathBuf::from(format!("{}:\\", letter as char));
            let mount_point = canonicalize(&path, canonicalize_paths);

            if !disks.iter().any(|d| d.mount_point == mount_point) {
                disks.push(Disk::new(None, &path, false, canonicalize_paths));
            }
        }

        drives >>= 1;
        letter += 1;
    }

    disks
}

#[cfg(windows)]
extern "C" {
    pub fn GetLogicalDrives() -> u32;
}

#[cfg(not(windows))]
fn load_disks(canonicalize_paths: bool) -> Vec<Disk> {
    sysinfo::Disks::new_with_refreshed_list()
        .iter()
        .map(|d| Disk::from_sysinfo_disk(d, canonicalize_paths))
        .collect()
}
