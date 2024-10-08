> [!WARNING]  
> This CLI tool was made for learning purposes. It is not fully functioinal/stable and probably never will be.


# dotup
A command line tool for backing up dotfiles. Inspired by [yadm](https://github.com/TheLocehiliosan/yadm).


### Setup a new repository:
Initialize a repository
```
dotup init
```
This will:
- Initialize the dotup git repo.
- Create dotup's configuration file `config.toml`.
After running `dotup init` you can run `dotup status` to see that `config.toml` exists in the dotup repo but has not been committed or added to staging.

Add a file to the list of tracked files:
```
dotup track ~/.zshrc
```
Check the status of the local repo:
```
dotup status
```
Commit the change:
```
dotup commit -m "Initial file"
```
Change branch to main:
```
dotup branch -M main
```
Add the remote origin (This repo should be set up already using Codeberg/Gitlab/Github/Gitea or similar.)
```
dotup remote add origin git@github.com:concur1/dotfiles.git
```
Push to the origin:
```
dotup push -u origin main
```

### Clone an existing repository (MAKE SURE TO INCLUDE THE `.`)
Clone the remote repo into the local dotup repo with:
```
dotup clone <url> .
```
The `config.toml` file should already include a section for the file mappings under `[files.hostname1]`.
But you should now also see that the config.toml file has been updated to include a section with the current `[files.hostname2]`.


### Tracking files
Add a file to the list of tracked files
```
dotup track <important file>
```
Remove a file from the list of tracked files
```
dotup untrack <important file>
```

### Use a git GUI/TUI application
Using the `run` command we can run a (git ui) application whilst the dotup repo files are being sync's with the system files.
To do this:
1. Create a section for the program you to be launched in the config.toml:
    ```
    [program.gitui] # This is the name of the section we will be configuring
    name = "gitui" # This is the name of the program that will be running
    additional_args = "--watcher --logging" # These are the arguments that will be supplied to the program
    ```
2. You will need to install the program you want to run using you prefered package manager e.g. `cargo install gitui`.
3. Run program and sync files with dotup:
    ```
    dotup run gitui
    ```

By default gitui, lazygit, gitkraken and hx are already configured. 

### Use command line git
Add files/changes to git with dotup followed by a git command
```
dotup status 
dotup add <important file>
dotup commit
```

### Example usage:
[demo.webm](https://github.com/concur1/dotup/assets/87602390/6012283e-7ab5-4176-aa6c-09f6db37bc8a)

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
- [x] rename data.json and move to be in the repo
- add check to ensure the mappings are file to file.
- add check to make sure that each file is mapped once per hostname.
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
- is 'track' the right word? Maybe 'mirror' is better?

some cases to consider:
- a file is merged into a system that doesn't need it, the file is untracked. Should the file still exist?
- should untack only work if the file exists locally or should we copy the file locally before using it?
- merging/cloning a repo with files more recent than the system files will lead to the system files being lost.
    - soln1 - create a seperate repo that only contains backups of the system files.
    - soln2 - detect when a clone/merge is going to overwrite system files
    - soln3 - only copy from system to the repo and never from the repo to the system.
    - soln4 - use `git pull --allow-unrelated-histories` and disallow/raise error for clone.
    - soln5 - make tracking seperate. Check if the tracking files match before allowing merge/pull1.
    - soln6 - have apply command that will apply the local repo to the system files.
        - This apply command will :
            - check that there is no merge conflict in the repo
            - check that the the tracked files in the repo are a subset of the system tracked files. 
            If there are repo tracked files that are not in system then raise a error/warning/options: "Additional Tracked files detected in local repo. Add these to the list of tracked files."

    - soln7 - Do not use a shared tracked files file. Instead each file will be specified locally or with a config file.
        - if using cli to specify files to track

