# Effless

A modern, secure desktop application providing essential developer tools in a single interface. Built with Rust and Dioxus for performance, security, and cross-platform compatibility.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Dioxus](https://img.shields.io/badge/GUI-Dioxus-blue)](https://dioxuslabs.com/)

## 🚀 Features

### Encoders/Decoders  
- **Base64**: Encode and decode Base64 strings with error handling

### Generators
- **UUID**: Generate UUID v4 identifiers
- **ULID**: Generate ULIDs for distributed systems
- **QR Code**: Generate QR codes with customizable output
- **Icon Generator**: Create custom icons with text overlays and color customization

### Calculators
- **Haversine Distance**: Calculate distances between geographical coordinates

### System Design
- **Back-of-the-Envelope Calculator**: Sophisticated system architecture planning tool
  - Daily Active User (DAU) based calculations
  - Read/Write ratio analysis (1:1, 10:1, etc.)
  - Storage estimation with multi-unit display (Bytes → Petabytes)
  - Real-time RPS/TPS calculations

## 🛡️ Why Desktop?

Effless migrated from web to desktop for enhanced security and capabilities:
- **No network dependencies**: All processing happens locally
- **Enhanced privacy**: No data sent to external servers
- **Better performance**: Native Rust performance
- **System integration**: Clipboard access, file system operations
- **Offline availability**: Works without internet connection

## 📦 Installation

> **⚠️ In Development**: Effless is currently in active development. Pre-built binaries are not yet available. For now, you'll need to build from source.

### Prerequisites
- Rust 1.70.0 or later
- Cargo (comes with Rust)

### Build from Source
```bash
git clone https://github.com/yourusername/effless.git
cd effless
cargo build --release
./target/release/effless  # Run the built binary
```

### Development Mode
```bash
cargo run  # Build and run in development mode
```

### Package for Distribution

**Note**: Bundling is platform-specific. You must build on the target platform (macOS for macOS bundles, Windows for Windows bundles).

#### macOS
```bash
# Bundle for macOS (creates .app and .dmg)
dx bundle --platform desktop --package-types "macos" --package-types "dmg"
```

#### Windows
```bash
# Bundle for Windows (creates .exe and .msi)
dx bundle --platform desktop --package-types "msi"
```

The bundled applications will be created in the `dist/` directory. For cross-platform builds, consider using CI/CD services like GitHub Actions.

## 🎯 Usage

1. **Launch** the application
2. **Browse** tools in the categorized sidebar
3. **Search** for specific tools using the search box
4. **Select** a tool to open it in the main area
5. **Copy results** to clipboard when available

### Example: System Design Calculator
1. Enter your Daily Active Users (e.g., `1000000`)
2. Set Read:Write ratio (e.g., `10:1` for read-heavy systems)
3. Input average payload size in bytes (e.g., `1024`)
4. Click **Calculate** to get:
   - Reads per second
   - Writes per second  
   - Annual storage projections across all units

## 🏗️ Architecture

### Core Components
- **Main Application**: Dioxus desktop application with reactive state management
- **Tool System**: Modular architecture using Rust enums for type safety
- **Individual Tools**: Self-contained modules with Dioxus component patterns
- **Business Logic**: Separated testable business logic modules

### Tool Pattern
Each tool follows the Dioxus component pattern:
```rust
pub struct ToolName {
    // Tool configuration
}

impl ToolName {
    pub fn new() -> Self { /* ... */ }
    pub fn view(&self) -> Element { /* delegates to component */ }
}

#[component]
pub fn ToolNameView() -> Element {
    let state = use_signal(|| initial_state);
    // Component implementation
}
```

## 🧪 Testing

Run the full test suite:
```bash
cargo test
```

Run specific tool tests:
```bash
cargo test system_design  # System design calculator tests
cargo test base64         # Base64 tool tests
```

## 🛠️ Development

### Code Quality
```bash
cargo fmt         # Format code
cargo clippy      # Run linter
cargo check       # Type check without building
```

### Adding New Tools
1. Create a new module in `src/tools/`
2. Add the tool type to `ToolType` enum in `src/tools/mod.rs`
3. Implement the tool struct with required methods
4. Add to `Tool` enum and message routing
5. Update the sidebar tool list in `src/main.rs`

See [`CLAUDE.md`](./CLAUDE.md) for detailed development guidance.

## 📊 System Requirements

- **Memory**: 50MB RAM (typical usage)
- **Storage**: 10MB disk space
- **OS**: macOS, Linux, Windows
- **CPU**: Any modern processor (x86_64, ARM64)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup
1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and add tests
4. Run tests: `cargo test`
5. Commit your changes: `git commit -am 'Add new feature'`
6. Push to the branch: `git push origin feature-name`
7. Submit a pull request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Dioxus](https://dioxuslabs.com/) - A modern reactive GUI framework for Rust
- Inspired by the need for secure, offline developer tools
- Thanks to the Rust community for excellent crates and documentation

---

**Note**: Effless prioritizes security and privacy by keeping all processing local to your machine.