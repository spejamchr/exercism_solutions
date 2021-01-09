use std::collections::HashMap;

pub struct School {
    students: HashMap<String, u32>,
}

impl School {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.insert(student.to_string(), grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut g: Vec<u32> = self.students.values().copied().collect();
        g.sort_unstable();
        g.dedup();
        g
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        let mut vec = self
            .students
            .iter()
            .filter(|(_, v)| **v == grade)
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>();

        if vec.is_empty() {
            None
        } else {
            vec.sort_unstable();
            Some(vec)
        }
    }
}

impl Default for School {
    fn default() -> Self {
        Self::new()
    }
}
