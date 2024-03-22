use std::{fs, path::Path};

use clap::{arg, command, Parser, Subcommand};

use super::{
    dbguard::{DBConfig, DBOperator},
    tasks::TodoRecord,
};

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
    #[command(visible_aliases = & ["update"])]
    Done {
        #[arg(short, long, required = true)]
        id: Option<u32>,

        #[arg(short, long)]
        title: Option<String>,
    },

    #[command(about = "Remove a task.")]
    #[command(visible_aliases = & ["rm","delete","del"])]
    Remove {
        #[arg(short, long, required = true)]
        id: Option<u32>,

        #[arg(short, long)]
        title: Option<String>,
    },

    #[command(about = "List all tasks")]
    #[command(visible_aliases = & ["ls","la","ll"])]
    List,
}

pub fn cli_add(
    dbconf: DBConfig,
    db_op: &mut DBOperator,
    title: String,
    description: Option<String>,
    priority: u8,
    due_date: Option<String>,
    done: bool,
) {
    let newest_id = DBOperator::get_newest_id(&dbconf).unwrap_or(1);
    match db_op.add_record(TodoRecord::new(
        newest_id,
        title.clone(),
        description,
        priority,
        due_date.clone(),
    )) {
        Ok(_) => {
            println!("✔️\t Add record success!");
            if done {
                match db_op.done_record(TodoRecord::new(
                    newest_id,
                    title,
                    None,
                    TodoRecord::DEFAULT_PRIORITY,
                    due_date,
                )) {
                    Ok(_) => {
                        println!("📩\t Done record success!");
                    }
                    Err(e) => {
                        eprintln!("🦀\t Done record failed: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("🦀\t Add record failed: {}", e);
        }
    }
}

pub fn cli_done(id: Option<u32>, db_op: &mut DBOperator, title: Option<String>) {
    if let Some(id) = id {
        match db_op.done_record(TodoRecord::new(
            id,
            title.unwrap_or(String::new()),
            None,
            TodoRecord::DEFAULT_PRIORITY,
            None,
        )) {
            Ok(_) => {
                println!("📩\t Done record success!");
            }
            Err(e) => {
                eprintln!("🦀\t Done record failed: {}", e);
                fs::remove_file(Path::new(&db_op.db_path).with_extension("tmp")).unwrap();
            }
        }
    }
}

pub fn cli_remove(id: Option<u32>, db_op: &mut DBOperator, title: Option<String>) {
    if let Some(id) = id {
        match db_op.remove_record(TodoRecord::new(
            id,
            title.unwrap_or(String::new()),
            None,
            TodoRecord::DEFAULT_PRIORITY,
            None,
        )) {
            Ok(_) => {
                println!("❎\t Remove record success!");
            }
            Err(e) => {
                eprintln!("🦀\t Remove record failed: {}", e);
                fs::remove_file(Path::new(&db_op.db_path).with_extension("tmp")).unwrap();
            }
        }
    }
}

pub fn cli_list(mut db_op: DBOperator) {
    let _ = db_op.list_records(&mut std::io::stdout());
}

pub fn cli_about() {
    println!(
        r#"
    🌍 A unique Rust todo application was built to conquer the world. 🚀

    📝 About
    This Rust-based todo application is designed to help you manage your tasks efficiently and effortlessly. 
    With its sleek and intuitive interface, it provides a seamless experience for organizing your daily activities, projects, and goals.

    ⭐ Features
    ✅ Create, done, and see tasks with ease.
    ✅ Set due dates and priorities to stay organized.
    ✅ Mark tasks as completed to track your progress.
    ✅ Add descriptions to provide additional details.
    ✅ Enjoy a minimalistic and user-friendly design.

    💡 How to Use
    1. Add a task: Use the 'add|new' command followed by the task details.
    2. Done a task: Utilize the 'Done|update' command with the task ID and new information.
    3. Delete a task: Remove a task using the 'remove|delete|del|rm' command and the task ID.
    4. View all tasks: Access your task list with the 'list|ls|la|ll' command.
    5. Prioritize tasks: Assign priority levels to tasks to manage your workload effectively.

    🔨 Built with Rust
    This todo application was created using the powerful Rust programming language, known for its speed, safety, and concurrency. 
    Rust ensures that your tasks are managed efficiently and reliably, providing a robust foundation for your productivity needs.

    ⚡ Start conquering your tasks now! ⚡
    "#
    );
}
