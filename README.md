# Sole: A Terminal Text Editor.

## Description

This is a in-terminal text editor with the following features:

- Create new files.
- Update existing files.
- Search within file.
- Track file modification.
- Syntax Highlighting for rust files.

## Usage

Use the following command in the directory of your choice to open a new file:

```bash
   sole
```

Use the following command to open a file of your choice:

```bash
   sole {YOUR_FILE_NAME}
```

## Installation

Inorder to build this project you need to have Rust and Cargo installed on your system. You can do it from [Rust Website](https://www.rust-lang.org/tools/install).

1. Git Clone the repository to your workspace, and navgigate into the directory.

```bash
   git clone https://github.com/amar-nath-bhat/sole.git
   cd sole
```

2. Now build it with Cargo.

```bash
   cargo build --release
```

3. Now install the binary.

```bash
   cargo install --path .
```

4. Finally run the `sole` command with your `FILE_NAME`.

```bash
   sole {YOUR_FILE_NAME}
```

## Future Roadmap:

Here are some planned features and improvements for future:

- Support Syntax Highlighting for multiple filetypes.
- Auto Indentation
- Line Wrap

## Credits

The information and inspiration for this project was sourced from the following references:

- [Phillip's Blog](https://www.flenker.blog/hecto/)
- [Rust Documentation](https://doc.rust-lang.org/book/)

## License

See the [LICENSE](https://github.com/amar-nath-bhat/sole/blob/main/LICENSE) file for more details.
