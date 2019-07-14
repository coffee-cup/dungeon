import { Game, Vector } from "../../crate/pkg";
import { renderMap } from "./draw";
import { Context } from "./types";
import { keys } from "./utils";

const MAX_MAP_WIDTH = 2000;
const MAP_WIDTH = 40;
const MAP_HEIGHT = 30;

const documentWidth = Math.min(document.body.clientWidth, MAX_MAP_WIDTH);
const documentHeight = Math.min(document.body.clientHeight);

const tileSize = Math.floor(
  documentWidth / MAP_WIDTH < documentHeight / MAP_HEIGHT
    ? documentWidth / MAP_WIDTH
    : documentHeight / MAP_HEIGHT,
);

export const start = async (mod: typeof import("../../crate/pkg")) => {
  const setupCanvas = (canvas: HTMLCanvasElement) => {
    canvas.width = MAP_WIDTH * tileSize;
    canvas.height = MAP_HEIGHT * tileSize;
  };

  const canvas = document.getElementById("canvas") as HTMLCanvasElement | null;
  if (canvas == null) {
    alert("canvas does not exist!");
    return;
  }

  setupCanvas(canvas);
  const context = canvas.getContext("2d");

  if (context == null) {
    alert("2d context does not exist!");
    return;
  }

  const game = mod.Game.new(MAP_WIDTH, MAP_HEIGHT);

  const ctx: Context = {
    game,
    tileSize,
    ctx2d: context,
    mod,
  };

  renderMap(ctx);

  document.addEventListener("keydown", e => {
    const keysToDir = {
      [keys.up]: mod.Direction.N,
      [keys.down]: mod.Direction.S,
      [keys.right]: mod.Direction.E,
      [keys.left]: mod.Direction.W,
      [keys.k]: mod.Direction.N,
      [keys.j]: mod.Direction.S,
      [keys.l]: mod.Direction.E,
      [keys.h]: mod.Direction.W,
      [keys.b]: mod.Direction.SW,
      [keys.n]: mod.Direction.SE,
      [keys.y]: mod.Direction.NW,
      [keys.u]: mod.Direction.NE,
    };

    const dir = keysToDir[e.keyCode];
    if (dir != null) {
      game.move_player(dir);
      renderMap(ctx);
    }
  });
};
