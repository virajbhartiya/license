# [LICENSE GENERATOR](license.virajbhartiya.com)

Generate license files for your projects.

## Installation

```shell
git clone https://github.com/virajbhartiya/license.git
cd license
cargo install --path .
```

## Usage

### Install a license

Generate a license file with the specified license, year, fullname, project, and extension.

```shell
license install <license> -y <year> -n <fullname> -p <project> -e <extension>
```

Replace `<license>` with the name of the license, `<year>` with the year, `<fullname>` with your full name, `<project>` with the name of the project, and `<extension>` with the file extension for the license.

### View a license

View the contents of a license file.

```shell
license view <license>
```

Replace `<license>` with the name of the license.

## Building for macOS

To build an executable for macOS, run the following command:

```shell
cargo build --release --target x86_64-apple-darwin
```

This will create a `license` binary in the `target/release/x86_64-apple-darwin` directory.

## Contributing

Contributions are welcome! If you'd like to contribute, please open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [chrono](https://github.com/chronotope/chrono) - Date and time handling
- [license-generator](https://github.com/yourusername/license-generator) - License generation logic
