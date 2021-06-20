use std::path::PathBuf;

pub fn cleanup_str(path: PathBuf) -> String {
    let mut _str = String::new();
    let str = path.into_os_string().into_string().unwrap();
    let (_strp1, strp2) = str.split_at(2);
    _str.push_str(strp2.clone());
    _str
}
