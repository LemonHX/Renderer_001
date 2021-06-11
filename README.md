# Shulsaga
>  Shulsaga is an astral goddess in Sumerian religion

![platform: Linux | Windows | macOS](https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-blue) [![CI](https://github.com/shulsaga/shulsaga/actions/workflows/main.yml/badge.svg)](https://github.com/shulsaga/shulsaga/actions/workflows/main.yml)

The goal of this project is to use the Rust language to implement a Research-Oriented rendering system. Compared with other Research-Oriented renderers, we pay more attention to software engineering and code simplicity instead of extreme performance optimization, so for those who want to enter the field of graphics, this may be a better platform.

## Motivation
> Modern rendering system with proper software design

There are many successful rendering systems out there, the most notable one being:

- [mitsuba](https://www.mitsuba-renderer.org/)
- [PBRT](https://pbr-book.org/)

However, `Mitsuba 0.6` suffers from poor dependency selection and `PBRT v4` follows problematic software design methodologies.

The Rust language offers excellent software engineering features and a complete toolchain which makes future development and maintenance more convenient.

## Target
> As portable as possible

We are ready to support the following platforms:

- `MacOS` our development platform
- `Windows` platforms we must support
- `Linux` platform we must support
- `FreeBSD` platform we must support
- `WASM` platforms we may consider supporting

## Project structure
- `src/` directory contains all the source files
- `src/core` the renderer core, basically anything related to how the image being rendered.
- `src/utils` containing all the utility code used by other utility folders in `src/`
- other dirs in `src/` are featured utility used by core.
- in each folder in `src/` has a `tests/` folder for containing all the tests, and using the same file name as the file should be tested.
- each test function should be easy to identify (or looks like) the original function or part of the function.


for example:
```rs
// src/geometry/ray.rs
fn point_at();
// src/geometry/tests/ray.rs
fn test_point_at();
```