# Shulsaga
>  Shulsaga is an astral goddess in Sumerian religion


The goal of this project is to use the Rust language to implement a Research-Oriented rendering system. Compared with other Research-Oriented renderers, we pay more attention to software engineering and code simplicity instead of extreme performance optimization, so for those who want to enter the field of graphics, this may be a better platform.


## Why we start this project?
It is true that there are many implementations of rendering systems, and the more outstanding ones are:

- [mitsuba](https://www.mitsuba-renderer.org/)
- [PBRT](https://pbr-book.org/)
- [smallvcm](http://www.smallvcm.com/)

But they also have many problems. For example, `SmallVCM` is a project for beginners, but many of the technologies used in this project are a little outdated. 

On the other hand, `Mitsuba` and `PBRT` are relatively cutting-edge technologies, but they are developing In terms of software engineering, it was relatively unsuccessful at the time, and it seemed very academic.

So we are planning to use the Rust language to write this rendering system. The main reason is that the Rust language has excellent software engineering features and a very complete tool chain so that beginners can quickly install and run our code, and do it at the same time. The success in software engineering makes future development and maintenance more convenient.

## Target
We are going to write as portable as we can do.

We are ready to support the following platforms:

- `MacOS` our development platform
- `Windows` platforms we must support
- `Linux` platform we must support
- `FreeBSD` platform we must support
- `WASM` platforms we may consider supporting