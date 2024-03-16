# ecto

## Introduction

`ecto` is a trivial ruining of the familiar `echo` command. Have you ever
wanted to generate "ghostly" output on purpose or due to a mistype? Now you
can! For some reason! `ecto` takes a perfectly normal string as input and
returns it in the wavering, backwards-talking voice of a ghost. Spooky!

Anyway, the code's probably pretty bad since I'm relatively in experienced in
Rust and mostly wanted to see how quickly I could get this idea to work since,
no kidding, it's been bouncing around in my brain for more than a decade.
Maybe I'll spruce it up at some point. Maybe I won't. Who knows? (The Shadow
knows.)

Oh, and it's `ecto`as in "ectoplasm."

## Installation

For now at least you'll have to compile it yourself.

```sh
  $ git clone https://github.com/nausicaa-rose/ecto.git
  $ cd ecto/src
  $ rustc main.rs -o ecto -C opt-level=3
  $ mv ecto ~/bin # Or the directory of your choice
```

## Usage

To use `ecto` simply type `ecto` in your shell followed by whatever text
you want to mangle:

```sh
$ ecto Make my words spooky!
```

Which produces

> 👻 !         o         s         w                   M
>      y    o    p         d    o         m    e    a
>        k         s         r         y         k

That's it! That's all it does.

