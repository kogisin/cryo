use std::{sync::Arc, time::SystemTime};

use crate::{args, parse, summaries};
use cryo_freeze::{FreezeError, FreezeSummary};
use indicatif::ProgressBar;

/// run freeze for given Args
pub async fn run(args: args::Args) -> Result<Option<FreezeSummary>, FreezeError> {
    // parse inputs
    let t_start = SystemTime::now();
    let (query, source, sink) = match parse::parse_opts(&args).await {
        Ok(opts) => opts,
        Err(e) => return Err(e.into()),
    };
    let t_parse_done = SystemTime::now();

    let n_chunks_remaining = query.get_n_chunks_remaining(&sink)?;

    // print summary
    if !args.no_verbose {
        summaries::print_cryo_summary(&query, &source, &sink, n_chunks_remaining);
    }

    // check dry run
    if args.dry {
        if !args.no_verbose {
            println!("\n\n[dry run, exiting]");
        }
        return Ok(None)
    };

    // create progress bar
    let bar = Arc::new(ProgressBar::new(n_chunks_remaining));
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{wide_bar:.green} {human_pos} / {human_len}   ETA={eta_precise} ")
            .map_err(FreezeError::ProgressBarError)?,
    );

    // collect data
    if !args.no_verbose {
        summaries::print_header("\n\ncollecting data");
    }
    match cryo_freeze::freeze(&query, &source, &sink, bar).await {
        Ok(freeze_summary) => {
            // print summary
            let t_data_done = SystemTime::now();
            if !args.no_verbose {
                println!("...done\n\n");
                summaries::print_cryo_conclusion(
                    t_start,
                    t_parse_done,
                    t_data_done,
                    &query,
                    &freeze_summary,
                )
            }

            // return summary
            Ok(Some(freeze_summary))
        }

        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    }
}
