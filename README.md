# Util to split tracking time between tasks based on schedule and task complexity

### Build:

- cargo build --release

### Usage:

```
Usage: poker-tracker.exe [OPTIONS] --tasks <TASKS> --start-date <START_DATE> --end-date <END_DATE>

Options:
  -t, --tasks <TASKS> # Filename to csv with all tasks
  -s, --start-date <START_DATE> # Start date in format mm-dd-YYYY
  -e, --end-date <END_DATE> # End date in format mm-dd-YYYY
  --day-start <DAY_START>    [default: 10:00] # Workday start in format HH:MM
  --day-end <DAY_END>        [default: 18:00] # Workday end in format HH:MM
  -h, --help                     Print help
```

Examples of *.csv file with tasks is in resources/test directory


### TODO:

- [ ] Specify complexity in planning poker cards
- [ ] Tracking fill modes
- [X] continuous
- [ ] uniform
- [ ] Schedule rules(or manual calendar)
- [ ] Deadlines for specified tasks
