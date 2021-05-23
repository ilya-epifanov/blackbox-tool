use std::{
    fs::File,
    io::{BufWriter, Read, Write},
    path::PathBuf,
    str::FromStr,
};

use clap::Clap;
use fc_blackbox::BlackboxRecord;
use itertools::Itertools;

use crate::opts::{Opts, SubCommand};

mod opts;

fn main() -> Result<(), anyhow::Error> {
    let opts = Opts::parse();

    let mut bytes = Vec::new();
    let input_path = PathBuf::from_str(&opts.input)?;
    File::open(&input_path)?.read_to_end(&mut bytes)?;

    let mut bbox = fc_blackbox::BlackboxReader::from_bytes(&bytes)?;

    match opts.subcmd {
        SubCommand::DumpCsv(dump_csv) => {
            let output_basename = dump_csv
                .output_basename
                .unwrap_or_else(|| input_path.with_extension("").to_string_lossy().into_owned());
            let mut out_main = BufWriter::new(File::create(format!("{}.csv", &output_basename))?);
            let mut out_gnss =
                BufWriter::new(File::create(format!("{}.gnss.csv", &output_basename))?);
            let mut out_slow =
                BufWriter::new(File::create(format!("{}.slow.csv", &output_basename))?);
            let mut out_event =
                BufWriter::new(File::create(format!("{}.event.csv", &output_basename))?);

            writeln!(
                out_main,
                "{}",
                bbox.header
                    .ip_fields_in_order
                    .iter()
                    .map(|f| &f.name)
                    .join(",")
            )?;
            writeln!(
                out_gnss,
                "loopIteration,time,{}",
                bbox.header
                    .g_fields_in_order
                    .iter()
                    .map(|f| &f.name)
                    .join(",")
            )?;
            writeln!(
                out_slow,
                "loopIteration,time,{}",
                bbox.header
                    .s_fields_in_order
                    .iter()
                    .map(|f| &f.name)
                    .join(",")
            )?;
            writeln!(out_event, "loopIteration,time,event")?;

            let mut data_main = vec![Vec::new(); bbox.header.ip_fields_in_order.len()];

            while let Some(record) = bbox.next() {
                match record {
                    BlackboxRecord::Main(values) => {
                        for (dst, src) in data_main.iter_mut().zip(values.iter()) {
                            dst.push(*src);
                        }
                        writeln!(
                            out_main,
                            "{}",
                            values.iter().map(|v| v.to_string()).join(",")
                        )?;
                    }
                    BlackboxRecord::GNSS(values) => {
                        let values = values.iter().map(|v| v.to_string()).join(",");
                        writeln!(
                            out_gnss,
                            "{},{},{}",
                            bbox.last_loop_iteration, bbox.last_time, values
                        )?;
                    }
                    BlackboxRecord::Slow(values) => {
                        let values = values.iter().map(|v| v.to_string()).join(",");
                        writeln!(
                            out_slow,
                            "{},{},{}",
                            bbox.last_loop_iteration, bbox.last_time, values
                        )?;
                    }
                    BlackboxRecord::Event(event) => {
                        writeln!(
                            out_event,
                            "{},{},{:?}",
                            bbox.last_loop_iteration, bbox.last_time, event
                        )?;
                    }
                    BlackboxRecord::Garbage(length) => {
                        println!("Got {} bytes of garbage", length);
                    }
                }
            }

            println!("{} fields, {} rows", data_main.len(), data_main[0].len());
        }
    }

    Ok(())
}
