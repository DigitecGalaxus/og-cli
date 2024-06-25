
use clap::{Parser, Subcommand};
use og_cli::busybox::{self, BusyboxCommand};
use og_cli::fix::{self, FixCommand};
use og_cli::kubernetes::{self, KubernetesCommand};
use og_cli::mongo_db::{self, MongoDbCommand};
use og_cli::plugin::Plugin;
use og_cli::sql;
use og_cli::sql::SqlCommand;

#[derive(Parser, Debug)]
#[command(version, about)]
#[clap(name = "dg cli")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(name = "busybox")]
    Busybox(BusyboxCommand),
    /// Run an sql server inside a docker container
    Sql(SqlCommand),
    Kafka,
    Flink,
    Fix(FixCommand),
    Doctor,
    /// Run kubernetes config helpers
    Kubernetes(KubernetesCommand),
    MongoDb(MongoDbCommand),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Busybox(busybox_command) => busybox::Busybox::run(busybox_command),
        Commands::MongoDb(mongodb_command) => mongo_db::MongoDb::run(mongodb_command),
        Commands::Sql(sql_command) => sql::Sql::run(sql_command).await,
        Commands::Kafka => println!("Kafka has not been implemented yet"),
        Commands::Flink => println!("Flink has not been implemented yet"),
        Commands::Fix(fix_command) => {
            fix::Fix::run(fix_command);
        },
        Commands::Doctor => {
            let plugins: Vec<Box<dyn Plugin>> =
                vec![Box::new(fix::Fix), Box::new(busybox::Busybox), Box::new(mongo_db::MongoDb)];
            for plugin in &plugins {
                plugin.doctor();
            }
        },
        Commands::Kubernetes(kubernetes_command) => {
                kubernetes::Kubernetes::run(kubernetes_command).await
        }
    }
}
