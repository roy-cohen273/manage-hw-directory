# Create HW Folder
A tool for managing homework (HW) folders.

## Features
### Backend
* [x] Get a list of subjects.
* [x] Get the questions file: the last downloaded item from downloads directory.
* [x] Create a new HW directory under the chosen subject directory
  * [ ] Better parameterization:
        `HW_DIR_FORMAT` should accept a `subject_dir` parameter.
        In most use cases, the `HW_DIR_FORMAT` would be `{subject_dir}/...`.
* [x] Move the questions file into the HW directory.
* [x] Copy the LyX template into the HW directory.
* [x] Substitute parameters into the LyX file.
* [ ] Open an HW directory:
  * [ ] Open the questions file (e.g. using chrome).
  * [ ] Open the LyX file (using LyX).
* [ ] Better parameterization in general.
  * [ ] Hebrew names for subjects.

### Commandline Frontend
* [x] Display list of subjects.
* [x] Pick a subject.
  * [ ] Accept incomplete name fo a subject.
        e.g. if the user types `"Algo"` it should be recognized as `"Algorithms"`.
* [ ] Open the last HW directory in a subject.

### TUI Frontend
Not currently under development.
