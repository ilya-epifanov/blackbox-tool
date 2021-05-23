use clap::Clap;

/// Blackbox tool
#[derive(Clap)]
#[clap()]
pub struct Opts {
    /// Input blackbox file
    #[clap(short, long)]
    pub input: String,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    DumpCsv(DumpCsv),
}

/// Convert into csv file(-s)
#[derive(Clap)]
pub struct DumpCsv {
    /// Output file base name, will be suffixed with '.csv', '.gps.csv' etc.
    #[clap(short, long)]
    pub output_basename: Option<String>,
}
