# Contribution

Contributions are welcome!
Please follow the guidelines below, and feel free to open issues or pull requests.
These are for the sake of consistency and as a reminder of the projects principles.

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

## Rules for Contributions

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

## Project Paradigms

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
