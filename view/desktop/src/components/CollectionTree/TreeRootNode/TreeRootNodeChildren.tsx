import { useContext } from "react";

import { Tree } from "@/lib/ui/Tree";

import { EntryIcon } from "../../EntryIcon";
import { CollectionTreeContext } from "../CollectionTreeContext";
import TreeNode from "../TreeNode/TreeNode";
import { TreeCollectionRootNode } from "../types";
import { getChildrenNames } from "../utils";

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
  const { nodeOffset, displayMode } = useContext(CollectionTreeContext);

  const shouldRenderAddRootForm = isAddingRootFileNode || isAddingRootFolderNode;
  const restrictedNames = getChildrenNames(node);

  return (
    <Tree.RootNodeChildren>
      {node.childNodes.map((childNode, index) => {
        return (
          <TreeNode
            parentNode={childNode}
            key={childNode.id}
            node={childNode}
            depth={1}
            isLastChild={index === node.childNodes.length - 1}
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
          <Tree.NodeAddForm
            onSubmit={handleAddFormRootSubmit}
            onCancel={handleAddFormRootCancel}
            restrictedNames={restrictedNames}
          />
        </div>
      )}
    </Tree.RootNodeChildren>
  );
};
