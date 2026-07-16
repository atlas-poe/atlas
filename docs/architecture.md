# Atlas Architecture

This document describes the architecture of Atlas, a native Path of Exile companion application.

## Overview

Atlas is designed as a modular Rust workspace with clear separation of concerns. The architecture prioritizes:

- **Modularity**: Each crate has a single responsibility
- **Testability**: Components can be tested in isolation
- **Performance**: Native code with minimal overhead
- **Maintainability**: Clear boundaries between components

## Workspace Strategy

Atlas uses a Cargo workspace to organize multiple crates:

```
atlas/
├── Cargo.toml           # Workspace root
├── crates/
│   ├── atlas-core/      # Core domain logic
│   ├── atlas-trade/     # Trade API integration (planned)
│   ├── atlas-parse/     # Item parsing (planned)
│   ├── atlas-ui/        # User interface (planned)
│   └── atlas-app/       # Application entry point (planned)
└── docs/
```

### Benefits

- **Shared dependencies**: Common dependencies are declared once at workspace level
- **Atomic changes**: Changes across crates can be made in a single commit
- **Incremental compilation**: Only modified crates are recompiled
- **Clear ownership**: Each crate has defined responsibilities

## Crate Responsibilities

### atlas-core

**Purpose**: Core domain logic and data structures

**Responsibilities**:
- Item data models (rarity, type, modifiers)
- Currency data models
- Trade query structures
- Price calculation logic
- Validation rules

**Dependencies**: Minimal (serde for serialization)

### atlas-trade (planned)

**Purpose**: Trade API integration

**Responsibilities**:
- Official trade API client
- Rate limiting and request management
- Response parsing
- Authentication handling

**Dependencies**: atlas-core, HTTP client library

### atlas-parse (planned)

**Purpose**: Item data parsing

**Responsibilities**:
- Clipboard item parsing
- Text-to-structure conversion
- Modifier extraction
- Item validation

**Dependencies**: atlas-core, regex/text parsing

### atlas-ui (planned)

**Purpose**: User interface components

**Responsibilities**:
- GUI framework integration
- Component library
- Theme management
- User interactions

**Dependencies**: atlas-core, GUI framework

### atlas-app (planned)

**Purpose**: Application entry point and orchestration

**Responsibilities**:
- Application lifecycle
- Component wiring
- Configuration management
- Error handling

**Dependencies**: All other crates

## Application Layers

Atlas follows a layered architecture:

```
┌─────────────────────────────────────┐
│           Presentation              │
│         (atlas-ui, atlas-app)       │
├─────────────────────────────────────┤
│          Application                │
│        (use cases, orchestration)   │
├─────────────────────────────────────┤
│            Domain                   │
│     (atlas-core: models, rules)     │
├─────────────────────────────────────┤
│         Infrastructure              │
│   (atlas-trade, atlas-parse)        │
└─────────────────────────────────────┘
```

### Layer Responsibilities

1. **Presentation**: User interface and interaction handling
2. **Application**: Orchestrates domain logic and infrastructure
3. **Domain**: Core business rules and data structures
4. **Infrastructure**: External system integration (APIs, file system)

### Dependency Rule

Dependencies flow inward: Presentation → Application → Domain ← Infrastructure

Domain logic (atlas-core) has no dependencies on infrastructure, ensuring testability and portability.

## Data Flow

### Trade Search Flow

```
User Input
    │
    ▼
┌─────────────┐
│  atlas-ui   │  User enters search criteria
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  atlas-app  │  Validates input, creates query
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ atlas-trade │  Sends request to trade API
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  atlas-core │  Parses results into domain models
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  atlas-ui   │  Displays results to user
└─────────────┘
```

### Item Parsing Flow

```
Clipboard Content
    │
    ▼
┌─────────────┐
│ atlas-parse │  Extracts item data from text
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  atlas-core │  Validates and structures item
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  atlas-ui   │  Displays item analysis
└─────────────┘
```

## Future GUI Architecture

Atlas will use a native GUI framework. The current plan is [Iced](https://github.com/iced-rs/iced), but this may evolve.

### GUI Principles

1. **Reactive**: UI updates in response to state changes
2. **Component-based**: Modular, reusable UI components
3. **Cross-platform**: Works on Linux, macOS, and Windows
4. **Accessible**: Follows accessibility guidelines

### Proposed Structure

```
atlas-ui/
├── src/
│   ├── lib.rs
│   ├── app.rs           # Main application
│   ├── components/      # Reusable UI components
│   │   ├── mod.rs
│   │   ├── item_card.rs
│   │   ├── search_bar.rs
│   │   └── price_display.rs
│   ├── views/           # Screen layouts
│   │   ├── mod.rs
│   │   ├── search.rs
│   │   ├── item.rs
│   │   └── settings.rs
│   └── theme.rs         # Styling
```

## Error Handling Strategy

Atlas uses Rust's type system for error handling:

1. **No panics in library code**: Use `Result` types everywhere
2. **Custom error types**: Each crate defines its error enum
3. **Error propagation**: Use `?` operator for clean error chains
4. **User-friendly messages**: Convert errors to readable messages at UI boundary

```rust
// Example error type
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Invalid item rarity: {0}")]
    InvalidRarity(String),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
}
```

## Testing Strategy

### Unit Tests

- Test individual functions and methods
- Mock external dependencies
- Run with `cargo test`

### Integration Tests

- Test crate interactions
- Use real dependencies where appropriate
- Located in `tests/` directories

### Property-Based Testing

- Use `proptest` for complex validation logic
- Generate random inputs to find edge cases

## Performance Considerations

1. **Compile times**: Workspace structure enables incremental compilation
2. **Runtime performance**: Native Rust code with zero-cost abstractions
3. **Memory safety**: No garbage collector, predictable performance
4. **Concurrency**: Leverage Rust's ownership model for safe parallelism

## Security Considerations

See [SECURITY.md](../SECURITY.md) for detailed security policies.

Key principles:
- Never hardcode API keys
- Validate all external input
- Use HTTPS for all API calls
- Minimize attack surface

## Design Decisions

### Why Rust?

- **Performance**: Critical for real-time price calculations
- **Safety**: Memory safety without garbage collection
- **Ecosystem**: Strong crate ecosystem for our needs
- **Cross-platform**: Excellent support for all target platforms

### Why Workspace?

- **Modularity**: Clear separation of concerns
- **Testability**: Components can be tested independently
- **Scalability**: Easy to add new crates as needed
- **Collaboration**: Multiple developers can work on different crates

### Why Layered Architecture?

- **Maintainability**: Changes in one layer don't cascade
- **Testability**: Domain logic can be tested without UI
- **Flexibility**: Infrastructure can be swapped without affecting domain
- **Clarity**: Clear responsibility boundaries

## Contributing to Architecture

When proposing architectural changes:

1. Open an issue discussing the change
2. Document the motivation and alternatives
3. Get feedback from maintainers
4. Implement incrementally
5. Update this documentation

## References

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
