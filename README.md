<h1 align=center>Naviga</h1>

<div align="center">
  <img width="600"
       src="https://user-images.githubusercontent.com/70762494/188735570-3e066397-5635-4438-ab88-bf17b2473543.png">
</div>

## About

Naviga helps users navigate easily through directories. Naviga's design is inspired from <a href="https://github.com/ranger/ranger">Ranger</a>.

## Installation

```shell
$ cargo install naviga
```

A program can't change the current directory as a process can only affect its own environment. A shell function must be added in your shell configuration file (.bashrc, .zshrc) the terminal must be restarted.

> I agree, a shell function is not the smartest way.

```
function nav() {
    naviga

    if [ -f "$HOME/naviga.sh" ]; then
        . ~/naviga.sh
        rm ~/naviga.sh
    fi
}

export function nav
```

## Quick start

Launch **naviga** with **nav** (the shell function defined above). Use the arrow keys or `h` `i` `j` `k` to navigate, `Enter` to jump into the selected directory, and `q` to exit.

## Uninstallation

```shell
$ cargo uninstall naviga
```
