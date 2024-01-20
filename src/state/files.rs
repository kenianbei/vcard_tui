use std::borrow::BorrowMut;
use std::env;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

use ratatui::widgets::ListState;

use crate::state::list::StatefulList;

#[derive(Clone)]
pub struct FilesState {
    pub current: Option<(PathBuf, String)>,
    pub exclude: FilesExclude,
    pub files: Vec<(PathBuf, String)>,
    pub list: ListState,
    pub path: PathBuf,
}

impl FilesState {
    pub fn new(exclude: FilesExclude) -> Self {
        let mut state = FilesState {
            current: None,
            exclude,
            files: Vec::new(),
            list: ListState::default(),
            path: PathBuf::default(),
        };

        if let Ok(path) = env::current_dir() {
            state.path = path;
            if let Ok(files) = Self::files(&state.path, &state.exclude) {
                state.files = files
            }
        }

        if !state.files.is_empty() {
            state.set_selected(Some(0));
        }

        state
    }

    pub fn files(path: &Path, exclude: &FilesExclude) -> anyhow::Result<Vec<(PathBuf, String)>> {
        let mut files = Vec::new();

        if let Some(parent) = path.parent() {
            files.push((parent.to_path_buf(), String::from("..")))
        }

        for item in read_dir(path)? {
            let entry = item?;

            match exclude {
                FilesExclude::None => {
                    if let Some(str) = entry.file_name().to_str() {
                        if str.starts_with('.') {
                            continue;
                        }
                        files.push((entry.path(), String::from(str)))
                    }
                }
                FilesExclude::Files => {
                    if entry.path().is_dir() {
                        if let Some(str) = entry.file_name().to_str() {
                            files.push((entry.path(), String::from(str)))
                        }
                    }
                }
            }
        }

        files.sort();

        Ok(files)
    }

    pub fn refresh(&mut self) -> anyhow::Result<()> {
        self.files = Self::files(&self.path, &self.exclude)?;

        if !self.files.is_empty() {
            self.set_selected_index(0);
        }

        Ok(())
    }
}

impl StatefulList<(PathBuf, String)> for FilesState {
    fn list(&mut self) -> &mut ListState {
        self.list.borrow_mut()
    }
    fn get(&self) -> Option<&(PathBuf, String)> {
        if let Some(index) = self.list.selected() {
            self.files.get(index)
        } else {
            None
        }
    }
    fn set(&mut self, current: &(PathBuf, String)) {
        self.current = Some(current.clone());
    }
    fn vec(&self) -> Vec<(PathBuf, String)> {
        self.files.clone()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum FilesExclude {
    None,
    Files,
}
