use clap::{Parser, Subcommand};
use eyre::Result;

use og_cli::busybox::{self, BusyboxCommand};
use og_cli::config;
use og_cli::dotnet::{self, DotnetCommand};
use og_cli::fix::{self, FixCommand};
use og_cli::git;
use og_cli::git::GitCommand;
use og_cli::kubernetes::{self, KubernetesCommand};
use og_cli::mongo_db::{self, MongoDbCommand};
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
    Fix(FixCommand),
    Dotnet(DotnetCommand),
    Doctor,
    /// Run kubernetes config helpers
    Kubernetes(KubernetesCommand),
    #[clap(name = "mongodb")]
    MongoDb(MongoDbCommand),
    Git(GitCommand),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    config::init_config().await?;

    match cli.command {
        Commands::Busybox(busybox_command) => busybox::Busybox::run(busybox_command),
        Commands::MongoDb(mongodb_command) => mongo_db::MongoDb::run(mongodb_command),
        Commands::Sql(sql_command) => sql::Sql::run(sql_command).await?,
        Commands::Dotnet(command) => dotnet::Dotnet::run(command).expect("Reason"),
        Commands::Git(git_command) => git::Git::run(git_command),
        Commands::Fix(fix_command) => {
            fix::Fix::run(fix_command)?;
        }
        Commands::Doctor => og_cli::doctor::run(),
        Commands::Kubernetes(kubernetes_command) => {
            kubernetes::Kubernetes::run(kubernetes_command).await
        }
    }

    Ok(())
}
