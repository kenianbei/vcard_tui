use std::ops::{Add, Sub};

use tui::widgets::ListState;

pub trait StatefulList<T> {
    fn list(&mut self) -> &mut ListState;
    fn set(&mut self, current: &T);
    fn vec(&self) -> Vec<T>;
    fn set_by_index(&mut self, index: usize) {
        if let Some(current) = self.vec().get(index) {
            self.list().select(Some(index));
            self.set(current);
        }
    }
    fn home(&mut self) {
        self.set_by_index(0)
    }
    fn pageup(&mut self) {
        match self.list().selected() {
            None => self.set_by_index(0),
            Some(u) => {
                if u > 9 {
                    self.set_by_index(u.sub(10))
                }
            }
        }
    }
    fn prev(&mut self) {
        match self.list().selected() {
            None => self.set_by_index(0),
            Some(u) => {
                if u > 0 {
                    self.set_by_index(u.sub(1))
                }
            }
        }
    }
    fn next(&mut self) {
        match self.list().selected() {
            None => self.set_by_index(0),
            Some(u) => {
                if u < (self.vec().len() - 1) {
                    self.set_by_index(u.add(1))
                }
            }
        }
    }
    fn pagedown(&mut self) {
        match self.list().selected() {
            None => self.set_by_index(0),
            Some(u) => {
                if u < (self.vec().len() - 10) {
                    self.set_by_index(u.add(10))
                }
            }
        }
    }
    fn end(&mut self) {
        match self.list().selected() {
            None => self.set_by_index(0),
            Some(_) => self.set_by_index(self.vec().len() - 1),
        }
    }
}
