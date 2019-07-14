# Dungeon

Small
[roguelike](http://www.roguebasin.com/index.php?title=Roguelike_Dev_FAQ#What_is_a_roguelike.3F)
dungeon crawler I am having fun writing. This is an experiement for me to
understand the implementation of roguelike mechanics such as dungeon generation,
field of view, and pathfinding. I doubt the game will actually be fun.

The game logic is written in [Rust](https://www.rust-lang.org/) (compiled to
[WebAssembly](https://webassembly.org/)), and rendered to a HTML5 canvas with
[TypeScript](https://www.typescriptlang.org/).

## Getting Started

[Webpack](https://webpack.js.org/) bundles the TypeScript and Rust with the
[WasmPackPlugin](https://github.com/wasm-tool/wasm-pack-plugin) so it is
playable on the web.

After cloning

``` shell
cd www
yarn start
```

Navigate to [localhost:8080](http://localhost:8080)

