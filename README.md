# i3switch – Multi-language Implementations

The project started due to personal frustration with i3 window manager's switching controls,
and ideas for improving the experience. The most notable issues were:

- To switch tabs, You had to select the parent container (sometimes multiple times),
  which, while deterministic, was cumbersome and dated.
- Direction based switching would often lead to unexpected results, especially with
  multiple containers.
- Often, when switching directionally, the focus would jump to a hidden container, namely
  tabs and stack containers, which would lead to confusion and frustration.

The first issue was the easiest to solve, as You could search i3 tree for the focused window
and find the parent tabbed container. This is the feature that I was using since I've started
the project and never looked back.

The second and third issues required consideration of multiple display setups, and
understanding of i3 tree structure quirks, which is not exactly intuitive. However,
this is understandable, due to the architecture of i3 and all of its features.

## Multiple languages

With interest in developing my skills I've decided to implement the same functionality
in multiple languages: Python, C++, and Rust. Each implementation has its own strengths and
weaknesses, and serves as a learning experience for both the developer and users.

Due to the nature of multiple implementations, it's infeasible to expect to maintain
exact similarity and feature parity across all languages. Consider **Rust implementation as the most
complete** and feature-rich, with C++ being the second, and Python as a reference implementation.

### Why Multiple Languages?

- **Educational value**: Demonstrates tradeoffs and idioms across ecosystems.
- **Practical insight**: Shows why certain languages are more suitable for certain tasks (e.g., startup time, memory use).
- **Portfolio**: Demonstrates adaptability and cross-language expertise.

## Project Structure

```
i3switch/
├── python/      # Python implementation
├── cpp/         # C++ implementation
├── rust/        # Rust implementation
├── scripts/     # Shared scripts (e.g., changelog generation, builds)
├── .github/
│   └── workflows/  # CI/CD automation (changelog, builds, releases)
└── README.md    # This file
```

Each implementation is self-contained, with its own dependencies, build instructions, and documentation.

## Building & Running

Each implementation has it's own build.sh script, that is wrapped with root build.sh script.
You can run the build script from the root directory to build selected implementation(s):

```bash
./build.sh python  # Build Python implementation
./build.sh cpp     # Build C++ implementation
./build.sh rust    # Build Rust implementation
./build.sh all     # Build all implementations
./build.sh all rebuild  # Rebuild all implementations
```

The root build script will output where the output binaries are located.
You can also run the implementations directly from their directories:

```bash
cd python && ./build.sh release && ./dist/i3switch
```

**Proper binary versioning require .git repository to be present and tags to be fetched.**

---

## Language Notes & Motivation

### Python

Initial implementation in Python resembled the current i3switch, however after trying to use it
in i3, it was found to be **too slow** for frequent CLI invocation. The new Python implementation
relies on i3-msg calls for switching and tries to bind shortcuts, to run as a background daemon.
This implementation didn't allow for much flexibility in terms of i3 configuration, so it was
decided to implement the same functionality in C++.

- Fastest for prototyping and learning.
- **Not optimal for frequent CLI invocation** due to slow startup (e.g., for i3 utilities).
- Maintained as a reference implementation and for language comparison.
- Great portability and dependency management.
- Distribution requires Python interpreter and dependencies, which may not be available in all
  environments.

### C++

This was the first implementation that was **fast enough** for frequent CLI invocation. It required
a major restructuring that allowed for proper geometric based window switching. To ensure it's
responsiveness, it implements it's own i3 tree navigation, that's designed to reduce copying and
allocation overhead.

- Near-instant startup, low memory, suitable for system utilities.
- Classic choice for high-performance, resource-sensitive tasks.
- Available in most environments, making it easy to integrate.
- Requires careful memory management, but provides fine-grained control over performance.
- Implies mindfulness about dependencies and portability, as it may not be available in all
  environments.
- Distribution requires C++ libraries, which may not be available in all environments.

### Rust

