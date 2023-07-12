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
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

///*  ____[[Folder]]____
///  | * name           |
///  | * Vec<files>     |
///  | * Vec<folders>   |
///  | * size           |
///  |                  |
///  |__________________|
/// 
///  */

mod filesystem {
    pub use std::{rc::Rc, cell::RefCell, fmt::{Display, Debug}};

    pub struct Filesystem {
        root: Rc<RefCell<Folder>>,
        pwd: Rc<RefCell<Folder>>,
        small_folders: Vec<Rc<RefCell<Folder>>>,
    }

    impl Filesystem {
        /// Creates new filesystem with root folder (pwd = root)
        pub fn new() -> Self {
            let root = Folder::root();
            Filesystem { 
                root: Rc::clone(&root), 
                pwd: Rc::clone(&root),
                small_folders: Vec::new(), }
        }

        /// Creates a new child folder under pwd
        fn mkdir<T: ToString>(&self, name: T) -> Rc<RefCell<Folder>> {
            let child = Folder::new(name.to_string(), &self.pwd);
            self.pwd.borrow_mut().add_child_folder(&child);
            child
        }

        /// Sets pwd to token.
        /// If CdToken::Child(s) is supplied, s === name of child folder. 
        /// Creates child folder if necessary.
        pub fn cd<T: ToString>(&mut self, token: CdToken<T>) {
            match token {
                CdToken::Root => {
                    self.pwd = Rc::clone(&self.root)
                },
                CdToken::Parent => {
                    let parent = Rc::clone(self
                        .pwd
                        .borrow()
                        .parent_folder
                        .as_ref()
                        .unwrap_or(&self.root));
                    self.pwd = parent;
                },
                CdToken::Child(name) => {
                    let child = self
                        .pwd
                        .borrow()
                        .get_child(&name);
                    
                    if let Some(c) = child {
                        self.pwd = Rc::clone(&c);
                    } else {
                        self.pwd = self.mkdir(name.to_string());
                    }
                },
            }
        }
        
        /// Adds supplied Dir or File under pwd, if it doesn't already exist.
        pub fn ls_once<T: ToString>(&mut self, token: LsToken<T>) {
            match token {
                LsToken::Dir(s) => {
                    let exists = self.pwd.borrow().exists_child(&s);
                    if !exists {
                        self.mkdir(s);
                    }
                }
                LsToken::File(s, n) => {
                    let exists = self.pwd.borrow().exists_file(&s);
                    if !exists {
                        self.pwd.borrow_mut().add_file(s, n);
                    }
                }
            }
        }

        fn update_pwd_size(&self) -> Result<u32, Rc<RefCell<Folder>>> {
            let pwd_size = self.pwd.borrow().calc_size()?;
            self.pwd.borrow_mut().size = Some(pwd_size);
            Ok(pwd_size)
        }

        pub fn update_sizes(&mut self) {
            match self.update_pwd_size() {
                Ok(n) => {
                    if n < 100000 {
                        self.small_folders.push(Rc::clone(&self.pwd));
                    }
                    if &self.pwd == &self.root {return ()}
                    self.cd(CdToken::Parent::<String>);
                    self.update_sizes();
                }
                Err(f) => {
                    self.pwd = Rc::clone(&f);
                    self.update_sizes();
                }
            }
        }

        pub fn small_folder_sizes(&self) -> u32 {
            self.small_folders
            .iter()
            .map(|f| f.borrow().size.unwrap())
            .sum()
        }

        fn best_fit_of_children(&self, parent: &Rc<RefCell<Folder>>, missing_space: u32, mut best_fit: u32) -> u32 {
            
            for child in &parent.borrow().child_folders {
                
                let mut size = 0;
                if child.borrow().child_folders.len() == 0 {
                    size = child.borrow().size.expect("Unexpectedly found a size == None value")
                } else {
                    size = self.best_fit_of_children(&child, missing_space, best_fit)
                }
                if size > missing_space && size < best_fit {
                    println!("Found a better fit: {} < {}", size, best_fit);
                    best_fit = size.clone();
                } 
                let parent_size = parent.borrow().size.unwrap();
                if parent_size > missing_space && parent_size < best_fit {
                    println!("Found a better fit: {} < {}", parent_size, best_fit);
                    best_fit = parent_size.clone();
                } 

            }
            best_fit
        }

