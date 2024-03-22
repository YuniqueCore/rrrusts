use clap::Parser;
use rrrusts::todo::{
    cli::{Cli, Verbs},
    dbguard::{DBConfig, DBOperator},
    tasks::TodoRecord,
};

fn main() {
    let db_header = TodoRecord::get_csv_header();
    let db_user = String::from("root");
    let dbconf = DBConfig::new("./", db_user, db_header);

    let mut db_op = DBOperator::connect(&dbconf).unwrap();

    let args = Cli::parse();

    match args.command {
        Verbs::About => {
            rrrusts::todo::cli::cli_about();
        }
        Verbs::Add {
            title,
            description,
            due_date,
            priority,
            done,
        } => {
            rrrusts::todo::cli::cli_add(
                dbconf,
                &mut db_op,
                title,
                description,
                priority,
                due_date,
                done,
            );
        }
        Verbs::Done { id, title } => {
            rrrusts::todo::cli::cli_done(id, &mut db_op, title);
        }
        Verbs::Remove { id, title } => {
            rrrusts::todo::cli::cli_remove(id, &mut db_op, title);
        }
        Verbs::List => {
            rrrusts::todo::cli::cli_list(db_op);
        }
    }
}
