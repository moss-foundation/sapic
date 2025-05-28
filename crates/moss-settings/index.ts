import { mkdirSync, writeFileSync } from "fs";
import { z } from "zod/v4";

const enum Level {
  Application = 1,
  User = 2,
  Workspace = 3,
}

const schema = (obj: Record<string, z.ZodSchema>): string => {
  return JSON.stringify(z.toJSONSchema(z.object(obj)), null, 2);
};

(async () => {
  try {
    mkdirSync("./gen/schemas", { recursive: true });

    await Promise.all([
      writeFileSync(
        `./gen/schemas/default.schema.json`,
        schema({
          "workbench.colorTheme": z
            .enum(["light", "dark"])
            .describe("The color theme to use.")
            .optional()
            .meta({
              level: Level.User,
              enumItemProperties: [
                {
                  order: 1,
                  description: "The light color theme to use.",
                  mode: "light",
                  displayName: "Light Default",
                },
                {
                  order: 2,
                  description: "The dark color theme to use.",
                  mode: "dark",
                  displayName: "Dark Default",
                },
              ],
            }),
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
        }),
        {
          flag: "w",
        }
      ),
    ]);
  } catch (err) {
    console.error("Error generating schema:", err);
    process.exit(1);
  }
})();
