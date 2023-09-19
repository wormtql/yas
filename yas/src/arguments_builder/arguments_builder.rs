use clap::Command;

pub trait ArgumentsModifier {
    fn modify_arguments(cmd: Command) -> Command;
}

