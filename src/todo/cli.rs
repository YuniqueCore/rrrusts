use clap::{arg, command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version, about)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Verbs,
}

#[derive(Subcommand, Debug)]
pub enum Verbs {
    #[command(about = "About this application.")]
    #[command(aliases = & ["info","i","a"],display_order = 100)]
    About,

    #[command(about = "Add a new task")]
    #[command(visible_aliases = & ["new"])]
    Add {
        #[arg(short, long, required = true)]
        #[arg(help = "The title of the task")]
        title: String,

        #[arg(short, long)]
        #[arg(help = "The description of the task")]
        description: Option<String>,

        #[arg(short = 'e', long,aliases = & ["end","times"],visible_aliases= & ["end","times"])]
        #[arg(help = "The due date of the task")]
        due_date: Option<String>,

        #[arg(short, long, default_value_t = 3, allow_negative_numbers = false)]
        #[arg(help = "The priority of the task")]
        priority: u8,

        #[arg(short = 'y', long, default_value_t = false)]
        #[arg(help = "The status of the task is done")]
        done: bool,
    },

    #[command(about = "Mark a task as done")]
    #[command(visible_aliases = & ["rm"])]
    Done {
        #[arg(short, long)]
        title: Option<String>,

        #[arg(short, long, required = true)]
        id: Option<u32>,
    },

    #[command(about = "List all tasks")]
    #[command(visible_aliases = & ["ls","la","ll"])]
    List,
}
