import { syntaxTree } from "@codemirror/language";
// Create a zero-width widget to replace the brace characters
import { Decoration, DecorationSet, ViewPlugin, WidgetType } from "@codemirror/view";

// Create a zero-width widget to replace the brace characters
class HiddenWidget extends WidgetType {
  toDOM() {
    const span = document.createElement("span");
    span.style.display = "none"; // collapse completely
    return span;
  }
  ignoreEvent() {
    return false;
  }
}

const hiddenWidget = new HiddenWidget();
const replaceDeco = Decoration.replace({ widget: hiddenWidget, inclusive: true });

// The plugin that builds replace decorations for VarStart/VarEnd
export const hideVariableBracesPlugin = ViewPlugin.fromClass(
  class {
    decorations: DecorationSet;
    constructor(view) {
      this.decorations = this.build(view);
    }
    update(update) {
      if (update.docChanged || update.viewportChanged || update.selectionSet) {
        this.decorations = this.build(update.view);
      }
    }
    build(view) {
      const builder: Range<Decoration>[] = [];
      const tree = syntaxTree(view.state);
      tree.iterate({
        from: view.viewport.from,
        to: view.viewport.to,
        enter: (node) => {
          if (node.name === "VarStart" || node.name === "VarEnd") {
            // Replace the token range with a zero-width widget
            builder.push(replaceDeco.range(node.from, node.to));
          }
        },
      });
      return Decoration.set(builder, true);
    }
  },
  {
    decorations: (v) => v.decorations,
  }
);
