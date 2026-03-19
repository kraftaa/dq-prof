use std::process::ExitCode;

use dq_prof::cli;
use dq_prof::report;

fn main() -> ExitCode {
    let (dataset_path, opts) = cli::parse_args();

    match dq_prof::app::run(&dataset_path, &opts) {
        Ok(report) => {
            match opts.output_format {
                dq_prof::types::OutputFormat::Text => {
                    if let Err(err) = report::terminal::print(&report, opts.color.clone()) {
                        eprintln!("failed to render report: {err}");
                        return ExitCode::from(1);
                    }
                }
                dq_prof::types::OutputFormat::Json => {
                    if let Err(err) = report::json::print(&report) {
                        eprintln!("failed to render json: {err}");
                        return ExitCode::from(1);
                    }
                }
            }

            let code = report::summary::exit_code(&report.summary, opts.fail_on);
            ExitCode::from(code as u8)
        }
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(1)
        }
    }
}
