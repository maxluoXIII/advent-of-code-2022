use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    rc::Rc,
};

type InodeId = usize;

struct Inode {
    name: String,
    children: Vec<InodeId>,
    parent: InodeId,
    size: usize,
    id: InodeId,
}

struct FileSystem {
    inodes: HashMap<InodeId, Inode>,
    next_inode_id: InodeId,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            inodes: HashMap::new(),
            next_inode_id: 0,
        }
    }

    fn new_inode(&mut self, name: &str, parent: InodeId, size: usize) -> Inode {
        let ret = Inode {
            name: name.to_string(),
            children: Vec::new(),
            parent,
            size,
            id: self.next_inode_id,
        };
        self.next_inode_id += 1;

        ret
    }

    fn get_id(&self, path: &Path) -> Option<InodeId> {
        let mut path_comps = path.components();
        path_comps.next();
        let mut curr_inode = self.inodes.get(&0);
        if curr_inode.is_none() {
            // There is no root
            return None;
        }

        for path_comp in path_comps {
            curr_inode = curr_inode.unwrap().children.iter().find_map(|child| {
                let child_inode = self.inodes.get(&child)?;
                if child_inode.name == path_comp.as_os_str().to_str()? {
                    Some(child_inode)
                } else {
                    None
                }
            });

            if curr_inode.is_none() {
                return None;
            }
        }

        Some(curr_inode.unwrap().id)
    }

    fn add_inode(&mut self, path: &Path, size: usize) {
        if let Some(parent) = path.parent() {
            let parent_inode_id = self.get_id(parent).expect("Could not find parent id");
            let new_inode = self.new_inode(
                path.file_name().unwrap().to_str().unwrap(),
                parent_inode_id,
                size,
            );
            let new_inode_id = new_inode.id;
            self.inodes.insert(new_inode.id, new_inode);
            let parent_inode = self.inodes.get_mut(&parent_inode_id).unwrap();
            parent_inode.children.push(new_inode_id);
        } else {
            if path.to_str().unwrap() == "/" {
                let new_inode = self.new_inode("/", size, 0);
                self.inodes.insert(new_inode.id, new_inode);
            } else {
                let new_inode = self.new_inode(path.file_name().unwrap().to_str().unwrap(), size, 0);
                self.inodes.insert(new_inode.id, new_inode);
            }
        }
    }

    fn set_size(&mut self, path: &Path, size: usize) {
        let inode = self
            .inodes
            .get_mut(&self.get_id(path).expect("Could not find inode id for path"))
            .unwrap();
        inode.size = size;
    }

    fn contains(&mut self, path: &Path) -> bool {
        self.get_id(path).is_some()
    }

    fn calc_size(&mut self, id: InodeId) -> usize {
        let inode = self.inodes.get(&id).unwrap();
        if inode.children.is_empty() {
            return inode.size;
        }

        let mut sum = 0;
        for child_id in inode.children.clone() {
            sum += self.calc_size(child_id);
        }

        let mut inode = self.inodes.get_mut(&id).unwrap();
        inode.size = sum;

        return inode.size;
    }

    fn sum_of_small_dirs(&self, max_size: usize, id: InodeId) -> usize {
        let mut sum = 0;
        let curr_inode = self.inodes.get(&id).unwrap();
        for child_id in &curr_inode.children {
            sum += self.sum_of_small_dirs(max_size, *child_id);
        }

        if !curr_inode.children.is_empty() && curr_inode.size <= max_size {
            sum += curr_inode.size;
        }

        sum
    }

    fn find_freeing_dir_size(&self, needed_size: usize, id: InodeId) -> usize {
        let curr_inode = self.inodes.get(&id).unwrap();
        let mut res = if curr_inode.size > needed_size {
            curr_inode.size
        } else {
            usize::MAX
        };

        for child_id in &curr_inode.children {
            let child_res = self.find_freeing_dir_size(needed_size, *child_id);
            res = res.min(child_res);
        }

        res
    }

    fn get_total_size(&self) -> usize {
        self.inodes.get(&0).unwrap().size
    }
}

enum Command {
    ChangeDir(String),
    ListDir,
}

fn main() {
    let file = File::open("data/day7-full.txt").expect("Could not find data file");
    let reader = BufReader::new(file);

    let mut line_iter = reader.lines();
    let mut curr_path = PathBuf::new();
    let mut listing = false;
    let mut fs = FileSystem::new();
    loop {
        match line_iter.next() {
            Some(Ok(line)) => {
                if line.starts_with("$") {
                    // Process command
                    listing = false;
                    let mut words = line.split_ascii_whitespace();
                    words.next();

                    match words.next() {
                        Some("cd") => match words.next() {
                            Some("..") => {
                                curr_path.pop();
                            }
                            Some(dir_name) => {
                                curr_path.push(dir_name);
                                if !fs.contains(curr_path.as_path()) {
                                    fs.add_inode(curr_path.as_path(), 0);
                                }
                            }
                            None => {
                                panic!("No directory specified for cd");
                            }
                        },
                        Some("ls") => {
                            listing = true;
                        }
                        Some(unknown_command) => {
                            panic!("Unknown command {}", unknown_command);
                        }
                        None => {
                            // Ignore empty command line
                        }
                    }
                } else if listing {
                    let mut words = line.split_ascii_whitespace();
                    match words.next() {
                        Some("dir") => {
                            let dir_name = words.next().expect("No dir name");
                            curr_path.push(dir_name);
                            if !fs.contains(curr_path.as_path()) {
                                fs.add_inode(curr_path.as_path(), 0);
                            }
                            curr_path.pop();
                        }
                        Some(size) => {
                            let file_name = words.next().expect("No file name");
                            curr_path.push(file_name);
                            if !fs.contains(curr_path.as_path()) {
                                fs.add_inode(
                                    curr_path.as_path(),
                                    size.parse::<usize>().expect("Not size or dir"),
                                );
                            }
                            curr_path.pop();
                        }
                        None => {
                            // Ignore empty ls line
                        }
                    }
                } else {
                    panic!("Unknown terminal state")
                }
            }
            Some(err) => {
                panic!("{:?}", err)
            }
            None => break,
        }
    }

    fs.calc_size(0);
    let res = fs.sum_of_small_dirs(100000, 0);
    println!("{res}");
    let total_fs_size = 70000000;
    let update_size = 30000000;
    let total_free_space = total_fs_size - fs.get_total_size();
    let needed_space = update_size - total_free_space;

    let res = fs.find_freeing_dir_size(needed_space, 0);
    println!("{res}");
}
