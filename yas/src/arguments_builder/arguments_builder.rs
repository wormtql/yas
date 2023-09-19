use clap::Command;

pub trait ArgumentsBuilder {
    fn modify_arguments(cmd: Command) -> Command;
}
