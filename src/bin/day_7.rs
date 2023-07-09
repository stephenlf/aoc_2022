/*
/   1. Model our filesystem with nested structs
        DONE
/   2. Populate our filesystem from given commands/outputs
        DONE
/   3. Calculate the size of each directory
/   4. Solve the puzzle
/
/ */

use std::{path::PathBuf, io::{Lines, BufReader, BufRead}, fs::File};

fn file_to_lines(path: PathBuf) -> Lines<BufReader<File>> {
    let file = File::open("path").unwrap();
    BufReader::new(file).lines()
}

//-----------------Filesystem module--------

// Problem: Filesystem struct will be trolling through folders, occasionally grabbing references to super
//      folders and mutating them. Folders can't give mutable references to structs they only hold immutable
//      references to.
//      Typically, this would be solved with the .to_owned() method. But that usually clones, and we don't 
//      want to clone our folders.
// Solution 1: Give ownership of all folders to filesystem, so that it doesn't have to take references at all.
//      Use the Rc smart pointer. We can copy Rc and pass it around like a C-like pointer without 
//          worrying about multiple access until runtime.
//      Not recommended. "We use the Rc<T> type when we want to allocate some data on the heap for multiple 
//      parts of our program to read and we can’t determine at compile time which part will finish using the 
//      data last. If we knew which part would finish last, WE COULD JUST MAKE THAT PART THE DATA'S OWNER, and 
//      the normal ownership rules enforced at compile time would take effect."
//          Also, too-many-lists says that double-ended linkages like this are garbage. And we would need 
//      a messy, Rc<RefCell<Folder>> structure for interior mutability. 
//          We could also implement this with unsafe Rust, but the tooling recommended to test unsafe Rust 
//      (Miri) is too complicated for this project.

mod filesystem {
    use std::{rc::Rc, collections::HashMap};


    #[derive(Debug, Default)]
    pub struct Filesystem {
        pub folders: HashMap<String, Folder>,     // <qualified_name, Folder>
        pub pwd: String,                            // <qualified_name>
    }

    impl Filesystem {
        fn new() -> Self {
            let mut filesystem = Filesystem::default();
            filesystem.folders.insert(
                "root".to_owned(),
                Folder {
                    name: "root".to_owned(),
                    super_folder: None,
                    ..Default::default()
                });
            filesystem.pwd = "root".to_owned();
            filesystem
        }

        fn add_folder<T: ToString>(&mut self, name: T) -> String {
            let folder = Folder::new(name.to_string(), &self.pwd);
            let fqn = folder.get_qualified_name();
            self.folders.insert(fqn.clone(), folder);
            fqn 
        }

        fn cd_child<T: ToString>(&mut self, dir: T) {
            let mut q_requested_folder = self.pwd.clone();
            q_requested_folder.push_str(&dir.to_string());
            if let Some(f) = self.folders.get(&q_requested_folder) {
                self.pwd = f.get_qualified_name();
            } else {
                self.pwd = self.add_folder(dir.to_string());
            }
        }

        fn cd(&mut self, token: CdToken) {
            match token {
                CdToken::Root => {
                    self.pwd = "root".to_string();
                }
                CdToken::Parent => {
                    self.pwd = self.folders
                        .get(&self.pwd)
                        .unwrap()
                        .super_folder
                        .clone()
                        .unwrap();
                }
                CdToken::Child(d) => {
                    self.cd_child(d);
                }
            }
        }

        fn update_file_size(&mut self, folder: &String) -> Result<String, ()> {
            // Depth-first search for folder that can be calculated (size).
            // Attempt to read all folders. 
            // If a child folder doesn't have a calculated size yet, return child folder name
            // In corrlary function, call this function on child folder.
            let folder = self.folders.get_mut(folder).unwrap();

            let mut size: u32 = 0;

            for sub_folder in &folder.sub_folders {
                if let Some(s) = self.folders.get(sub_folder).unwrap().size {
                    size += s;
                } else {

                }
            }

            Err(())
        }
    }

    pub enum CdToken {
        Root,
        Parent,
        Child(String),
    }
    
    // qualified_name = /folder1/folder2...

    #[derive(Debug, Default)]
    pub struct Folder {
        pub name: String,
        pub super_folder: Option<String>,               // qualified_name
        pub sub_folders: Vec<String>,
        pub files: Vec<File>,
        pub size: Option<u32>,
    }

