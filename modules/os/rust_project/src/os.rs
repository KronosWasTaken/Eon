pub mod env;
pub mod exec;
use std::fs::{self, OpenOptions};
use std::io::Write;
pub fn fread(path: String) -> Option<String> {
    fs::read_to_string(path).ok()
}
pub fn fwrite(path: String, content: String) {
    fs::write(path, content).expect("Unable to write to file");
}
pub fn fexists(path: String) -> bool {
    fs::exists(path).unwrap()
}
pub fn fremove(path: String) {
    fs::remove_file(path).unwrap()
}
pub fn frename(old_path: String, new_path: String) {
    fs::rename(old_path, new_path).unwrap();
}
pub fn fcopy(src: String, dest: String) {
    fs::copy(src, dest).unwrap();
}
pub fn fappend(path: String, content: String) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    writeln!(file, "{content}").unwrap();
}
