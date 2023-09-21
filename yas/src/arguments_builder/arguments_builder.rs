use std::collections::HashSet;

use clap::{Command, Arg};

pub trait ArgumentsModifier {
    fn modify_arguments(builder: &mut ArgumentsBuilder);
}

pub struct ArgumentsBuilder {
    pub args: Vec<clap::Arg>,
    pub names: HashSet<String>
}

impl ArgumentsBuilder {
    pub fn new() -> ArgumentsBuilder {
        ArgumentsBuilder {
            args: Vec::new(),
            names: HashSet::new()
        }
    }

    pub fn arg(&mut self, a: Arg) -> &mut Self {
        let name = a.get_id().to_string();
        if self.names.contains(&name) {
            warn!("Arg name {} already exists", name);
            return self;
        }

        self.args.push(a);
        self
    }

    pub fn build(self, cmd: Command) -> Command {
        let mut cmd = cmd;
        for a in self.args {
            cmd = cmd.arg(a);
        }
        cmd
    }
}
