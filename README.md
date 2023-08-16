# octyl

octyl is a WIP text editor for the terminal, written in Rust. It aims to provide a text editing experience akin to that of ![helix](https://helix-editor.com/) or ![vim](https://www.vim.org/), with features like plugins, text highlighting, modal editing, etc. This project is still well in it's infancy, so don't bully me about how bad it is.









## Vision Board

- **Terminal-based:** Enjoy the convenience of a text editor directly in your terminal window, without the need for a graphical interface.

- **Highly Extensible Plugin System:** Customize your text editor experience by easily adding and managing plugins. The plugin architecture allows you to extend the functionality of the editor with very little performance cost.

- **Text Highlighting:** Increase readability and context with syntax highlighting for a wide range of programming languages and file types.

- **Efficient Editing:** Benefit from a responsive and efficient editing experience, even with large text files.

- **Keyboard Shortcuts:** Streamline your workflow using a variety of customizable keyboard shortcuts for common tasks.

- **Custom Themes:** Personalize your editor's appearance with a selection of built-in themes or create your own.

## Getting Started

### Prerequisites

Before you start, ensure you have the following prerequisites:

- Rust (nightly version recommended)
- Cargo (Rust's package manager)

### Installation

1. Clone this repository to your local machine:

   ```bash
   git clone https://github.com/yourusername/RustyTerminalTextEditor.git
   ```

2. Navigate to the project directory:

   ```bash
   cd RustyTerminalTextEditor
   ```

3. Build and run the editor:

   ```bash
   cargo run
   ```

### Usage

- **Opening Files:** Launch the editor and provide the file name as an argument to open it:

  ```bash
  cargo run my_file.txt
  ```

- **Keyboard Shortcuts:** The editor supports various keyboard shortcuts for navigation, editing, and more. Refer to the documentation for a complete list of shortcuts.

- **Plugins:** Extend the functionality of the editor by adding plugins. To enable a plugin, update the configuration file and restart the editor.

- **Themes:** Choose a theme for your editor by modifying the configuration. You can also create your own themes by following the provided guidelines.

### Contributing

Contributions to RustyTerminalTextEditor are welcome! If you'd like to contribute, please follow the guidelines outlined in [CONTRIBUTING.md](CONTRIBUTING.md).

### Roadmap

This project is still in active development, and here are some planned features:

- Auto-indentation
- Search and replace functionality
- Customizable keybindings
- Improved plugin system with API documentation

## Vision Board

- **Terminal-based:** Enjoy the convenience of a text editor directly in your terminal window, without the need for a graphical interface.

- **Highly Extensible Plugin System:** Customize your text editor experience by easily adding and managing plugins. The plugin architecture allows you to extend the functionality of the editor with very little performance cost.

- **Text Highlighting:** Increase readability and context with syntax highlighting for a wide range of programming languages and file types.

- **Efficient Editing:** Benefit from a responsive and efficient editing experience, even with large text files.

- **Keyboard Shortcuts:** Streamline your workflow using a variety of customizable keyboard shortcuts for common tasks.

- **Custom Themes:** Personalize your editor's appearance with a selection of built-in themes or create your own.

## License

This project is licensed under the [MIT License](LICENSE).
