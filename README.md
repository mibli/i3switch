# i3switch - Intuitive i3 Window Switching Utility

We pick tiled window managers to improve our productivity. They allow us to lay out the workspace
in a way that we can see all the necessary information at a glance. This is the superpower of
tiled window managers. And the switching mechanism should follow this mindset. We want to switch
quickly to our layed out windows quickly, intuitively and efficiently.

This is what i3switch is all about. It allows You to switch to what You see, without the necessary
abstraction of the manual tiling structure.

For more history and motivation, see [history.md](docs/history.md).

## Preview

[![Preview Video](https://raw.githubusercontent.com/mibli/i3switch/main/docs/media/preview.gif)](https://raw.githubusercontent.com/mibli/i3switch/main/docs/media/preview.mp4)

## Features

* **Directional Switching**: Switch to the next window VISIBLE window in the specified direction.
* **Tab Navigation**: Switch to the next window in the current tabbed container.
* **Tab Number Switching**: Switch to the specified tab number in the current tabbed container.
* **Floating Switching**: Switch between floating windows in the direction or windows-like tab
  navigation.
* **Multi-Monitor Support**: Switch windows across multiple monitors, respecting their layout.

## Building & Running

Each implementation has it's own Makefile, default target will build a release binary.

```bash
# Build one language release binary
make -C python
make -C cpp dist/i3switch
make cpp/dist/i3switch

# Build all release binaries
make all

# Build release binaries and packages
# Note: For dependability version must match the current language version git tag.
make i3switch_1.0.0-ubuntu_amd64.deb

# Build changelog
make rust/dist/CHANGELOG.md
```

**Proper binary versioning require .git repository to be present and tags to be fetched.**

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

## Getting Started

See each subdirectory for build/run instructions:

- [python/README.md](python/README.md)
- [cpp/README.md](cpp/README.md)
- [rust/README.md](rust/README.md)

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

## Branches

- **Main Branch**: `main` (contains the latest stable code)
- **Development Branches**: start with intent  (feature/, bugfix/, etc.) and describe the change
  - **Example**: `feature/add-new-switching-mode`, `bugfix/fix-memory-leak`

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
