import { useContext } from "react";

import { cn } from "@/utils";

import { TreeContext } from "../Tree";
import TreeNode from "../TreeNode/TreeNode";
import { TreeCollectionNode, TreeCollectionRootNode } from "../types";

interface TreeRootNodeChildrenProps {
  node: TreeCollectionRootNode;
  onNodeUpdate: (node: TreeCollectionNode) => void;
  isAddingRootFileNode: boolean;
  isAddingRootFolderNode: boolean;
  handleAddFormRootSubmit: (newNode: TreeCollectionRootNode) => void;
  handleAddFormRootCancel: () => void;
}

export const TreeRootNodeChildren = ({
  node,
  onNodeUpdate,
  // isAddingRootFileNode,
  // isAddingRootFolderNode,
  // handleAddFormRootSubmit,
  // handleAddFormRootCancel,
}: TreeRootNodeChildrenProps) => {
  const { nodeOffset, onRootAddCallback, displayMode } = useContext(TreeContext);

  const nodesToRender =
    displayMode === "RequestFirst"
      ? node.Requests.childNodes
      : [node.Requests, node.Schemas, node.Components, node.Endpoints];

  return (
    <ul className={cn("h-full w-full", { "pb-2": nodesToRender.length > 0 })}>
      {nodesToRender.map((childNode, index) => (
        <TreeNode
          parentNode={node}
          onNodeUpdate={onNodeUpdate}
          key={childNode.uniqueId}
          node={childNode}
          depth={1}
          isLastChild={index === nodesToRender.length - 1}
        />
      ))}
      {/* {(isAddingRootFileNode || isAddingRootFolderNode) && (
        <div className="flex w-full min-w-0 items-center gap-1 py-0.5" style={{ paddingLeft: nodeOffset * 1 }}>
          <TestCollectionIcon type={node.type} className="opacity-0" />
          <TestCollectionIcon type={node.type} className={cn({ "opacity-0": isAddingRootFileNode })} />
          <NodeAddForm
            isFolder={isAddingRootFolderNode}
            restrictedNames={nodesToRender.map((childNode) => childNode.id)}
            onSubmit={(newNode) => {
              handleAddFormRootSubmit(newNode);
              onRootAddCallback?.({ ...node, childNodes: [...nodesToRender, newNode] } as TreeNodeProps);
            }}
            onCancel={handleAddFormRootCancel}
          />
        </div>
      )} */}
    </ul>
  );
};
