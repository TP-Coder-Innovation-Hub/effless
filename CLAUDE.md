# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Effless is a Rust desktop application built with the Iced GUI framework. It provides a collection of developer tools in a single interface, including encoders/decoders, generators, calculators, and system design utilities.

## Development Commands

### Building and Running
```bash
# Build the project
cargo build

# Run the application
cargo run

# Build for release
cargo build --release

# Check for compilation errors without building
cargo check
```

### Code Quality
```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test
```

## Architecture

### Core Structure
- **Main Application** (`src/main.rs`): Iced Application implementation with sidebar navigation and tool area
- **Tool System** (`src/tools/mod.rs`): Modular tool architecture using enums for type safety
- **Individual Tools** (`src/tools/*.rs`): Self-contained tool implementations

### Tool Architecture Pattern
Each tool follows a consistent pattern:
- Enum-based message system for UI events
- State management through update() method
- View generation through view() method returning Iced Elements
- Integration with system clipboard via arboard crate

### Available Tools
- **JSON**: Formatter and minifier for JSON data (Converters category)
- **Base64**: Encoder/decoder for Base64 text (Encoders/Decoders category)
- **URL**: URL encoding/decoding utilities (Encoders/Decoders category)
- **Hash**: Multiple hash algorithms (SHA2, MD5) with checksum generation (Generators category)
- **UUID**: UUID v4 generation (Generators category)
- **ULID**: ULID generation for distributed systems (Generators category)
- **QR Code**: QR code generator with image output (Generators category)
- **Icon Generator**: Simple icon generator with text and shape selection (circle, square), customizable colors and sizes, with PNG/ICO download support (Generators category)
- **Distance**: Haversine distance calculator for geographical coordinates (Calculators category)
- **System Design**: Back-of-the-envelope calculations for system architecture planning with DAU, read/write ratios, and storage estimation (System Design category)

### Key Dependencies
- `iced`: GUI framework (v0.13.1)
- `tokio`: Async runtime
- `arboard`: Clipboard integration
- `image`: Image processing for icon generation
- `rfd`: Native file dialog for downloads
- `ico`: ICO format conversion
- Tool-specific crates: `base64`, `serde_json`, `uuid`, `ulid`, `qrcode`, `sha2`, `md5`, `url`, `chrono`

### Adding New Tools
1. Create new module in `src/tools/`
2. Add tool type to `ToolType` enum in `src/tools/mod.rs`
3. Implement tool struct with `new()`, `update()`, and `view()` methods
4. Add tool to `Tool` enum and match patterns
5. Update sidebar tool list in `src/main.rs`

### UI Layout
- Two-column layout: sidebar (250px) + main tool area
- Sidebar includes search functionality and categorized tool list organized by function type
- Tool area displays the currently selected tool interface
- Consistent styling with Iced's built-in themes

## System Design Tool Details

The System Design tool has been enhanced with sophisticated back-of-the-envelope calculations based on proven system design principles:

### Key Features
- **Daily Active User (DAU) input**: Base calculation parameter for system load
- **Read:Write Ratio parsing**: Supports formats like "1:1", "10:1" for realistic traffic patterns  
- **Data size calculations**: Payload size in bytes with automatic unit conversions
- **Real-time calculations**: Computes reads/writes per second based on DAU distribution
- **Storage projections**: Annual storage estimates across multiple units (Byte → PB)
- **Copy to clipboard**: Export complete calculation results
- **Input validation**: Clear error messages for invalid inputs

### Calculation Constants
- `SECOND_IN_DAY = 86400` (24 hours × 60 minutes × 60 seconds)
- `DAY_IN_YEAR = 365` (standard year assumption)

### Core Algorithm
```
read_per_second = (DAU × read_ratio) / SECOND_IN_DAY
write_per_second = (DAU × write_ratio) / SECOND_IN_DAY  
storage_per_year = data_size × write_per_second × DAY_IN_YEAR
```

### Test Coverage
Comprehensive test suite covers:
- Valid input scenarios with expected precision
- Edge cases (zero DAU, large data sizes)
- Error conditions (invalid ratios, malformed inputs)
- High read ratio scenarios (10:1, 100:1 patterns)

## Icon Generator Tool Details

The Icon Generator tool creates simple icons with text overlays on colored backgrounds:

### Key Features
- **Text Input**: Up to 3 characters maximum for optimal icon readability
- **Shape Selection**: Circle or square background shapes
- **Size Customization**: Configurable icon size in pixels (default 128px)
- **Color Customization**: Hex color picker for background and text colors
- **Live Preview**: Real-time preview of generated icons
- **Multi-format Export**: Download as PNG or ICO formats
- **Base64 Export**: Copy Base64-encoded icon data to clipboard
- **Crash Protection**: Comprehensive error handling prevents application crashes

### Technical Implementation
- **Bitmap Font Rendering**: Custom 5x7 pixel bitmap patterns for A-Z and 0-9 characters
- **Dynamic Font Sizing**: Automatic scaling based on character count and icon size
- **Pixel-perfect Drawing**: Direct pixel manipulation for crisp text rendering
- **Character Spacing**: Proper spacing algorithm for multi-character icons
- **Shape Rendering**: Mathematical circle and square background generation
- **Format Conversion**: PNG to ICO conversion for Windows compatibility

### Usage Guidelines
- Keep text short (1-3 characters) for best results
- Use high contrast between background and text colors
- Recommended sizes: 16px, 32px, 64px, 128px, 256px for standard icon usage
- ICO format recommended for Windows applications, PNG for web/cross-platform