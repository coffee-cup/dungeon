import { Vector } from "../../crate/pkg";
import {
  floorStyle,
  font,
  playerStyle,
  StyleOptions,
  wallStyle,
} from "./styles";
import { Context } from "./types";
import { posToIndex, xPos, yPos } from "./utils";

const drawTileText = (
  ctx: Context,
  pos: Vector,
  { fontScale, textColour, text }: StyleOptions,
) => {
  const { ctx2d, tileSize } = ctx;

  ctx2d.font = `${Math.floor(tileSize * fontScale)}px ${font}`;
  ctx2d.fillStyle = textColour;
  ctx2d.textAlign = "center";
  ctx2d.textBaseline = "middle";
  ctx2d.fillText(
    text,
    xPos(ctx, pos.x) + tileSize / 2,
    yPos(ctx, pos.y) + tileSize / 2,
  );
};

const drawTileSquare = (
  ctx: Context,
  pos: Vector,
  { bgColour }: StyleOptions,
) => {
  ctx.ctx2d.fillStyle = bgColour;
  ctx.ctx2d.fillRect(
    xPos(ctx, pos.x),
    yPos(ctx, pos.y),
    ctx.tileSize,
    ctx.tileSize,
  );
};

export const fillTile = (ctx: Context, pos: Vector, style: StyleOptions) => {
  drawTileSquare(ctx, pos, style);
  drawTileText(ctx, pos, style);
};

export const renderMap = async (ctx: Context) => {
  const { ctx2d, tileSize, game } = ctx;
  const EntityType = ctx.mod.EntityType;

  const size = game.size;
  const map = await game.get_map();

  ctx2d.clearRect(0, 0, game.size.x * tileSize, game.size.y * tileSize);
  const numRows = size.y;
  const numCols = size.x;

  const entityToStyle = {
    [EntityType.Wall]: wallStyle,
    [EntityType.Floor]: floorStyle,
    [EntityType.Player]: playerStyle,
  };

  for (let row = 0; row < numRows; row += 1) {
    for (let col = 0; col < numCols; col += 1) {
      const index = posToIndex(game, row, col);
      const tile = map[index];

      const pos = new ctx.mod.Vector(col, row);
      const style = entityToStyle[tile];

      if (!style) {
        if (tile !== EntityType.Hidden) {
          console.log("Unknown tile", tile === EntityType.Hidden);
        }
        continue;
      }

      if (tile === EntityType.Wall) {
        const style = {
          text: `${col},${row}`,
          textColour: "black",
          bgColour: wallStyle.bgColour,
          fontScale: 0.2,
        };
        fillTile(ctx, pos, style);
      } else if (tile === EntityType.Floor) {
        const style = {
          text: `${col},${row}`,
          textColour: "white",
          bgColour: floorStyle.bgColour,
          fontScale: 0.2,
        };
        fillTile(ctx, pos, style);
      } else {
        fillTile(ctx, pos, style);
      }
    }
  }
};
