
<p align="center">
  <img src="https://raw.githubusercontent.com/lacodda/rellr/main/rellr.webp" width="320" alt="rellr">
</p>
<h1 align="center">rellr</h1>
<br>

rellr is a command-line tool designed to automate the release creation process for software projects. It streamlines the generation of release notes, collects changes, and prepares release information, making the release process more efficient and less error-prone.

## Features ✨

- **Automated Release Notes Generation** 📝: Automatically generate detailed release notes based on commit messages and tags.
- **Git Integration** 🔗: Seamlessly interact with Git repositories to gather necessary information for releases.
- **Configuration File Handling** ⚙️: Supports TOML and JSON configuration files for flexible and customizable setups.
- **Command-Line Interface** 💻: Easy-to-use CLI with various options and arguments to fit different workflows.

## Installation 🛠️

To install rellr, ensure you have [Rust](https://www.rust-lang.org/) installed, then run:

```sh
cargo install rellr
```

## Usage 🚦

After installation, you can use rellr directly from your command line. Below are some common commands and options:

### Initialize Configuration ⚙️

```sh
rellr init
```

### Create a New Release 🎉

```sh
rellr release
```

### Full Command List 📜

For a complete list of commands and options, run:

```sh
rellr --help
```

## Configuration 🛠️

rellr uses a configuration file (`rellr.toml` or `rellr.json`) to manage settings. Below is an example of a `rellr.toml` configuration file:

```toml
[release]
version = "0.1.0"
tag_prefix = "v"
```

## Dependencies 📦

rellr relies on several Rust libraries:

- `clap` for command-line argument parsing.
- `colored` for colored terminal output.
- `git2` for Git repository interactions.
- `path-absolutize` for path manipulation.
- `regex` for regular expression handling.
- `serde` and `serde_json` for configuration file parsing.
- `git-cliff-core` for generating changelogs.

## Contributing 🤝

Contributions are welcome! Please fork the repository and submit a pull request with your changes. For major changes, please open an issue first to discuss what you would like to change.

## License 📄

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Author 👤

rellr is developed and maintained by [Kirill Lakhtachev](https://lacodda.com).

## Acknowledgements 🙏

Special thanks to all the contributors and the open-source community for their invaluable work and support.

## Links 🔗

- [Repository](https://github.com/lacodda/rellr)
- [Issues](https://github.com/lacodda/rellr/issues)
- [Pull Requests](https://github.com/lacodda/rellr/pulls)