Contributing to Casper
Thank you for your interest in contributing to Casper! We welcome contributions from the community to help improve this JARVIS-inspired ghost copilot. Whether you're fixing bugs, adding features, improving documentation, or suggesting ideas, your help is appreciated.
Code of Conduct
By participating in this project, you agree to abide by our Code of Conduct. Please read it to understand the expectations for behavior in our community.
How to Contribute
Reporting Issues
If you find a bug or have a feature request:

Check if the issue already exists in the Issues section.
If not, open a new issue with a clear title and description. Include steps to reproduce (for bugs), expected behavior, and any relevant logs or screenshots.

Submitting Pull Requests

Fork the Repository: Create a fork of the main repository on GitHub.
Clone Your Fork: Clone your fork locally:git clone https://github.com/yourusername/casper.git
cd casper


Create a Branch: Create a new branch for your changes:git checkout -b feature/your-feature-name


Make Changes: Implement your changes. Follow the coding guidelines below.
Test Your Changes: Run tests and ensure everything works:
Build: cargo build --workspace
Test: cargo test --workspace
Run Daemon: cd casper-daemon && cargo run
Run TUI: cd casper-tui && cargo run


Commit Changes: Use clear, descriptive commit messages:git commit -m "Add feature: voice recognition integration"


Push to Your Fork: Push the branch to your fork:git push origin feature/your-feature-name


Open a Pull Request: Go to the original repository and open a PR from your fork. Provide a detailed description of your changes, reference any related issues, and explain why the change is needed.

Coding Guidelines

Rust Edition: Use Rust 2024 edition.
Formatting: Run cargo fmt before committing.
Linting: Use cargo clippy to catch common mistakes.
Dependencies: Add new dependencies sparingly; justify them in your PR.
Error Handling: Use Result for functions that can fail; provide meaningful error messages.
Modularity: Keep features in separate modules within casper-core.
Testing: Add unit tests for new functions; aim for high coverage.
Documentation: Use Rustdoc comments for public functions; update README.md if needed.
Platform Focus: Initial focus is ArchLinux with Gnome/Wayland; test changes there.

Development Setup

Install Rust: Use rustup (https://rustup.rs/).
Install System Dependencies (ArchLinux):sudo pacman -S espeak-ng grim libnotify gtk4


Build the Workspace:cargo build --workspace


Run the Daemon and Clients as described in README.md.

Areas for Contribution

Implement placeholders (e.g., voice with vosk-rust, AI with rust-bert).
Add support for MCP (clarify requirements).
Enhance TUI with more features (e.g., menu for request types).
Improve tray integration for Wayland/Gnome.
Add cross-platform support (e.g., X11 fallback).
Write tests and documentation.

Questions?
If you have questions, open an issue or join discussions on GitHub.
Thanks for contributing to Casper!