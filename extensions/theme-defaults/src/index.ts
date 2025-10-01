import { writeFile } from "fs/promises";
import { defaultDarkTheme } from "./themes/dark";
import { defaultLightTheme } from "./themes/light";
import { defaultVSCodeTheme } from "./themes/vscode";

(async () => {
  try {
    await Promise.all([
      writeFile(`light.json`, JSON.stringify(defaultLightTheme, null, 2), { flag: "w" }),
      writeFile(`dark.json`, JSON.stringify(defaultDarkTheme, null, 2), { flag: "w" }),
      writeFile(`vscode.json`, JSON.stringify(defaultVSCodeTheme, null, 2), { flag: "w" }),
    ]);
  } catch (err) {
    console.error("Error generating themes:", err);
    process.exit(1);
  }
})();
