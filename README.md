## dotup
A command line tool for backing up dotfiles.


TODO:
- Get tracking working on local system.
    - [x] Use repo path in notify watcher
    - [x] Get untrack working.
    - [x] Get syncing working on local system.
    - [x] Get syncing to pick up newly added files
- [x] Get notify/sync command working.
- [x] create config
- [x] Run gitui/lazygit/other from updot
- Update to use git commands
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
