# tidyup

A lightweight CLI tool written in Rust that organizes files by type.

`tidyup` scans a directory and moves supported files into categorized folders, keeping your workspace clean without manually sorting files.

## Features

- **Simple CLI** — organize a directory with a single command.
- **Automatic categorization** — sorts supported files into `Images/`, `Documents/`, `Videos/`, and `Audios/`.
- **Custom directory support** — organize the current directory or provide a path.
- **Lightweight** — built with Rust and designed to do one job simply.
- **Open source** — available on GitHub and installable through Cargo.

## Installation

### Option 1: Install from crates.io (recommended)

Requires a working Rust toolchain with Cargo.

```bash
cargo install tidyup-cli
```

This installs the `tidyup` executable.

Verify the installation:

```bash
which tidyup
```

### Option 2: Install directly from GitHub

To build and install the latest source from the repository's default branch:

```bash
cargo install --git https://github.com/utkarsh-saxena271/tidyup.git
```

To reinstall or replace an existing installation:

```bash
cargo install tidyup-cli --force
```

### Option 3: Download a prebuilt binary

Visit the [GitHub Releases](https://github.com/utkarsh-saxena271/tidyup/releases) page and download an asset matching your operating system and CPU architecture, if available.

Extract the archive and run the executable. On macOS, unsigned binaries may trigger Gatekeeper warnings. See the release notes and verify the downloaded artifact before running it.

> Prebuilt binaries are only available for the platforms and architectures listed in each release.

## Usage

### Organize the current directory

```bash
tidyup .
```

### Organize a specific directory

```bash
tidyup /path/to/directory
```

For example:

```bash
tidyup ~/Downloads
```

`tidyup` processes files in the specified directory and moves supported files into category folders created inside that directory.

## How it works

Suppose your directory contains:

```text
Downloads/
├── photo.jpg
├── report.pdf
├── presentation.pptx
├── song.mp3
└── video.mp4
```

After running:

```bash
tidyup .
```

The directory will look like:

```text
Downloads/
├── Images/
│   └── photo.jpg
├── Documents/
│   ├── report.pdf
│   └── presentation.pptx
├── Audios/
│   └── song.mp3
└── Videos/
    └── video.mp4
```

The category folders are created when needed. Files with unsupported extensions are left untouched.

**Important:** `tidyup` moves files; it does not copy them. Review the target directory before running it, and test with a disposable directory first if you are unsure about the result.

## Supported file types

| Category  | Extensions                                       |
| --------- | ------------------------------------------------ |
| Images    | `.jpg`, `.jpeg`, `.png`, `.gif`, `.webp`, `.svg` |
| Documents | `.docx`, `.pdf`, `.txt`, `.xlsx`, `.pptx`        |
| Videos    | `.mp4`, `.mkv`, `.avi`, `.mov`, `.webm`          |
| Audio     | `.mp3`, `.wav`, `.flac`, `.aac`, `.ogg`          |

Extension matching is currently case-sensitive. For example, `.jpg` is supported, but `.JPG` is not recognized by the current implementation.

## Limitations

- Only the specified directory's immediate entries are processed; subdirectories are not recursively scanned.
- Unsupported file types are left in place.
- The current version does not provide a dry-run mode or an undo command.
- Command-line options such as `--help` and `--version` are not implemented yet.

## Developmentnoty

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (included with Rust)

### Build from source

```bash
git clone https://github.com/utkarsh-saxena271/tidyup.git
cd tidyup
cargo build --release
```

The compiled executable will be available at:

```text
target/release/tidyup
```

### Run locally

```bash
cargo run -- .
```

### Run tests

```bash
cargo test
```

## Contributing

Contributions, bug reports, and feature suggestions are welcome.

1. Fork the repository.
2. Create a branch for your change.
3. Make your changes and test them.
4. Open a pull request describing what changed and how you tested it.

For bugs, include your operating system, `tidyup` version, the command you ran, and the expected versus actual behavior.

[Open an issue](https://github.com/utkarsh-saxena271/tidyup/issues) to report a bug or suggest an improvement.

## Roadmap

Potential improvements include:

- `--help` and `--version` flags.
- Dry-run mode to preview file movements.
- Better handling of uppercase extensions.
- Collision handling and clearer error reporting.
- Recursive organization as an optional feature.
- Automated multi-platform release builds.

## License

This project is licensed under the [MIT License](LICENSE).

## Links

- **Source code:** [GitHub](https://github.com/utkarsh-saxena271/tidyup)
- **Package:** [crates.io/crates/tidyup-cli](https://crates.io/crates/tidyup-cli)
- **Releases:** [GitHub Releases](https://github.com/utkarsh-saxena271/tidyup/releases)
- **Issues:** [Report a bug or request a feature](https://github.com/utkarsh-saxena271/tidyup/issues)
