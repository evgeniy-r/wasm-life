# "Life" game in WebAssembly  

This is Conway's [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) written in Rust (and then compiled to `*.wasm`).

It is inspired by [this tutorial](https://rustwasm.github.io/book/game-of-life/implementing.html#implementing-conways-game-of-life), 
but I used my own implementation: I worked with a large field size (more than a million cells), used manipulations with
`ImageData.data` for rendering, implemented loading the starting position from a file, and also made a slightly different UI.

I made the implementation of game rules itself testable without a compilation to `wasm`. 

You can change a canvas size in the [index.js](www/index.js) and [index.html](www/index.html) files.

The starting position located in the [start-position.js](www/start-position.js) file.

Installation (I tested it on Linux Ubuntu and MS Windows):

* Install [Node.js and npm](https://nodejs.org).

* Install [rustup](https://rustup.rs/) toolchain.

* Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).

* Clone this repository and build a JS package:
```shell
git clone git@github.com:evgeniy-r/wasm-life.git
cd wasm-life
wasm-pack build
```

* Install npm packages and run `webpack-dev-server`:
```shell
cd www
npm install
npm run start
```

* Open [http://localhost:8080](http://localhost:8080) in a web browser.

You can run tests (they are very basic):
* the unit tests without a `wasm` compilation (only the implementation of the game rules):
```shell
cargo test
```
* the integration tests with the compilation (you can use different browsers):
```shell
wasm-pack test --firefox --headless
```
or if you want to see the test results in the browser:
```shell
wasm-pack test --firefox
```
