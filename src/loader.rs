use anyhow::Error;
use ignore::gitignore::GitignoreBuilder;
use log::info;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub enum FileType {
    Page,
    Resource,
}

#[derive(Debug)]
pub struct SiteFile {
    ty: FileType,
    name: String,
    file_path: PathBuf,
}

#[derive(Debug)]
pub struct SiteDir {
    name: String,
    file_path: PathBuf,
    files: Vec<SiteFile>,
    subdirs: Vec<SiteDir>,
}

impl SiteDir {
    fn new<S: Into<String>, P: AsRef<Path>>(name: S, file_path: P) -> SiteDir {
        SiteDir {
            name: name.into(),
            file_path: file_path.as_ref().to_path_buf(),
            files: Vec::new(),
            subdirs: Vec::new(),
        }
    }
}

pub struct FileLoader {
    root: PathBuf,
    ignore_builder: GitignoreBuilder,
}

impl FileLoader {
    pub fn new<P: AsRef<Path>>(path: P) -> FileLoader {
        let root = path.as_ref().to_path_buf();
        let mut ignore_builder = GitignoreBuilder::new(&root);
        ignore_builder.add(&root.join(".warpignore"));
        ignore_builder.add_line(None, ".warpignore").unwrap();
        FileLoader {
            root,
            ignore_builder,
        }
    }

    pub fn from_parent<P: AsRef<Path>>(path: P, f: &FileLoader) -> FileLoader {
        let root = path.as_ref().to_path_buf();
        let mut ignore_builder = f.ignore_builder.clone();
        ignore_builder.add(&root.join(".warpignore"));
        FileLoader {
            root,
            ignore_builder,
        }
    }

    pub fn to_site_dir(&self) -> Result<SiteDir, Error> {
        let ignorer = self.ignore_builder.build()?;

        info!(
            "Creating page tree from {:?}, warpignore has {} ignores, {} whitelists",
            self.root,
            ignorer.num_ignores(),
            ignorer.num_whitelists()
        );
        let dir_name = self.root.file_name().unwrap();
        let mut page_tree = SiteDir::new(dir_name.to_string_lossy(), &self.root);
        let files: Vec<SiteFile> = WalkDir::new(&self.root)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.file_type().is_dir())
            .filter(|e| {
                let m = ignorer.matched(e.path(), false);
                m.is_whitelist() || !m.is_ignore()
            })
            .map(|e| {
                let ty = if e.path().extension().map(|ex| ex == "md").unwrap_or(false) {
                    FileType::Page
                } else {
                    FileType::Resource
                };
                SiteFile {
                    ty,
                    name: e.path().file_stem().unwrap().to_string_lossy().into(),
                    file_path: e.into_path(),
                }
            })
            .collect();
        info!("{:?}: Added {} files", dir_name, files.len());
        page_tree.files = files;
        let subdirs: Result<Vec<SiteDir>, Error> = WalkDir::new(&self.root)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
            .filter(|e| e.path() != self.root)
            .filter(|e| {
                let m = ignorer.matched(e.path(), true);
                m.is_whitelist() || !m.is_ignore()
            })
            .map(|e| {
                let pt = FileLoader::from_parent(&e.path(), &self);
                pt.to_site_dir()
            })
            .collect();
        page_tree.subdirs = subdirs?;
        info!("{:?}: Added {} subdirs", dir_name, page_tree.subdirs.len());
        Ok(page_tree)
    }
}
