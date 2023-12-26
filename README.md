## dotup
A command line tool for backing up dotfiles. Inspired by [yadm](https://github.com/TheLocehiliosan/yadm).

### Setup
Initialize a repository
```
dotup init
```
**or**
Clone an existing repository (MAKE SURE TO INCLUDE THE `.`)
```
dotup clone <url> .
```


### Tracking files
Add a file to the list of tracked files
```
dotup track <important file>
```
Remove a file from the list of tracked files
```
dotup untrack <important file>
```

### Use command line git or a git GUI/TUI application
Run the git ui application that is specified in the config.toml
If a TUI or GUI application such as GitUi, LazyGit, GitKraken is installed and specified in metadata it will be launched with the directory set to the dotfiles repo.
```
dotup ui
```
**or**
Add files/changes to git with dotup followed by a git command
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
- [x] add repo_names
- rename data.json and move to be in the repo
- Add logging.
- Get working for multiple systems (generalize home/local/config)
- change name to dotup
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
