# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Effless is a Rust desktop application built with the Dioxus GUI framework. It provides a collection of developer tools in a single interface, including encoders/decoders, generators, calculators, and system design utilities.

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
- **Main Application** (`src/main.rs`): Dioxus desktop application with reactive state management, sidebar navigation and tool area
- **Tool System** (`src/tools/mod.rs`): Modular tool architecture using enums for type safety
- **Individual Tools** (`src/tools/*.rs`): Self-contained tool implementations with proper Dioxus component patterns
- **Business Logic** (`src/logic/mod.rs`): Separated testable business logic modules for core functionality

### Tool Architecture Pattern
Each tool follows a consistent pattern:
- **Struct Pattern**: Tool struct with `new()` and `view()` methods that delegate to component functions
- **Component Pattern**: Proper `#[component]` functions with `use_signal` hooks at the top level
- **Hooks Compliance**: All hooks (`use_signal`, `use_effect`) called at component top level, never conditionally
- **Business Logic Separation**: UI components use `crate::logic::*` modules for testable business operations
- **CSS-in-JS Styling**: Consistent layout with proper padding, flexbox, and overflow control to prevent scrolling
- **Clipboard Integration**: Copy functionality via arboard crate for all tools

### Available Tools
- **Base64**: Encoder/decoder for Base64 text using business logic (Encoders/Decoders category)
- **UUID**: UUID v4 generation with counter tracking (Generators category)
- **ULID**: ULID generation for distributed systems with timestamp sorting (Generators category)
- **QR Code**: QR code generator placeholder with input validation (Generators category)
- **Icon Generator**: Simple icon generator with text overlays, shape selection, and color customization (Generators category)
- **Distance**: Haversine distance calculator using validated coordinate business logic (Calculators category)
- **System Design**: Back-of-the-envelope calculations for system architecture planning with DAU, read/write ratios, and storage estimation (System Design category)

### Key Dependencies
- `dioxus`: Modern reactive GUI framework (v0.6) with desktop support and component-based architecture
- `dioxus-desktop`: Desktop platform integration for native window management
- `tokio`: Async runtime for Dioxus desktop applications
- `arboard`: System clipboard integration for copy functionality
- Tool-specific crates: `base64`, `uuid`, `ulid`, `percent-encoding`, `url`, `sha2`, `md5`

### Adding New Tools
1. Create new module in `src/tools/`
2. Add tool type to `ToolType` enum in `src/tools/mod.rs`  
3. Implement tool struct with `new()` and `view()` methods that delegate to component functions
4. Create `#[component]` function with proper hooks usage at the top level
5. Add business logic to `src/logic/` if needed for testable operations
6. Add tool to `Tool` enum and match patterns in `tools/mod.rs`
7. Update sidebar tool list in `src/main.rs`
8. Follow consistent CSS styling: `padding: 20px; height: 100%; display: flex; flex-direction: column; box-sizing: border-box; overflow: hidden;`

### Important Dioxus Patterns
- **Hooks Rules**: Always call `use_signal`, `use_effect` at component top level, never conditionally
- **Component Structure**: Use `#[component] pub fn ComponentView() -> Element` pattern
- **State Management**: Use `use_signal` for reactive state, avoid direct state in structs
- **Error Prevention**: Proper CSS layout prevents content overflow and scrolling issues

### UI Layout
- Two-column layout: sidebar (250px) + main tool area using CSS absolute positioning
- Sidebar includes search functionality and categorized tool list organized by function type
- Tool area displays the currently selected tool interface with reactive state
- Proper viewport control: `position: absolute; top: 0; left: 0; right: 0; bottom: 0` prevents scrolling
- Consistent styling with CSS-in-JS approach for maintainability

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

## Business Logic Architecture

### Logic Modules (`src/logic/`)
- **base64_logic.rs**: Base64 encoding/decoding with proper error handling
- **distance_logic.rs**: Haversine distance calculations with coordinate validation
- Each module provides struct-based API with comprehensive unit test coverage (18 tests passing)

### Benefits
- **Separation of Concerns**: Business logic separated from UI components  
- **Testability**: Logic modules have comprehensive unit test coverage
- **Reusability**: Business logic can be shared across components
- **Error Handling**: Structured error types for robust operation

### Usage Pattern
```rust
use crate::logic::base64_logic::Base64Logic;

let encoded = Base64Logic::encode(input);
match Base64Logic::decode(input) {
    Ok(decoded) => /* handle success */,
    Err(error) => /* handle error */,
}
```