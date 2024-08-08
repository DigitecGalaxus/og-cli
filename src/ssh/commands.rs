use std::path::PathBuf;

use clap::{Args, Subcommand};

/// Git helpers
#[derive(Args, Debug)]
pub struct SshCommand {
    #[command(subcommand)]
    // if this is in a seperate file it needs to be public
    pub command: SshSubCommands,
}

#[derive(Subcommand, Debug)]
pub enum SshSubCommands {
    /// Create .ssh/config entries for your tailscale hosts.
    TailscaleAlias {
        /// username for the ssh connection
        #[arg(short = 'u', long)]
        user: String,
        /// ssh identity file path
        #[arg(short = 'i', long)]
        identity_file: PathBuf,
        /// ssh identity file path
        #[arg(short = 'f', long)]
        tag_filter: Option<String>,
        #[arg(short = 'p', long)]
        hostname_prefix: Option<String>,
    },
}
