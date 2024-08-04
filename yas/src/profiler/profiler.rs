use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use anyhow::Result;

pub struct Profiler {
    scope: Vec<String>,

    begin_time: Vec<SystemTime>,
    time_table: HashMap<String, (usize, Duration)>,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            scope: Vec::new(),
            begin_time: Vec::new(),
            time_table: HashMap::new(),
        }
    }

    fn get_key(&self) -> String {
        let mut ret = String::new();
        for item in self.scope.iter() {
            ret = ret + item;
        }
        ret
    }

    pub fn begin(&mut self, name: &str) {
        self.scope.push(String::from(name));
        self.begin_time.push(SystemTime::now());
    }

    pub fn end(&mut self, name: &str) -> Result<()> {
        if self.scope.len() == 0 {
            panic!("Profiler called end without begin");
        }
        let len = self.scope.len();
        let top = self.scope[len - 1].as_str();
        if top == name {
            let key = self.get_key();
            let entry = self.time_table.entry(key).or_insert((0, Duration::new(0, 0)));
            let elapsed = self.begin_time[len - 1].elapsed()?;

            entry.0 += 1;
            entry.1 += elapsed;

            self.scope.pop();
            self.begin_time.pop();
        } else {
            panic!("Profiler end {} and begin {} not match", name, top);
        }

        Ok(())
    }

    pub fn print(&self) {
        println!("Profile:");
        for (k, v) in self.time_table.iter() {
            let ms = v.1.as_millis() as f64 / (v.0 as f64);
            println!("{}: avg {}ms, execution count: {}", k, ms, v.0);
        }
    }
}
