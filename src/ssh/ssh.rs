use super::commands::{SshCommand, SshSubCommands};
use eyre::Result;

pub struct SshPlugin;

impl SshPlugin {
    pub async fn run(cli: SshCommand) -> Result<()> {
        match cli.command {
            SshSubCommands::TailscaleAlias {
                user,
                identity_file,
                tag_filter,
                hostname_prefix,
            } => {
                which::which("tailscale").expect("tailscale must be installed");
                let hostnames = get_tailscale_hosts(tag_filter).await.unwrap();
                let filtered = hostnames.iter().filter(|h| {
                    if let Some(prefix) = &hostname_prefix {
                        return h.starts_with(prefix);
                    }
                    return true;
                });

                for hostname in filtered {
                    print_config_entry(
                        hostname.as_str(),
                        user.as_str(),
                        identity_file.to_str().unwrap(),
                    )
                }
                Ok(())
            }
        }
    }
}
fn print_config_entry(hostname: &str, user: &str, ssh_identity_path: &str) {
    println!("Host {}", hostname);
    println!("\tUser {}", user);
    println!("\tIdentityFile {ssh_identity_path}",);
}

async fn get_tailscale_hosts(tag_filter: Option<String>) -> Result<Vec<String>> {
    let socket_path = "/var/run/tailscale/tailscaled.sock";
    let client = tailscale_localapi::LocalApi::new_with_socket_path(socket_path);
    let client_status = client.status().await?;

    let buildservers: Vec<String> = client_status
        .peer
        .iter()
        .filter(|(_, ps)| {
            if let Some(filter) = &tag_filter {
                return ps.tags.contains(filter);
            }
            return true;
        })
        .map(|(_, ps)| ps.hostname.clone())
        .collect();

    println!("{:?}", buildservers);
    Ok(buildservers)
}
