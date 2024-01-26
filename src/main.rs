mod calendar;
mod csv_export;
mod csv_import;
mod models;
pub mod poker_planner;
mod prop_div;

use chrono::{NaiveDate, NaiveTime, ParseError};
use clap::{Args, Parser, Subcommand};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "pt")]
#[command(author = "Kerrigan <xybermind@gmail.com>")]
#[command(about = "Simple time tracker calculator")]
#[command(name = "pt", author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand)]
enum CliCommands {
    #[command(name = "print", about = "Print result csv to terminal")]
    Print(CliArgs),
    #[command(name = "export", about = "Export result csv to output file")]
    Export(ExportArgs),
}

#[derive(Args)]
struct CliArgs {
    #[arg(short, long, help = "Input csv file with tasks and priorities")]
    tasks: PathBuf,

    #[arg(short, long, value_parser = parse_date, help = "Start date in format mm-dd-YYYY")]
    start_date: NaiveDate,

    #[arg(short, long, value_parser = parse_date, help = "Finish date in format mm-dd-YYYY")]
    end_date: NaiveDate,

    #[arg(long = "ds", default_value = "10:00", value_parser = parse_time, help = "Workday start time in format HH:MM")]
    day_start: NaiveTime,

    #[arg(long = "de", default_value = "18:00", value_parser = parse_time, help = "Workday end time in format HH:MM")]
    day_end: NaiveTime,
}

#[derive(Args)]
struct ExportArgs {
    #[clap(flatten)]
    args: CliArgs,

    #[arg(short, long, help = "Output csv filename")]
    output_filename: PathBuf,
}

fn parse_date(arg: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(arg, "%d-%m-%Y")
}

fn parse_time(arg: &str) -> Result<NaiveTime, ParseError> {
    NaiveTime::parse_from_str(arg, "%H:%M")
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        CliCommands::Print(args) => {
            run_pokerplanner(
                args.tasks,
                args.start_date,
                args.end_date,
                args.day_start,
                args.day_end,
                io::stdout(),
            );
        }
        CliCommands::Export(export_args) => {
            let file = File::create(export_args.output_filename)?;
            run_pokerplanner(
                export_args.args.tasks,
                export_args.args.start_date,
                export_args.args.end_date,
                export_args.args.day_start,
                export_args.args.day_end,
                file,
            );
        }
    }
    Ok(())
}

fn run_pokerplanner<W>(
    tasks_file: PathBuf,
    date_from: NaiveDate,
    date_to: NaiveDate,
    workday_start: NaiveTime,
    workday_end: NaiveTime,
    output: W,
) where
    W: Write + Sized,
{
    let calendar = calendar::WorkCalendar::new();
    let tasks = csv_import::csv_import(tasks_file).unwrap();
    let time_records = poker_planner::calculate_hours_by_tasks(
        &tasks,
        &date_from,
        &date_to,
        &workday_start,
        &workday_end,
        &calendar,
    );
    csv_export::csv_export(output, &time_records);
}

/*
 * TODO: settings: specify calendar
 * TODO: input: specify tasks with priority
 * TODO: input: specify workday
 * TODO: input: specify start/finish dates
 */
