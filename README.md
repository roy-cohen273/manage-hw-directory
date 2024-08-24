# Manage HW Directory
A tool for managing homework (HW) directories.

## Installation
1. Install `cargo`.
2. Clone this repo.
3. Modify `settings.json5`.
4. `$ cargo run`.

## Features/Progress
### Backend
* [x] Get a list of subjects.
* [x] Get the questions file: the last downloaded item from downloads directory.
* [x] Create a new HW directory under the chosen subject directory
* [x] Move the questions file into the HW directory.
* [x] Copy the LyX template into the HW directory.
* [x] Substitute parameters into the LyX file.
* [x] Open an HW directory:
  * [x] Open the questions file (e.g. using chrome).
  * [x] Open the LyX file (using LyX).
* [ ] Better parameterization in general.
  * [x] Per-subject settings file. 
  * [x] Hebrew names for subjects.

### Commandline Frontend
* [x] Display list of subjects.
* [ ] Pick a subject.
  * [x] Accept index in the subjects list instead of full name.
  * [ ] Sort subjects list (by last update time?)
* [x] Open the last HW directory in a subject.

### TUI Frontend
Not currently under development.
