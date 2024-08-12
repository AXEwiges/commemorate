# Commemorate

commemorate is a tool for calculating memorable moments.

commemorate will encrypt the events you want to remember and save them in ~/.commemorate. You can use this tool through multiple commands.

## Usage

1. `commemorate add` Add new event.
```bash
commemorate add --name {eventname} --description {"description"} --time(option) --area(option) --path(option)
```

2. `commemorate list` List all event in `~/.commemorate`.
```bash
commemorate list
```

3. `commemorate tell` Tell the event for you.
```bash
commemorate tell {eventname}
```