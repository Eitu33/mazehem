# Mazehem
Mazehem is a minimalist maze-solving multiplayer game written in [Rust](https://www.rust-lang.org/).


## Gameplay
The 4 players spawn in each corner of the maze and must reach the center first in order to win.

## Precompiled Binaries
Precompiled binaries are only available for Linux at the moment: https://github.com/Eitu33/mazehem/releases

## Screenshot
![image](https://user-images.githubusercontent.com/89928840/150605767-96515eb3-41b3-47cb-bc3c-f7a08279dda7.png)

## Building from Source

If needed, install the Rust toolchain:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Build:
```
cargo build
```

## Running

```
cargo run server
```
```
cargo run client ${server_addr}:${port}
```
