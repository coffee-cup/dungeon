import { Game } from "../../crate/pkg";

export interface Context {
  game: Game;
  ctx2d: CanvasRenderingContext2D;
  tileSize: number;
  mod: typeof import("../../crate/pkg");
}