The Rust implementation was added to explore modern systems programming paradigms and memory safety.
It tightly resembles the C++ implementation, but uses Rust's ownership model to ensure memory
safety. Additionally the simplicity of Rust testing and build system allowed to find and fix
long prominent bugs present in the C++ implementation.

- Modern, memory-safe, zero-cost abstraction.
- Comparable performance to C++, with additional compile-time safety.
- Great for learning Rust's systems programming features.
- Excellent for building robust, strongly tested applications.
- Crates enable robust system for extending language capabilities, which encourages small, focused
  libraries.
- Distribution can be simplified with Cargo, but requires Rust toolchain to be installed.

---

## Automation & CI/CD

- Changelogs are generated from commit history from matching commits.
- Versioning of binaries is handled by Git tags, e.g., `python-vX.Y.Z`, `cpp-vX.Y.Z`, `rust-vX.Y.Z`.

### Reproducibility

With version tags and automated builds, you can always reproduce a specific release of any
implementation, keeping track of changes, features, and bug fixes across languages.

### Changelog Generation

Changelogs are generated with git cliff from commit messages.

To generate a changelog for a specific language implementation, run in git repository root:

```bash
git cliff --config=<LANG>/cliff.toml --output CHANGELOG.md
```

---

## Commit & Tag Conventions

- **Commits**: Follow Structured Commit Messages
  - **Format**: `<type>(<context>): <description>`
    - **Type**: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`
    - **Context**: Language prefix (e.g., `py`, `cpp`, `rs`)
    - **Description**: lowercase summary of the change in the imperative mood
  - **Example**: `feat(py): support config files`

---

## Getting Started

See each subdirectory for build/run instructions:

- [python/README.md](python/README.md)
- [cpp/README.md](cpp/README.md)
- [rust/README.md](rust/README.md)

---

## Contribution

Contributions welcome!
Please follow the guidelines below, and feel free to open issues or pull requests.
These are for the sake of consistency and as a reminder of the projects principles.

### Rules for Contributions

- **Follow commit conventions**: Use structured commit messages to ensure clarity and consistency.
- **Prefer minimal dependencies**: Especially in C++ and Rust, avoid unnecessary libraries.
- **Follow coding conventions**: Each language has its own style guide, so please follow it.
- **Document the code**: Don't use cat-like comments (don't repeat the code in comments). Think
  about what is not obvious and try to explain the problem that the code solves.
- **Test your changes**: Ensure your changes are tested, especially in Rust and C++.
- **State the purpose of your changes**: In the commit message, explain what your change does
  and why it is needed.
- **Keep existing functionality intact**: If you are adding new features, ensure that existing
  functionality is not broken.
- **Write code from the usability perspective**: Start from the higher-level features and
  abstraction, this will make the code descriptive and easy to understand. Even if the details
  will force You to produce multiple prototypes, the result will be more satisfactory.
- **Doxygen is not welcome**: I make the rules here. And my personal opinion is that it
  while being useful for large projects, results in cluttered code and sloppily written
  description of the code.

### Project Paradigms

- KISS (Keep It Simple, Stupid): Focus on simplicity and clarity (as much as possible, because i3
  tree navigation is a noodle).
- DRY (Don't Repeat Yourself): Avoid duplication, especially in i3 tree navigation logic.
- YAGNI (You Aren't Gonna Need It): Don't add features until they are needed – this is a utility,
  not a framework.
- !SOLID is NOT welcome!: This project is not meant to be a complex, object-oriented system.
  It is a simple utility that should be easy to understand and maintain. Although some of it's
  principles are still recommended, such as interface segregation and dependency inversion,
  which ensure layered architecture and separation of concerns.
- **Linux Philosophy**: Small, single-purpose tools that can be combined to achieve complex tasks –
  This project is not meant to be a swiss army knife.

---

## License

[MIT](LICENSE)

---

## Acknowledgements

Thanks to the communities that created such awesome tools like i3, and to the developers
and contributors of the languages used in this project. Your work inspires and enables
projects like this to exist and thrive.
