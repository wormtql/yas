use clap::Command;

pub trait ArgumentsModifier {
    fn modify_arguments(command: &mut Command);
}
