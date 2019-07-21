import { Game, Vector } from "../../crate/pkg";
import { Context } from "./types";

export const xPos = (ctx: Context, col: number): number => col * ctx.tileSize;
export const yPos = (ctx: Context, row: number): number => row * ctx.tileSize;

export const posToIndex = (game: Game, row: number, col: number): number =>
  game.size.x * row + col;
