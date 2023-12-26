## dotup
A command line tool for backing up dotfiles. Inspired by [yadm](https://github.com/TheLocehiliosan/yadm).

### Initialize a new repository
```
dotup init
```

### Clone an existing repository (MAKE SURE TO INCLUDE THE `.`)
```
dotup clone <url> .
```

### Add a file to the list of tracked files
```
dotup track <important file>
```

### Remove a file from the list of tracked files
```
dotup untrack <important file>
```

### Run the git ui application that is specified in the config.toml
If a TUI or GUI application such as GitUi, LazyGit, GitKraken is installed and specified in metadata it will be launched with the directory set to the dotfiles rpeo.
```
dotup ui
```

### Add files/changes to git with dotup followed by a git command
```
dotup status 
dotup add <important file>
dotup commit
```

TODO:
- Get tracking working on local system.
    - [x] Use repo path in notify watcher
    - [x] Get untrack working.
    - [x] Get syncing working on local system.
    - [x] Get syncing to pick up newly added files
- [x] Get notify/sync command working.
- [x] create config
- [x] Run gitui/lazygit/other from updot
- [x] Update to use git commands
- add profiles
- rename data.json
- add consts for repo path and data.json 
- Add logging.
- Get working for multiple systems (generalize home/local/config)
- change name to updot
- add additional info to updot status
- Create tests:
    - test track
    - test untrack
    - test git commands including merge
    - test gitui/lazygit launches ect
    - Fix warnings.
- Create just file with linting/tests.

some cases to consider:
- a file is merged into a system that doesn't need it, the file is untracked. Should the file still exist?
- should untack only work if the file exists locally or should we copy the file locally before using it?
