# i3switch Project History

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
