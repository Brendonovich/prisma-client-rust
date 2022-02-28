use std::{
    fs::{self, Permissions},
    path::Path,
};

use super::{global_unpack_dir, platform};

pub fn unpack(data: &[u8], name: &str) {
    let file_name = format!("prisma-query-engine-{}", name);
    let temp_dir = global_unpack_dir();
    let dir = platform::check_for_extension(
        &platform::name(),
        &Path::new(&temp_dir)
            .join(file_name)
            .into_os_string()
            .into_string()
            .unwrap(),
    );

    std::fs::create_dir_all(temp_dir).unwrap();

    if let Ok(_) = fs::metadata(&dir) {
        // println!("query engine exists, not unpacking");
        return;
    }

    fs::write(&dir, data).unwrap();

    #[cfg(any(target_os = "unix", target_os = "macos"))]
    if let Ok(meta) = fs::metadata(&dir) {
        use std::os::unix::fs::PermissionsExt;
        {
            let mut perms = meta.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&dir, perms).unwrap();
        }
    }
}
