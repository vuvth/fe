mod task;

use clap::Parser;
use fe_common::panic::install_panic_hook;
use task::Commands;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct FelangCli {
    #[clap(subcommand)]
    command: Commands,
}

#[cfg(feature = "lsp-support")]
#[tokio::main]
async fn main() {
    install_panic_hook();

    let cli = FelangCli::parse();

    match cli.command {
        Commands::Build(arg) => {
            task::build(arg);
        }
        Commands::Check(arg) => {
            task::check(arg);
        }
        Commands::New(arg) => {
            task::create_new_project(arg);
        }
        Commands::Lsp(args) => {
            task::start_lsp_sever(args).await;
        }
    }
}

#[cfg(not(feature = "lsp-support"))]
fn main() {
    install_panic_hook();

    let cli = FelangCli::parse();

    match cli.command {
        Commands::Build(arg) => {
            task::build(arg);
        }
        Commands::Check(arg) => {
            task::check(arg);
        }
        Commands::New(arg) => {
            task::create_new_project(arg);
        }
    }
}
