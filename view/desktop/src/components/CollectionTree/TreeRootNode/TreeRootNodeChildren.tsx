import { useContext } from "react";

import { cn } from "@/utils";

import { EntryIcon } from "../../EntryIcon";
import { NodeAddForm } from "../NodeAddForm";
import { TreeContext } from "../Tree";
import TreeNode from "../TreeNode/TreeNode";
import { TreeCollectionRootNode } from "../types";
import { getChildrenNames, sortByOrder } from "../utils";

interface TreeRootNodeChildrenProps {
  node: TreeCollectionRootNode;
  isAddingRootFileNode: boolean;
  isAddingRootFolderNode: boolean;
  handleAddFormRootSubmit: (name: string) => void;
  handleAddFormRootCancel: () => void;
}

export const TreeRootNodeChildren = ({
  node,
  isAddingRootFileNode,
  isAddingRootFolderNode,
  handleAddFormRootSubmit,
  handleAddFormRootCancel,
}: TreeRootNodeChildrenProps) => {
  const { nodeOffset, displayMode } = useContext(TreeContext);

  const nodesToRender =
    displayMode === "REQUEST_FIRST"
      ? sortByOrder(node.requests.childNodes)
      : sortByOrder([node.endpoints, node.schemas, node.components, node.requests]);

  const shouldRenderAddRootForm = displayMode === "REQUEST_FIRST" && (isAddingRootFileNode || isAddingRootFolderNode);
  const restrictedNames = getChildrenNames(node.requests);

  return (
    <ul className={cn("h-full w-full")}>
      {nodesToRender.map((childNode, index) => {
        return (
          <TreeNode
            parentNode={displayMode === "REQUEST_FIRST" ? node.requests : childNode}
            key={childNode.id}
            node={childNode}
            depth={1}
            isLastChild={index === nodesToRender.length - 1}
            isRootNode={displayMode === "DESIGN_FIRST"}
          />
        );
      })}

      {shouldRenderAddRootForm && (
        <div className="flex w-full min-w-0 items-center gap-1 py-0.5" style={{ paddingLeft: nodeOffset * 1 }}>
          <EntryIcon
            entry={{
              id: "Placeholder_AddingNodeId",
              name: "Placeholder_AddingNodeName",
              kind: "Dir",
              protocol: undefined,
              expanded: true,
              childNodes: [],
              path: {
                raw: "",
                segments: [],
              },
              class: "Request",
            }}
            className="opacity-0"
          />
          <NodeAddForm
            onSubmit={handleAddFormRootSubmit}
            onCancel={handleAddFormRootCancel}
            restrictedNames={restrictedNames}
          />
        </div>
      )}
    </ul>
  );
};
