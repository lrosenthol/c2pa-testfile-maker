# Contributing to C2PA Testfile Maker

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Development Setup

### Prerequisites

- Rust 1.70 or later
- C/C++ compiler (required for some dependencies)
- Git

### Getting Started

1. Clone the repository:
```bash
git clone https://github.com/lrosenthol/c2pa-testfile-maker.git
cd c2pa-testfile-maker
```

2. **Important**: This project depends on a local copy of the c2pa-rs library. You'll need to clone it:
```bash
cd ..
git clone https://github.com/contentauth/c2pa-rs.git
cd c2pa-testfile-maker
```

The `Cargo.toml` references the c2pa-rs SDK via a relative path: `{ path = "../c2pa-rs/sdk" }`

3. Build the project:
```bash
cargo build
```

4. Run tests:
```bash
cargo test
```

## Running Tests

The integration tests require test certificates. They are automatically included in the repository at `tests/fixtures/certs/`.

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_dog_jpg_simple_manifest
```

See [TESTING.md](TESTING.md) for more details on the test infrastructure.

## Code Style

This project follows standard Rust conventions:

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix any warnings
- Write tests for new functionality
- Update documentation as needed

```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy

# Check formatting without changing files
cargo fmt -- --check
```

## Making Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for your changes
5. Ensure all tests pass (`cargo test`)
6. Run `cargo fmt` and `cargo clippy`
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to your branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

## Pull Request Guidelines

- Provide a clear description of the changes
- Reference any related issues
- Ensure CI checks pass
- Update documentation if needed
- Add tests for new functionality

## Project Structure

```
c2pa-testfile-maker/
├── src/
│   └── main.rs           # Main CLI application
├── tests/
│   ├── common/
│   │   └── mod.rs        # Test helper functions
│   ├── fixtures/
│   │   └── certs/        # Test certificates
│   └── integration_tests.rs  # Integration tests
├── examples/             # Example manifest files and certificates
├── testfiles/            # Test images
└── Cargo.toml
```

## Adding New Features

When adding new features:

1. Update the CLI arguments in `src/main.rs` if needed
2. Add example manifest files to `examples/` if relevant
3. Add integration tests in `tests/integration_tests.rs`
4. Update the README.md with usage examples
5. Update TESTING.md if test setup changes

## Testing with Different Formats

To add support for a new media format:

1. Add test images to `testfiles/`
2. Add test cases in `tests/integration_tests.rs`
3. Update `get_test_images()` in `tests/common/mod.rs`
4. Ensure the c2pa library supports the format

## Reporting Issues

When reporting issues, please include:

- Rust version (`rustc --version`)
- Operating system
- Steps to reproduce
- Expected vs actual behavior
- Error messages or logs

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Help others learn and grow

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

Feel free to open an issue for questions or discussion!

## Resources

- [C2PA Specification](https://c2pa.org/specifications/specifications/1.0/index.html)
- [c2pa-rs Documentation](https://docs.rs/c2pa/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

