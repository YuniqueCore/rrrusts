use clap::Parser;
use rrrust::todo::{
    cli::{Cli, Verbs},
    dbguard::{DBConfig, DBOperator},
    tasks::TodoRecord,
};

fn main() {
    let db_header = TodoRecord::get_csv_header();
    let db_user = String::from("root");
    let dbconf = DBConfig::new("./", db_user, db_header);

    let args = Cli::parse();

    match args.command {
        Verbs::About => {
            println!("A yunique rust todo application was built to construct world.");
        }
        Verbs::Add {
            title,
            description,
            due_date,
            priority,
            done,
        } => {
            let mut db_op = DBOperator::connect(&dbconf).unwrap();
            db_op.add_record(TodoRecord::new(0, title, description, priority));
        }
        Verbs::Done { id, title } => todo!(),
        Verbs::List => {}
    }
}
