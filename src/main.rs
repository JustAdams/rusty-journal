mod cli;
mod tasks;

use cli::{ Action::*, CommandLineArgs };
use structopt::StructOpt;
use tasks::Task;

fn main() {
    // Get the command-line arguments
    let CommandLineArgs {
        action,
        journal_file
    } = CommandLineArgs::from_args();

    // Unpack journal file
    let journal_file = journal_file.expect("Given journal file does not exist");

    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position)
    }
    .expect("Failed to perform {action}");
}
