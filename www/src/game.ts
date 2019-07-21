import { Game, Vector } from "../../crate/pkg";
import { render } from "./draw";
import { Context } from "./types";
import { keys } from "./keys";
import * as WebFont from "webfontloader";

const MAX_MAP_WIDTH = 2000;

// const map = `
// #######
// # #   #
// # # # #
// # #   #
// # #   #
// # #   #
// #     #
// #######
// `.trim();
const map = `
######################################################
#       ##############################################
#            ###               ######    ##          #
#       #### ###    ########## ####      ## ######## #
#       #### ###### ########## #### #    ## ######## #
#       #### ###### ###          ## #    ## ##   ### #
#       ####    ### ###  #    #  ## ## #### ## # ### #
## ######### ## ### ###  #    #  ## #    ##    # ### #
#     ###### ## ### ###  #    #  ## #    ####### ### #
#     ###### ## ### ###  #    #  ## #    ##       ## #
############ ## ### ###          ## ### ###       ## #
############ ##     ######### ##### #    ##       ## #
#   #        ################ ##### #             ## #
#     #############        ##       ################ #
#   # #############  ####  ######################### #
##### #########      #  #       ######          #### #
#   # ######### ###        #### #####            ### #
#     ######### ############### ####    #         ## #
#   #           ######          ###                # #
###### ### ### ####################       ##         #
#   #   #   #   ##      ##    #####                ###
#       #   #   ## #### ##    ######         #    ####
#   #   #   #      #### ##    #######            #####
#######################       ########          ######
######################################################
`.trim();

const lines = map.split("\n");

const MAP_WIDTH = 60;
const MAP_HEIGHT = 40;
// const MAP_WIDTH = lines[0].length;
// const MAP_HEIGHT = lines.length;

console.log({ MAP_WIDTH, MAP_HEIGHT });

const documentWidth = Math.min(document.body.clientWidth, MAX_MAP_WIDTH);
const documentHeight = Math.min(document.body.clientHeight);

const tileSize = Math.floor(
  documentWidth / MAP_WIDTH < documentHeight / MAP_HEIGHT
    ? documentWidth / MAP_WIDTH
    : documentHeight / MAP_HEIGHT,
);

const loadFont = (cb: () => any) => {
  WebFont.load({
    google: {
      families: ["Cutive Mono"],
    },
    active: cb,
    inactive: cb,
  });
};

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

  const reset = (): Context => {
    const game = mod.Game.new(MAP_WIDTH, MAP_HEIGHT, map.replace(/\n/g, ""));
    const ctx: Context = {
      game,
      tileSize,
      ctx2d: context,
      mod,
    };

    render(ctx);

    return ctx;
  };

  let ctx = reset();

  loadFont(() => render(ctx));

  document.addEventListener("keydown", e => {
    if (e.keyCode === keys.m) {
      ctx = reset();
    }

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
      ctx.game.move_player(dir);
      render(ctx);
    }
  });
};
