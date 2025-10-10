import { writeFile } from "fs/promises";

import { defaultDarkTheme } from "./themes/dark";
import { defaultLightTheme } from "./themes/light";
import { defaultVSCodeTheme } from "./themes/vscode";

(async () => {
  try {
    await Promise.all([
      writeFile(`themes/light-default.json`, JSON.stringify(defaultLightTheme, null, 2), { flag: "w" }),
      writeFile(`themes/dark-default.json`, JSON.stringify(defaultDarkTheme, null, 2), { flag: "w" }),
      writeFile(`themes/vscode.json`, JSON.stringify(defaultVSCodeTheme, null, 2), { flag: "w" }),
    ]);
  } catch (err) {
    console.error("Error generating themes:", err);
    process.exit(1);
  }
})();
