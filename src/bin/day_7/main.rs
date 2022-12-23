use lazy_static::lazy_static;
use regex::Regex;
use std::{cell::RefCell, fs, rc::Rc};

lazy_static! {
    static ref REGEX_CMD: Regex =
        Regex::new(r"^\$ (\S+)[ ]?(\S*)$").expect("Failed to create Regex CMD");
    static ref REGEX_LS_DIR: Regex =
        Regex::new(r"^dir (\S+)$").expect("Failed to create Regex LS_DIR");
    static ref REGEX_LS_FILE: Regex =
        Regex::new(r"^(\d+) (\S+)$").expect("Failed to create Regex LS_FILE");
}

const DISK_SIZE: u32 = 70000000;
const UPDATE_SIZE: u32 = 30000000;

fn main() {
    println!("\n=== Day 7  ====");

    let inputs =
        fs::read_to_string("src/bin/day_7/input.txt").expect("Unable to find 'input.txt' !");
    // "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k\n\n".to_string();

    let mut fs = FileSystem::new();
    let mut is_ls_run = false;

    for line in inputs.lines() {
        if line.is_empty() {
            continue;
        }

        if line.contains("$") {
            let (cmd, arg) = parse_command(line);
            is_ls_run = false;

            match cmd.as_str() {
                "cd" => {
                    match arg[0].as_str() {
                        "/" => fs.move_to_root(),
                        ".." => fs.move_to_parent(),
                        _ => {
                            if !fs.move_to(arg[0].to_string()) {
                                println!("'{}' not found...", arg[0]);
                            }
                        }
                    };
                }
                "ls" => {
                    is_ls_run = true;
                }
                _ => (),
            }
        } else if is_ls_run {
            if line.starts_with("dir") {
                fs.add_folder(parse_ls_dir(line));
            } else {
                let (name, size) = parse_ls_file(line);
                fs.add_file(name, size);
            }
        }
    }

    let folders_size = fs.root.borrow().get_folders_size();

    /* ===== FIRST PART ===== */
    let mut result_one: u32 = 0;
    for f in folders_size.iter().filter(|f| f.1 <= 100000) {
        result_one += f.1;
    }

    println!("\nPart one answer: {}", result_one);

    /* ===== SECOND PART ===== */
    let needed_space = UPDATE_SIZE - (DISK_SIZE - fs.root.borrow().get_size());
    let smaller_to_del = folders_size
        .iter()
        .filter(|f| f.1 >= needed_space)
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();

    println!(
        "\nPart two answer: {} ({})",
        smaller_to_del.1, smaller_to_del.0
    );
}

fn parse_command(line: &str) -> (String, Vec<String>) {
    let capt = REGEX_CMD.captures(line).expect("Fail to capture command");
    return (
        capt[1].to_string(),
        capt[2]
            .to_string()
            .split(" ")
            .map(|s| s.to_string())
            .collect(),
    );
}

fn parse_ls_dir(line: &str) -> String {
    let capt = REGEX_LS_DIR.captures(line).expect("Fail to capture ls dir");
    return capt[1].to_string();
}

fn parse_ls_file(line: &str) -> (String, u32) {
    let capt = REGEX_LS_FILE
        .captures(line)
        .expect("Fail to capture ls dir");
    return (capt[2].to_string(), capt[1].parse().unwrap());
}

struct File {
    _name: String,
    size: u32,
}

type RcRefCell<T> = Rc<RefCell<T>>;
struct Folder {
    files: Vec<File>,
    folders: Vec<RcRefCell<Folder>>,
    name: String,
    parent: Option<RcRefCell<Folder>>,
}

struct FileSystem {
    root: RcRefCell<Folder>,
    current: RcRefCell<Folder>,
}

impl FileSystem {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(Folder::new("/".to_string())));
        FileSystem {
            root: Rc::clone(&root),
            current: Rc::clone(&root),
        }
    }

    fn move_to_root(&mut self) {
        self.current = Rc::clone(&self.root);
    }

    fn move_to(&mut self, folder_name: String) -> bool {
        match self.find_foler(folder_name) {
            Some(f) => {
                self.current = f;
            }
            None => {
                return false;
            }
        }

        return true;
    }

    fn move_to_parent(&mut self) {
        if self.current.borrow().parent.is_some() {
            let tmp = Rc::clone(self.current.borrow().parent.as_ref().unwrap());
            self.current = tmp;
        }
    }

    fn find_foler(&self, folder_name: String) -> Option<RcRefCell<Folder>> {
        let current = (*self.current).borrow_mut();

        let val = current
            .folders
            .iter()
            .find(|folder| folder.borrow().name == folder_name);
        match val {
            Some(f) => Some(Rc::clone(f)),
            None => None,
        }
    }

    fn add_folder(&mut self, name: String) {
        self.current
            .borrow_mut()
            .add_folder(name, Rc::clone(&self.current));
    }

    fn add_file(&mut self, name: String, size: u32) {
        self.current.borrow_mut().add_file(name, size);
    }
}

impl Folder {
    fn new(name: String) -> Self {
        Folder {
            files: Vec::new(),
            folders: Vec::new(),
            name,
            parent: None,
        }
    }

    fn get_size(&self) -> u32 {
        let mut size: u32 = 0;

        for f in &self.files {
            size += f.size;
        }

        for f in &self.folders {
            size += f.borrow().get_size();
        }

        return size;
    }

    fn add_folder(&mut self, name: String, parent: RcRefCell<Folder>) {
        let mut folder = Folder::new(name);
        folder.parent = Some(parent);
        self.folders.push(Rc::new(RefCell::new(folder)));
    }

    fn add_file(&mut self, name: String, size: u32) {
        self.files.push(File::new(name, size));
    }

    fn get_folders_size(&self) -> Vec<(String, u32)> {
        let mut result: Vec<(String, u32)> = Vec::new();

        for f in self.folders.iter() {
            let mut sub = f.borrow().get_folders_size();

            result.push((f.borrow().name.clone(), f.borrow().get_size()));
            result.append(&mut sub);
        }

        return result;
    }
}

impl File {
    fn new(name: String, size: u32) -> Self {
        File { _name: name, size }
    }
}

#[cfg(test)]
mod test {
    use crate::FileSystem;

    #[test]
    fn basic_size() {
        let mut fs = FileSystem::new();

        let size = fs.root.borrow().get_size();

        assert_eq!(size, 0);
        assert_eq!(fs.current.borrow().name, "/".to_string());

        fs.move_to_parent();
        assert_eq!(fs.current.borrow().name, "/".to_string());
    }

    #[test]
    fn add_folder() {
        let mut fs = FileSystem::new();

        fs.add_folder("test".to_string());
        assert_eq!(fs.current.borrow().name, "/".to_string());
        fs.move_to("test".to_string());
        assert_eq!(fs.current.borrow().name, "test".to_string());
    }

    #[test]
    fn add_file() {
        let mut fs = FileSystem::new();

        assert_eq!(fs.root.borrow().get_size(), 0);

        fs.add_file("plop".to_string(), 100);
        fs.add_folder("test".to_string());
        fs.move_to("test".to_string());
        fs.add_file("plop".to_string(), 200);

        assert_eq!(fs.current.borrow().get_size(), 200);
        assert_eq!(fs.root.borrow().get_size(), 300);
    }
}
