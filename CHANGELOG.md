# Changelog

## 🎉 [0.0.1] - 2024-05-22

### ✂️ Revert

- The reset command has been removed

### ✨ Features

- Added basic commands
- Added module Msg for outputting messages
- Added Init command for creating a configuration file
- Added Next command for increasing the version number
- Added Git module
- Added git methods for creating, renaming and switching branches
- Two new commands have been added: feat and fix
- Added helpers
- Added Commit method to Git module
- Added Changelog module for creating CHANGELOG.md file
- Updated Changelog module
- Generation of the CHANGELOG.md file has been replaced from using an external utility to the git-cliff-core library
- Added Merge method to Git module
- Added next_to_current method to ProjectConfig module
- Added Reset method to Git module
- New fields "changelog" and "package_managers" has been added to the ProjectConfig structure
- The "changelog" field in the rellr.json file can be used to change the default name from CHANGELOG.md to any other name
- PackageManagerTrait and two structures (Cargo and Npm) have been added to the ProjectConfig structure
- The "to_path_str" function has been added to the "Helpers" module to convert a vector of strings to a path string
- The "release" command has started using "package managers" from "rellr.json" to flexibly add config files for committing a release
- Checking for configuration files has been added when running the release command
- Launch of publishing to the npm repository has been added when running the "release" command
- Added "only_changelog" argument to release command
- Project description and logo added

### 🎛️ Refactor

- Git module method "repo" has been renamed to "new"
- Git module methods "get_branch_ref_name" and "checkout" are updated
- Init command has been updated
- Small fix to the "build" method in the changelog module

### 🛠️ Bug Fixes

- Fixed increment function
- Improved performance of the Git module
- Added additional methods to Git module
- Removed optional arguments
- Improved performance of the Release command
- The “checkout” and “commit” methods in the Git module has been fixed

### 🧪 Testing

- Added examples

