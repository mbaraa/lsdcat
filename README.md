# Wut?

![](https://github.com/mbaraa/lsdcat/raw/main/assets/nom.jpg)

_image "borrowed" from [lolcat](https://github.com/busyloop/lolcat/blob/master/ass/nom.jpg)_

# Screenshot

![](https://github.com/mbaraa/lsdcat/raw/main/assets/screenshot.png)

# Usage:

## Dependencies

- rust (duh)
- cargo (duh2)

## Install the thing

```bash
; cargo install lsdcat
```

_OR_

Install my [Gentoo Overlay](https://github.com/mbaraa/gentoo-overlay)

```bash
; sudo emerge -qav lsdcat
```

_OR_

- Clone the repo

```bash
; git clone https://github.com/mbaraa/lsdcat
```

- Run it using cargo (might take some time compiling)

```bash
; echo "What you guys are referring to as Linux, is in fact, GNU/Linux, or as I've recently taken to calling it, GNU plus Linux. Linux is not an operating system unto itself, but rather another free component of a fully functioning GNU system made useful by the GNU corelibs, shell utilities and vital system components comprising a full OS as defined by POSIX. Many computer users run a modified version of the GNU system every day, without realizing it. Through a peculiar turn of events, the version of GNU which is widely used today is often called \"Linux\", and many of its users are not aware that it is basically the GNU system, developed by the GNU Project. There really is a Linux, and these people are using it, but it is just a part of the system they use. Linux is the kernel: the program in the system that allocates the machine's resources to the other programs that you run. The kernel is an essential part of an operating system, but useless by itself; it can only function in the context of a complete operating system. Linux is normally used in combination with the GNU operating system: the whole system is basically GNU with Linux added, or GNU/Linux. All the so-called \"Linux\" distributions are really distributions of GNU/Linux."
    | cargo run
```

# More usage:

```bash
; lsdcat --freq 0.1 --spread 3.0 --bg 0

    --freq: defaults to 0.1, and controls the frequency of the color change.
    --spread: defaults to 3.0, and controls how fast the color spread across characters.
    --bg: default to 0, and sets the color to background instead of foreground when set to 1.

Examples:
    ; echo "the rainbow, I tastes it!" | lsdcat --freq 4.2 --spread 6.9
    ; echo "the rainbow, I tastes it!" | lsdcat --freq 4.2 --spread 6.9 --bg 1
    ; cat ./README.md | lsdcat --freq 4.2 --spread 6.9
```
