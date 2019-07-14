import { Game, Vector } from "../../crate/pkg";
import { Context } from "./types";

export const keys = {
  j: 74,
  k: 75,
  l: 76,
  h: 72,
  u: 85,
  y: 89,
  b: 66,
  n: 78,
  up: 38,
  right: 39,
  down: 40,
  left: 37,
};

export const xPos = (ctx: Context, col: number): number => col * ctx.tileSize;
export const yPos = (ctx: Context, row: number): number => row * ctx.tileSize;

export const posToIndex = (game: Game, row: number, col: number): number =>
  game.size.x * row + col;
