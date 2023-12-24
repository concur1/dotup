## dotup
A command line tool for backing up dotfiles.


TODO:
- add consts for repo path and data.json 
- Get tracking working on local system.
    - [x] Use repo path in notify watcher
    - [x] Get untrack working.
    - [x] Get syncing working on local system.
    - [x] Get syncing to pick up newly added files
- [ ] create config
- [ ] Run gitui/lazygit/other from updot
- Add logging.
- Get notify/sync command working.
- Get working for multiple systems (generalize home/local/config)
- Test running with gitui
- change name to updot
- add additional info to updot status
- Create tests:
    - test track
    - test untrack
    - test git commands including merge
    - test gitui/lazygit launches ect
    - Fix warnings.
- Create just file with linting/tests.