    impl Folder {
        // qualified name = "super_folder[q]/self.name"
        fn get_qualified_name(&self) -> String {
            if let Some(s) = &self.super_folder {
                let mut qualified_name = s.clone();
                qualified_name.push('/');
                qualified_name.push_str(&self.name);
                qualified_name
            } else {
                self.name.clone()
            }
        }

        fn new<T: ToString>(name: T, super_folder: &str) -> Self {
            Self {
                name: name.to_string(),
                super_folder: Some(super_folder.to_owned()),
                ..Default::default()
            }
        }

        fn add_file(&mut self, file: File) {
            self.files.push(file);
        }

        /// Add by name
        fn add_sub_folder(&mut self, folder: String) {
            self.sub_folders.push(folder);
        }
    }

    #[derive(Debug, Clone, Default, PartialEq, Eq)]
    pub struct File {
        pub name: String,
        pub size: u32,
    }

    impl File {
        fn new(name: String, size: u32) -> Self {
            Self {name, size}
        }

        fn get_size(&self) -> u32 {
            self.size
        }
    }

    #[cfg(test)]
    mod filesystem_tests {
        use super::*;

        #[test]
        fn test_new_file() {
            let file = File::new("Test".to_owned(), 20);
            assert_eq!(file, File {name: "Test".to_owned(), size: 20});
        }

        #[test]
        fn test_add_file_to_folder() {
            let file = File::new("Test".to_owned(), 20);
            let mut folder = Folder::default();
            folder.add_file(file.clone());
            assert_eq!(folder.files[0], file);
        }

        #[test]
        fn test_new_folder() {
            let folder = Folder::new("hey".to_owned(), "yo");
            assert_eq!(folder.name, "hey".to_owned());
        }

        #[test]
        fn test_get_q_name() {
            let folder = Folder::new("hey".to_owned(), "yo");
            assert_eq!(folder.get_qualified_name(), "yo/hey".to_owned())
        }

        #[test]
        fn test_new_filesystem() {
            let filesystem = Filesystem::new();
            assert_eq!(filesystem.pwd, "root".to_owned());
            filesystem.folders.get("root").unwrap();
        }

        #[test]
        fn test_add_folder_to_fs() {
            let mut filesystem = Filesystem::new();
            let fqd = filesystem.add_folder("name");
            let new_file = filesystem.folders.get(&fqd).unwrap();
            assert_eq!(new_file.super_folder, Some("root".to_owned()));
            assert_eq!(new_file.get_qualified_name(), "root/name".to_owned());
        }

        #[test]
        fn test_cd_child() {
            let mut filesystem = Filesystem::new();
            assert_eq!(filesystem.folders.contains_key("root/sub_folder"), false);
            assert_eq!(filesystem.pwd, "root".to_owned());

            filesystem.cd_child("sub_folder");
            assert_eq!(filesystem.pwd, "root/sub_folder".to_owned());
            assert_eq!(filesystem.folders.contains_key("root/sub_folder"), true);          
        }

        #[test]
        fn test_cd() {
            let mut filesystem = Filesystem::new();

            filesystem.cd_child("sub_folder");
            assert_eq!(filesystem.pwd, "root/sub_folder".to_owned());
            assert_eq!(filesystem.folders.contains_key("root/sub_folder"), true); 

            filesystem.cd(CdToken::Parent);
            assert_eq!(filesystem.pwd, "root".to_owned());

            filesystem.cd(CdToken::Child("sub_folder".to_string()));
            assert_eq!(filesystem.pwd, "root/sub_folder".to_owned());
            assert_eq!(filesystem.folders.contains_key("root/sub_folder"), true); 

            filesystem.cd(CdToken::Child("sub_folder".to_string()));
            assert_eq!(filesystem.pwd, "root/sub_folder/sub_folder".to_owned());
            assert_eq!(filesystem.folders.contains_key("root/sub_folder/sub_folder"), true); 

            filesystem.cd(CdToken::Root);
            assert_eq!(filesystem.pwd, "root".to_owned());
        }
    }

    // Add file: folder.files.push(File::new())
    // Add folder: folder.sub_folders.push(Folder::new())
    // cd: Folder::cd(folder: String)
    // Hold cd: &Folder
}

fn main() {

}

#[cfg(test)]
mod day_7_tests {

}