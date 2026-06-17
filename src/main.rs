use clap::Parser;

#[derive(Parser)]
#[command(name = "h41", version, about = "Discover and display TCP ports in use")]
struct Cli {
    /// Port to run the web server on
    #[arg(short, long, default_value_t = 8941)]
    port: u16,

    /// Output as JSON to stdout instead of starting the web server
    #[arg(long)]
    json: bool,

    /// Start the web server instead of the TUI
    #[arg(long)]
    web: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if !h41::ports::is_available() {
        if cfg!(unix) {
            eprintln!("🙉 lsof is not available on this system");
        } else {
            eprintln!("🙉 netstat is not available on this system");
        }
        std::process::exit(1);
    }

    if cli.json {
        let entries = h41::ports::all();
        println!("{}", serde_json::to_string_pretty(&entries).unwrap());
    } else if cli.web {
        h41::server::start(cli.port).await;
    } else {
        if let Err(e) = h41::tui::run() {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
