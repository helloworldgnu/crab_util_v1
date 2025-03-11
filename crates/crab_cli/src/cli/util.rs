pub(crate) fn verify_file(path: &str) -> Result<String, &'static str> {
    let path = std::path::Path::new(path);
    if !path.exists() {
        return Err("File not found");
    }
    if !path.is_file() {
        return Err("Not a file");
    }

    Ok(path.to_string_lossy().to_string())
}

pub(crate) fn verify_path(path: &str) -> Result<std::path::PathBuf, &'static str> {
    let path = std::path::Path::new(path);
    if !path.exists() {
        return Err("Path not found");
    }
    if !path.is_dir() {
        return Err("Not a directory");
    }

    Ok(path.to_path_buf())
}
