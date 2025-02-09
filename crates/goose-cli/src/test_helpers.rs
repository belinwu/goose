#[cfg(test)]
pub fn run_with_tmp_dir<F>(f: F)
where
    F: FnOnce(&std::path::Path),
{
    let dir = tempfile::tempdir().unwrap();
    std::env::set_var("HOME", dir.path());
    f(dir.path());
}
