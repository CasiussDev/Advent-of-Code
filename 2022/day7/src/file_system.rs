use std::slice;

pub type FileSizeInt = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Child<'a> {
    File(FileSizeInt),
    Directory { name: &'a str, id: Option<usize> },
}
pub type ChildList<'a> = Vec<Child<'a>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Directory<'a> {
    pub name: &'a str,
    pub parent: Option<usize>,
    pub size: FileSizeInt,
    pub children: ChildList<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Directories<'a> {
    dirs: Vec<Directory<'a>>,
    cwd: usize,
}

impl<'a> Directory<'a> {
    pub fn new(name: &'a str, parent: Option<usize>) -> Self {
        Self {
            name,
            parent,
            size: 0,
            children: Default::default(),
        }
    }
}

impl<'a> Directories<'a> {
    pub fn new() -> Self {
        let mut new_dirs = Self {
            dirs: Vec::with_capacity(1024),
            cwd: 0,
        };
        new_dirs.add_dir("/", None);
        new_dirs
    }

    pub fn cd_root(&mut self) {
        self.cwd = 0;
    }

    pub fn cd_parent(&mut self) {
        self.cwd = self.dirs[self.cwd].parent.unwrap();
    }

    pub fn cd_child(&mut self, name: &str) {
        self.cwd = self.dirs[self.cwd]
            .children
            .iter()
            .find_map(|x| {
                if let Child::Directory {
                    name: child_name,
                    id,
                } = x
                {
                    if &name == child_name {
                        return *id;
                    }
                }
                None
            })
            .unwrap();
    }

    pub fn add_children(&mut self, mut children: ChildList<'a>) {
        let size: usize = children
            .iter()
            .map(|child| match child {
                Child::File(size) => *size,
                Child::Directory { .. } => 0,
            })
            .sum();

        children.iter_mut().for_each(|child| {
            if let Child::Directory { name, id } = child {
                *id = Some(self.dirs.len());
                self.dirs.push(Directory::new(name, Some(self.cwd)));
            }
        });

        let current_dir = &mut self.dirs[self.cwd];
        current_dir.size = size;
        let mut parent_dir = current_dir.parent;
        while let Some(parent_id) = parent_dir {
            parent_dir = self.dirs[parent_id].parent;
            self.dirs[parent_id].size += size;
        }

        self.dirs[self.cwd].children = children;
    }

    pub fn add_dir(&mut self, name: &'a str, parent: Option<usize>) {
        let new_dir = Directory::new(name, parent);
        let new_dir_id = self.dirs.len();
        let parent = new_dir.parent;
        self.dirs.push(new_dir);

        if let Some(parent_id) = parent {
            let child_dir = self.dirs[parent_id]
                .children
                .iter_mut()
                .find(|x| {
                    if let Child::Directory {
                        name: child_name, ..
                    } = **x
                    {
                        return child_name == name;
                    }
                    false
                })
                .unwrap();
            if let Child::Directory { id, .. } = child_dir {
                *id = Some(new_dir_id);
            } else {
                unreachable!();
            }
        }
    }

    pub fn dirs(&self) -> slice::Iter<Directory> {
        self.dirs.iter()
    }
}
