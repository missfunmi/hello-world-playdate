# Hello World, Playdate (on Rust)

Simple starter game that uses the [crankstart](https://crates.io/crates/crankstart) crate to write games for the [Playdate](https://play.date) handheld gaming console. 

## To run the game on the Playdate simulator (no device required)

Run the following command from the project root directory:

```bash
crank run --release
```

## To run the game on the Playdate device

You obviously need a Playdate first. If you have one already, run the following command from the project root directory:

```bash
crank build --device --release
```

This creates a device-compatible binary in the [target](/target) directory that you can then [sideload onto your Playdate device](https://help.play.date/games/sideloading/).