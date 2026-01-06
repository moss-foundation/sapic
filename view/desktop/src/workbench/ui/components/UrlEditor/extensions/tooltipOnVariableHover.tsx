import { createRoot } from "react-dom/client";

import { EndpointViewContextProps } from "@/workbench/views/EndpointView/EndpointViewContext";
import { syntaxTree } from "@codemirror/language";
import { hoverTooltip } from "@codemirror/view";

import { VariableReactTooltip } from "../components/VariableReactTooltip";

// Hover tooltip extension for variable names (using lezer parser)
export const tooltipOnVariableHover = (ctx: EndpointViewContextProps) => {
  return hoverTooltip(
    (view, pos) => {
      const tree = syntaxTree(view.state);
      const node = tree.resolve(pos, -1);

      // Walk up the tree to find an Identifier node (which maps to variableName tag)
      let currentNode: typeof node | null = node;
      while (currentNode) {
        const nodeStart = currentNode.from;
        const nodeEnd = currentNode.to;

        // Check if the node type is "Identifier" (which is tagged as variableName in the grammar)
        // and if the cursor position is within this node
        if (currentNode.type.name === "Identifier" && pos >= nodeStart && pos <= nodeEnd) {
          const nodeText = view.state.doc.sliceString(nodeStart, nodeEnd);
          const tooltipContainer = document.createElement("div");

          const root = createRoot(tooltipContainer);
          root.render(<VariableReactTooltip variableName={nodeText} ctx={ctx} />);

          return {
            pos: nodeStart,
            end: nodeEnd,
            create: () => {
              return {
                dom: tooltipContainer,
                destroy: () => {
                  root.unmount();
                },
              };
            },
          };
        }

        currentNode = currentNode.parent;
      }

      return null;
    },
    {
      hoverTime: 0,
    }
  );
};
