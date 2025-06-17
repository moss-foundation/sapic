/**
 * TypeDoc plugin to remove @repo/ prefix from package names
 */

function load(app) {
  app.converter.on("resolveReflection", (context, reflection) => {
    if (reflection.name && reflection.name.startsWith("@repo/")) {
      reflection.name = reflection.name.replace("@repo/", "");
    }
  });

  app.renderer.on("beginRender", (event) => {
    const project = event.project;

    function renameChildren(reflection) {
      if (reflection.name && reflection.name.startsWith("@repo/")) {
        reflection.name = reflection.name.replace("@repo/", "");
      }

      if (reflection.children) {
        reflection.children.forEach(renameChildren);
      }
    }

    renameChildren(project);
  });
}

module.exports = { load };
