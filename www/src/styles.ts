export type Colour = string;

export const font = "Cutive Mono";

export interface TextOptions {
  text: string;
  textColour: Colour;
  fontScale: number;
}

export interface TileOptions {
  bgColour: Colour;
}

export const defaultTextOptions: TextOptions = {
  text: "",
  textColour: "red",
  fontScale: 0.7,
};

export const defaultTileOptions: TileOptions = {
  bgColour: "red",
};

export type StyleOptions = TextOptions & TileOptions;

const createStyle = (style: Partial<StyleOptions>): StyleOptions => ({
  ...defaultTextOptions,
  ...defaultTileOptions,
  ...style,
});

export const wallStyle = createStyle({
  text: "",
  textColour: "black",
  bgColour: "#724cf9",
});

export const floorStyle = createStyle({
  text: ".",
  textColour: "rgba(255, 255, 255, 0.5)",
  bgColour: "black",
});

export const playerStyle = createStyle({
  text: "@",
  textColour: "#fcf943",
  bgColour: floorStyle.bgColour,
  fontScale: 0.9,
});

export const memoryStyle = createStyle({
  bgColour: "rgba(0, 0, 0, 0.7)",
});
