import "./index.scss";

const loadModule = (): Promise<typeof import("../../crate/pkg")> => {
  return import("../../crate/pkg");
};

loadModule().then(mod => {});

const TILE_SIZE = 32;

const renderMap = (
  canvas: HTMLCanvasElement,
  context: CanvasRenderingContext2D,
  map: number[][],
) => {
  const numRows = map.length;
  const numCols = map[0].length;

  context.clearRect(0, 0, canvas.width, canvas.height);

  context.fillStyle = "hotpink";
  for (let i = 0; i < numRows; i += 1) {
    for (let j = 0; j < numCols; j += 1) {
      if (map[i][j] === 1) {
        context.fillRect(j * TILE_SIZE, i * TILE_SIZE, TILE_SIZE, TILE_SIZE);
      }
    }
  }
};

const setupCanvas = (canvas: HTMLCanvasElement) => {
  canvas.width = document.body.clientWidth;
  canvas.height = document.body.clientHeight;
};

const start = (canvas: HTMLCanvasElement) => {
  setupCanvas(canvas);
  const context = canvas.getContext("2d");

  if (context == null) {
    alert("2d context does not exist!");
    return;
  }

  const map = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1],
    [1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
  ];

  renderMap(canvas, context, map);
};

const canvas = document.getElementById("canvas");
if (canvas == null) {
  alert("canvas does not exist!");
} else {
  start(canvas as HTMLCanvasElement);
}
