use clap::Command;

pub trait ArgumentsBuilder {
    fn modify_arguments(&self, cmd: &mut Command);
}
