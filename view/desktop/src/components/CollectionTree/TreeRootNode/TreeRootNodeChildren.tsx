import { useContext } from "react";

import { cn } from "@/utils";

import { NodeAddForm } from "../NodeAddForm";
import { TestCollectionIcon } from "../TestCollectionIcon";
import { TreeContext } from "../Tree";
import TreeNode from "../TreeNode/TreeNode";
import { TreeCollectionNode, TreeCollectionRootNode } from "../types";
import { calculateRestrictedNames } from "./utils";

interface TreeRootNodeChildrenProps {
  node: TreeCollectionRootNode;
  onNodeUpdate: (node: TreeCollectionNode) => void;
  isAddingRootFileNode: boolean;
  isAddingRootFolderNode: boolean;
  handleAddFormRootSubmit: (name: string) => void;
  handleAddFormRootCancel: () => void;
}

export const TreeRootNodeChildren = ({
  node,
  onNodeUpdate,
  isAddingRootFileNode,
  isAddingRootFolderNode,
  handleAddFormRootSubmit,
  handleAddFormRootCancel,
}: TreeRootNodeChildrenProps) => {
  const { nodeOffset, displayMode } = useContext(TreeContext);

  const nodesToRender =
    displayMode === "REQUEST_FIRST"
      ? node.requests.childNodes
      : [node.endpoints, node.schemas, node.components, node.requests]; //TODO: this should check it's root nodes orders

  const shouldRenderAddRootForm = displayMode === "REQUEST_FIRST" && (isAddingRootFileNode || isAddingRootFolderNode);
  const restrictedNames = calculateRestrictedNames(node, isAddingRootFolderNode);

  return (
    <ul className={cn("h-full w-full", { "pb-2": nodesToRender.length > 0 })}>
      {nodesToRender.map((childNode, index) => {
        return (
          <TreeNode
            onNodeUpdate={onNodeUpdate}
            key={childNode.id}
            node={childNode}
            depth={1}
            isLastChild={index === nodesToRender.length - 1}
          />
        );
      })}
      {shouldRenderAddRootForm && (
        <div className="flex w-full min-w-0 items-center gap-1 py-0.5" style={{ paddingLeft: nodeOffset * 1 }}>
          <TestCollectionIcon type={"Dir"} className="opacity-0" />
          <TestCollectionIcon type={"Dir"} className={cn({ "opacity-0": isAddingRootFileNode })} />
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
