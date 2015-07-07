Rocket
======

> Rocket is a toy game written in Rust, using the Piston library. The code is thoroughly commented in order to help people to follow it easily.

## Screenshots

![Screenshot](screenshots/gameplay2.png)

You can find more screenshots in the `screenshots` directory

## How to play

As you can see in the screenshots below, you are the red rocket and have to save the world from the yellow invaders. To do so, you can use the following controls:

Keyboard | Action
-------- | ------------
Up       | Boost
Left     | Rotate left
Right    | Rotate right
Space    | Shoot

## Running it with Cargo

As always, it is a real pleasure to work with Cargo. You only need the following:

```
cargo run --release
```

**Caveat**: the version of Piston used in this project depends on the FreeType library, which must be installed in your system in order to compile the game. You can install it by following the instructions given in [Piston Tutorials/getting-started project](https://github.com/bvssvni/Piston-Tutorials/tree/4772bfa970247cd0da80e92c582898a7a9a3218c/getting-started#freetype-on-os-x)

## Why?

After having implemented some toy games in C++ using SDL and SFML, I thought it would be a good idea to try the same in Rust. Additionally, I had written a similar game in Haskell and wanted to port it to see the similarities and differences between Haskell and Rust. Another reason to program this game was to have an easy to follow Rust project that could be useful for people learning the language.

## License

MIT