        pub fn calc_delete(&mut self) -> u32 {
            let total_space: u32 = 70000000;
            let required_space: u32 = 30000000;
            let used_space: u32 = self.root.borrow().size.unwrap();
            let available_space: u32 = total_space - used_space;
            let missing_space: u32 = required_space - available_space;
            self.pwd = Rc::clone(&self.root);
            self.best_fit_of_children(&self.root, missing_space, used_space)
        }
    }

    pub enum CdToken<T> {
        Root,
        Parent,
        Child(T),
    }

    pub enum LsToken<T> {
        Dir(T),
        File(T, u32),
    }

    #[derive(PartialEq, Eq, Default)]
    struct Folder {
        name: String,
        qualified_name: String,
        parent_folder: Option<Rc<RefCell<Folder>>>,
        child_folders: Vec<Rc<RefCell<Folder>>>,
        files: Vec<File>,
        size: Option<u32>,
    }

    impl Folder {
        fn root() -> Rc<RefCell<Self>> {
            Rc::new( RefCell::new( 
                Folder {
                    name: String::from(" "),
                    qualified_name: String::from("/"),
                    parent_folder: None,
                    child_folders: Vec::new(),
                    files: Vec::new(),
                    size: None,
                }
            ))
        }

        fn new<T: ToString>(name: T, parent: &Rc<RefCell<Folder>>) -> Rc<RefCell<Self>> {
            let mut qualified_name = parent.borrow().qualified_name.clone();
            qualified_name.push_str(name.to_string().as_str());
            qualified_name.push('/');
            
            Rc::new( RefCell::new(
                Folder {
                    name: name.to_string(),
                    qualified_name,
                    parent_folder: Some(Rc::clone(parent)),
                    child_folders: Vec::new(),
                    files: Vec::new(),
                    size: None,
                }
            ))
        }

        fn add_parent_folder(&mut self, parent: &Rc<RefCell<Folder>>) {
            self.parent_folder = Some(Rc::clone(parent));
        }

        fn add_child_folder(&mut self, child: &Rc<RefCell<Folder>>) {
            self.child_folders.push(Rc::clone(child));
            self.size = None;
        }

        fn add_file<T: ToString>(&mut self, name: T, size: u32) {
            self.files.push(File { name: name.to_string(), size});
            self.size = None;
        }

        fn calc_size(&self) -> Result<u32, Rc<RefCell<Folder>>> {
            let mut folders_size: u32 = 0;

            for folder in &self.child_folders {
                if let Some(n) = folder.borrow().size {
                    folders_size += n;
                } else {
                    return Err(Rc::clone(folder));
                }
            }
            
            let files_size: u32 = self.files
                .iter()
                .map(|f| f.size)
                .sum();

            Ok(folders_size + files_size)
        }

        fn get_child<T: ToString>(&self, name: &T) -> Option<Rc<RefCell<Folder>>> {
            let name = name.to_string();
            for child in &self.child_folders {
                if child.borrow().name == name {return Some(Rc::clone(child))}
            }
            None
        }

        fn exists_child<T: ToString>(&self, name: &T) -> bool {
            let name = name.to_string();
            for child in &self.child_folders {
                if child.borrow().name == name {return true}
            }
            false
        }

        fn exists_file<T: ToString>(&self, name: &T) -> bool {
            let name = name.to_string();
            for file in &self.files {
                if file.name == name {return true}
            }
            false
        }
    }

