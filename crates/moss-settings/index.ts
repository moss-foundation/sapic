import { mkdirSync, writeFileSync } from "fs";
import { z } from "zod/v4";

const enum Level {
  Application = 1,
  User = 2,
  Workspace = 3,
}

const schema = z.object({
  "window.defaultWidth": z
    .number()
    .min(800)
    .max(3840)
    .default(800)
    .optional()
    .meta({
      level: Level.Application,
    })
    .describe("The width of the application window in pixels."),

  "window.defaultHeight": z
    .number()
    .min(600)
    .max(2160)
    .default(600)
    .optional()
    .meta({
      level: Level.Application,
    })
    .describe("The height of the application window in pixels."),
});

(async () => {
  try {
    mkdirSync("./gen/schemas", { recursive: true });

    await Promise.all([
      writeFileSync(`./gen/schemas/default.schema.json`, JSON.stringify(z.toJSONSchema(schema), null, 2), {
        flag: "w",
      }),
    ]);
  } catch (err) {
    console.error("Error generating themes:", err);
    process.exit(1);
  }
})();
