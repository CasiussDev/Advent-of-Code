use crate::file_system::ChildList;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectoryDestination<'a> {
    Root,
    Parent,
    Child(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command<'a> {
    ChangeDirectory(DirectoryDestination<'a>),
    List(ChildList<'a>),
}