    impl Display for Folder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(n) = self.size {
                write!(f, "{} {}", self.qualified_name.as_str(), n)
            } else {
                write!(f, "{} ?", self.qualified_name.as_str())
            }
        }
    }

    impl Debug for Folder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let fmt_parent_folder = match &self.parent_folder {
                Some(s) => (*s).borrow().qualified_name.clone(),
                None => "None".to_owned(),
            };

            let mut fmt_child_folders = String::from("[");
            for child in &self.child_folders {
                fmt_child_folders.push_str(child.borrow().name.as_str());
                fmt_child_folders.push_str(", ");
            }
            if let Some(s) = fmt_child_folders.strip_suffix(", ") {
                fmt_child_folders = s.to_owned();
            };
            fmt_child_folders.push(']');

            let output = format!("Folder(
\t- name: {}
\t- qualified_name: {}
\t- parent folder: {}
\t- child_folders: {}
\t- files: {:?}
\t- size: {:?}",
                self.name, self.qualified_name, fmt_parent_folder, fmt_child_folders, self.files, self.size);
            write!(f, "{}", output)
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct File {
        name: String,
        size: u32,
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
        fn test_file() {
            let file = File::new("Test".to_owned(), 20);
            assert_eq!(file, File {name: "Test".to_owned(), size: 20});
            assert_eq!(file.get_size(), 20);
        }

        #[test]
        fn test_new_folder() {
            let root = Folder::root();
            let folder = Folder::new("child1", &root);
            println!("Display: {}",folder.borrow());
            println!("Debug: {:?}",folder.borrow());
        }

        #[test]
        fn test_file_add_child_folder() {
            let root = Folder::root();
            let folder = Folder::new("child1", &root);
            root.borrow_mut().add_child_folder(&folder);
            assert_eq!(root.borrow().child_folders[0].borrow().name, String::from("child1"));
            println!("{:?}",root.borrow());
        }

        #[test]
        fn test_file_add_file() {
            let root = Folder::root();
            root.borrow_mut().add_file("file1", 3);
        }

        #[test]
        fn test_file_calc_size() {
            let root = Folder::root();
            assert_eq!(root.borrow().size, None);

            assert_eq!(root.borrow().calc_size(), Ok(0));
            
            root.borrow_mut().add_file("a", 10);
            root.borrow_mut().add_file("b", 10);
            root.borrow_mut().add_file("c", 10);
            assert_eq!(root.borrow().calc_size(), Ok(30));

            let known_file = Rc::new(RefCell::new(Folder {
                size: Some(100),
                ..Default::default()
            }));
            root.borrow_mut().add_child_folder(&known_file);
            assert_eq!(root.borrow().calc_size(), Ok(130));

            let unknown_folder = Folder::new("unknown_size", &root);
            root.borrow_mut().add_child_folder(&unknown_folder);
            assert!(root.borrow().calc_size().is_err());
        }

        #[test]
        fn test_file_get_child() {
            let root = Folder::root();
            let child = Folder::new("child1", &root);
            root.borrow_mut().add_child_folder(&child);
            assert_eq!(root.borrow().get_child(&"child1"), Some(child));
            assert_eq!(root.borrow().get_child(&"child2"), None);
        }

        #[test]
        fn test_file_exists_child() {
            let root = Folder::root();
            let child = Folder::new("child1", &root);
            root.borrow_mut().add_child_folder(&child);
            assert!(root.borrow().exists_child(&"child1"));
            assert!(!root.borrow().exists_child(&"child2"));
        }

        #[test]
        fn test_file_exists_file() {
            let root = Folder::root();
            root.borrow_mut().add_file("file1", 0);
            assert!(root.borrow().exists_file(&"file1"));
            assert!(!root.borrow().exists_file(&"file2"));
        }

        #[test]
        fn test_filesystem_mkdir() {
            let fs = Filesystem::new();
            let child = fs.mkdir("child1");
            assert!(fs.root.borrow().exists_child(&"child1"));
            assert_eq!(child.borrow().parent_folder, Some(Rc::clone(&fs.pwd)));
            assert_eq!(child.borrow().parent_folder, Some(Rc::clone(&fs.root)));
        }

        #[test]
        fn test_filesystem_cd() {
            let mut fs = Filesystem::new();
            fs.cd(CdToken::Child("child1"));

            assert!(fs.root.borrow().exists_child(&"child1"));
            assert_eq!(fs.pwd.borrow().name, String::from("child1"));
            assert_eq!(fs.pwd.borrow().parent_folder, Some(Rc::clone(&fs.root)));

            fs.cd(CdToken::Parent::<String>);
            assert_eq!(&fs.pwd, &fs.root);

            let child2 = fs.mkdir("child2");
            assert_eq!(fs.root.borrow().child_folders.len(), 2);
            fs.cd(CdToken::Child("child2"));
            assert_eq!(fs.root.borrow().child_folders.len(), 2);
            assert_eq!(fs.pwd, child2);
        }

        #[test]
        fn test_filesystem_ls_once() {
            let mut fs = Filesystem::new();
            fs.ls_once(LsToken::File("file1", 3));
            fs.ls_once(LsToken::Dir("child1"));
            assert_eq!(Rc::clone(&fs.root), Rc::clone(&fs.pwd));
            assert!(fs.root.borrow().exists_child(&"child1"));
            assert!(fs.root.borrow().exists_file(&"file1"));
        }

        #[test]
        fn test_filesystem_update_pwd_size() {
            let mut fs = Filesystem::new();
            fs.ls_once(LsToken::File("file1", 10));
            fs.ls_once(LsToken::File("file2", 10));
            fs.ls_once(LsToken::File("file3", 10));
            assert_eq!(fs.pwd.borrow().size, None);

            let _ = fs.update_pwd_size();
            assert_eq!(fs.pwd.borrow().size, Some(30));

            let mut fs_2 = Filesystem::new();
            fs.cd(CdToken::Child("child1"));
            fs.ls_once(LsToken::File("file1", 10));
            fs.ls_once(LsToken::File("file2", 10));
            fs.ls_once(LsToken::File("file3", 10));
            assert_eq!(fs.pwd.borrow().size, None);
            assert_eq!(fs.root.borrow().size, None);

            let _ = fs.update_pwd_size();
            assert_eq!(fs.pwd.borrow().size, Some(30));
            assert_eq!(fs.root.borrow().size, None);
        }

        #[test]
        fn test_filesystem_update_sizes() {
            let mut fs = Filesystem::new();
            fs.ls_once(LsToken::File("file1", 10));     // File under root
            fs.ls_once(LsToken::File("file2", 10));
            fs.ls_once(LsToken::File("file3", 10));

            fs.cd(CdToken::Child("child1"));
            fs.ls_once(LsToken::File("file3", 100));    // File under child1
            fs.ls_once(LsToken::File("file4", 100));
            fs.ls_once(LsToken::File("file5", 100));

            println!("root: {:?}", fs.root.borrow().files);
            println!("pwd: {:?}", fs.pwd.borrow().files);

            fs.update_sizes();                                  // pwd = root
            assert_eq!(fs.root.borrow().size, Some(330));

            assert_eq!(fs.small_folders.len(), 2);
            println!("{:?}", fs.small_folders);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    Cd(String),
    Ls,
    Dir(String),
    File(u32, String),
    Error
}

impl Token {
    fn parse_string(s: String) -> Self {
        let mut tokens = s.split_whitespace();
        match tokens.next() {
            None => Self::Error,
            Some("dir") => Self::Dir(tokens.next().unwrap().to_string()),
            Some("$") => {
                match tokens.next() {
                    Some("ls") => Self::Ls,
                    Some("cd") => Self::Cd(tokens.next().unwrap().to_string()),
                    _ => Self::Error,
                }
            }
            Some(n) => Self::File(n.parse::<u32>().unwrap(), tokens.next().unwrap().to_string())
        }
    }
}

// Add file: folder.files.push(File::new())
// Add folder: folder.sub_folders.push(Folder::new())
// cd: Folder::cd(folder: String)
// Hold cd: &Folder

fn main() {
    use filesystem::*;

    let lines = file_to_lines(PathBuf::from("inputs/7.inputs.txt"));
    let mut fs = Filesystem::new();

    for line in lines {
        let line = line.unwrap();
        let token = Token::parse_string(line);
        match token {
            Token::Cd(d) => {
                let s = d.as_str();
                match s {
                    "/" => fs.cd(CdToken::Root::<String>),
                    ".." => fs.cd(CdToken::Parent::<String>),
                    s => fs.cd(CdToken::Child(d)),
                };
            }
            Token::Ls => {
                continue
            }
            Token::Dir(d) => {
                fs.ls_once(LsToken::Dir(d));
            }
            Token::File(n, f) => {
                fs.ls_once(LsToken::File(f, n));
            }
            _ => {

            }
        }
    }

    fs.update_sizes();
    println!("{}",fs.small_folder_sizes());
    println!("{}",fs.calc_delete());
}

#[cfg(test)]
mod tests_7 {
    use super::*;
    #[test]
    fn test_token_parse_string() {
        let t = Token::parse_string("$ cd /".into());
        assert_eq!(t, Token::Cd("/".to_owned()));
        
        let t = Token::parse_string("$ ls".into());
        assert_eq!(t, Token::Ls);

        let t = Token::parse_string("dir bfqzjjct".into());
        assert_eq!(t, Token::Dir(String::from("bfqzjjct")));

        let t = Token::parse_string("293559 jztrccm.hvd".into());
        assert_eq!(t, Token::File(293559, String::from("jztrccm.hvd")));
    }
}