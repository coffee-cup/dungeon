import { start } from "./game";

import "./index.scss";

const loadModule = (): Promise<typeof import("../../crate/pkg")> => {
  return import("../../crate/pkg");
};

loadModule().then(mod => {
  start(mod);
});
