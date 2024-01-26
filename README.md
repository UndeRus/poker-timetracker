# Util to split tracking time between tasks based on schedule and task complexity

### Build:

- cargo build --release

### Usage:

```
Print result csv to terminal

Usage: poker-tracker.exe print [OPTIONS] --tasks <TASKS> --start-date <START_DATE> --end-date <END_DATE>

Options:
  -t, --tasks <TASKS>            Input csv file with tasks and priorities
  -s, --start-date <START_DATE>  Start date in format mm-dd-YYYY
  -e, --end-date <END_DATE>      Finish date in format mm-dd-YYYY
      --ds <DAY_START>           Workday start time in format HH:MM [default: 10:00]
      --de <DAY_END>             Workday end time in format HH:MM [default: 18:00]
  -h, --help                     Print help

Export result csv to output file

Usage: poker-tracker.exe export [OPTIONS] --tasks <TASKS> --start-date <START_DATE> --end-date <END_DATE> --output-filename <OUTPUT_FILENAME>

Options:
  -t, --tasks <TASKS>                      Input csv file with tasks and priorities
  -s, --start-date <START_DATE>            Start date in format mm-dd-YYYY
  -e, --end-date <END_DATE>                Finish date in format mm-dd-YYYY
      --ds <DAY_START>                     Workday start time in format HH:MM [default: 10:00]
      --de <DAY_END>                       Workday end time in format HH:MM [default: 18:00]
  -o, --output-filename <OUTPUT_FILENAME>  Output csv filename
  -h, --help                               Print help

```

Examples of *.csv file with tasks is in resources/test directory


### TODO:

- [X] Specify output to file/stdout
- [ ] Specify complexity in planning poker cards
- [ ] Tracking fill modes
- [X] continuous
- [ ] uniform
- [ ] Schedule rules(or manual calendar)
- [ ] Deadlines for specified tasks
- [ ] Specify planning quants(hour/half-hour/etc)
