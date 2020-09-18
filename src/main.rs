use std::{fs};
use std::path::Path;

fn main() {
    let root = Path::new("/home/gramar/code/");
    let all = get_files_recurse(root);
    let num_to_remove = all.len();
    println!("Found {} .attach_pid files:", num_to_remove);
    for path in &all {
        println!("    {}", path);
    }
    if num_to_remove > 0 {
        println!("Starting remove");
        delete_files(&all);
        println!("Finished removal of {} files, exiting", num_to_remove);
    } else {
        println!("No files to remove, exiting");
    }

}

fn get_files_recurse(path: &Path) -> Vec<String> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();
        if metadata.is_dir() {
            let mut files_in_sub_dirs = get_files_recurse(entry.path().as_path());
            if files_in_sub_dirs.len() > 0 {
                paths.append(&mut files_in_sub_dirs);
            }
        } else if metadata.is_file() {
            let str = String::from(entry.path().as_path().to_str().unwrap());
            let fname = entry.file_name().into_string().unwrap();
            if fname.starts_with(".attach_pid") {
                paths.push(str);
            }
        }
    }
    return paths;
}

fn delete_files(paths: &Vec<String>) {
    for raw in paths {
        fs::remove_file(raw).unwrap();
        println!("Removed file: {}", raw)
    }
}


