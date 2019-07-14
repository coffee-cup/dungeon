import { Vector, Game } from "../../crate/pkg";
import { colours, chars } from "./styles";
import { keys } from "./utils";

const MAX_MAP_WIDTH = 5000;
const MAP_WIDTH = 30;
const MAP_HEIGHT = 20;

const documentWidth = Math.min(document.body.clientWidth, MAX_MAP_WIDTH);
const documentHeight = Math.min(document.body.clientHeight);

const tileSize = Math.floor(
  documentWidth / MAP_WIDTH < documentHeight / MAP_HEIGHT
    ? documentWidth / MAP_WIDTH
    : documentHeight / MAP_HEIGHT,
);

export const start = async (mod: typeof import("../../crate/pkg")) => {
  const EntityType = mod.EntityType;

  const xPos = (col: number): number => col * tileSize;
  const yPos = (row: number): number => row * tileSize;

  const posToIndex = (gameSize: Vector, row: number, col: number): number =>
    gameSize.x * row + col;

  const drawTileText = (
    context: CanvasRenderingContext2D,
    row: number,
    col: number,
    text: string,
    colour: string,
    fontScale: number,
  ) => {
    context.font = `${tileSize * fontScale}px Consolas`;
    context.fillStyle = colour;
    context.textAlign = "center";
    context.textBaseline = "middle";
    context.fillText(text, xPos(col) + tileSize / 2, yPos(row) + tileSize / 2);
  };

  const drawTileSquare = (
    context: CanvasRenderingContext2D,
    row: number,
    col: number,
    colour: string,
  ) => {
    context.fillStyle = colour;
    context.fillRect(xPos(col), yPos(row), tileSize, tileSize);
  };

  const fillTile = (
    context: CanvasRenderingContext2D,
    row: number,
    col: number,
    options: {
      backgroundColour: string;
      textColour: string;
      char: string;
      fontScale?: number;
    },
  ) => {
    drawTileSquare(context, row, col, options.backgroundColour);
    drawTileText(
      context,
      row,
      col,
      options.char,
      options.textColour,
      options.fontScale || 0.6,
    );
  };

  const renderMap = async (
    canvas: HTMLCanvasElement,
    context: CanvasRenderingContext2D,
    game: Game,
  ) => {
    const size = game.size;
    const map = await game.get_map();

    context.clearRect(0, 0, canvas.width, canvas.height);
    const numRows = size.y;
    const numCols = size.x;

    for (let row = 0; row < numRows; row += 1) {
      for (let col = 0; col < numCols; col += 1) {
        const index = posToIndex(size, row, col);
        const tile = map[index];

        if (tile === EntityType.Wall) {
          fillTile(context, row, col, {
            backgroundColour: colours.background.wall,
            textColour: colours.foreground.wall,
            char: chars.wall,
          });
        } else if (tile === EntityType.Floor) {
          fillTile(context, row, col, {
            backgroundColour: colours.background.floor,
            textColour: colours.foreground.floor,
            char: chars.floor,
          });
        } else if (tile === EntityType.Player) {
          fillTile(context, row, col, {
            backgroundColour: colours.background.player,
            textColour: colours.foreground.player,
            char: chars.player,
            fontScale: 0.8,
          });
        }
      }
    }
  };

  const setupCanvas = (canvas: HTMLCanvasElement) => {
    canvas.width = MAP_WIDTH * tileSize;
    canvas.height = MAP_HEIGHT * tileSize;
  };

  const game = mod.Game.new(MAP_WIDTH, MAP_HEIGHT);

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

  renderMap(canvas, context, game);

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
      renderMap(canvas, context, game);
    }
  });
};
