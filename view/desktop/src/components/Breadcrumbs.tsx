import { useMemo, useState } from "react";

import { useCollectionsStore } from "@/store/collections";
import { useDockviewStore } from "@/store/Dockview";
import { cn } from "@/utils";

import { DropdownMenu } from ".";
import Icon from "./Icon";
import NodeLabel from "./Tree/NodeLabel";
import { TestCollectionIcon } from "./Tree/TestCollectionIcon";
import { NodeProps, TreeNodeProps } from "./Tree/types";
import { findNodeById, prepareCollectionForTree, updateTreeNode } from "./Tree/utils";

export const Breadcrumbs = () => {
  const { currentActivePanelId } = useDockviewStore();
  const { collections } = useCollectionsStore();
  const [activeTree, setActiveTree] = useState<NodeProps | null>(null);

  const path = useMemo(() => {
    if (!currentActivePanelId) return [];

    const target = String(currentActivePanelId);
    for (const collection of collections) {
      const newPath = findPath(collection.tree, target);
      if (newPath) {
        setActiveTree(collection.tree);
        return newPath;
      }
    }

    setActiveTree(null);
    return [];
  }, [collections, currentActivePanelId]);

  return (
    <div className="flex items-center gap-1 px-3 py-[5px] text-[#6F6F6F]">
      {path.map((pathNode, index) => {
        if (!activeTree) return null;

        const node = findNodeById(activeTree, pathNode)!;

        return (
          <div key={pathNode} className="flex items-center">
            <DropdownMenu.Root>
              <DropdownMenu.Trigger className="cursor-pointer hover:underline">{pathNode}</DropdownMenu.Trigger>
              <DropdownMenu.Content>
                <BreadcrumbTree tree={node} onNodeClickCallback={() => {}} />
              </DropdownMenu.Content>
            </DropdownMenu.Root>

            {index !== path.length - 1 && (
              <span>
                <Icon icon="TreeChevronRightIcon" />
              </span>
            )}
          </div>
        );
      })}
    </div>
  );
};

const findPath = (node: NodeProps, target: string): string[] | null => {
  if (node.id === target) return [node.id];

  if (node.childNodes && node.childNodes.length > 0) {
    for (const child of node.childNodes) {
      const path = findPath(child, target);
      if (path) return [node.id.toString(), ...path];
    }
  }

  return null;
};

export default Breadcrumbs;

const BreadcrumbTree = ({
  tree: initialTree,
  onNodeClickCallback,
}: {
  tree: NodeProps;
  onNodeClickCallback: (node: NodeProps) => void;
}) => {
  const [tree, setTree] = useState<TreeNodeProps>(prepareCollectionForTree(initialTree, false));

  const handleNodeUpdate = (updatedNode: TreeNodeProps) => {
    setTree(updateTreeNode(tree, updatedNode));

    onNodeClickCallback?.(updatedNode);
  };

  return (
    <ul>
      {tree.childNodes.map((childNode) => (
        <BreadcrumbNode node={childNode} onNodeUpdate={handleNodeUpdate} onNodeClickCallback={onNodeClickCallback} />
      ))}
    </ul>
  );
};

const BreadcrumbNode = ({
  node,
  onNodeUpdate,
  depth = 0,
  onNodeClickCallback,
  paddingLeft = 12,
  paddingRight = 8,
  nodeOffset = 12,
}: {
  node: TreeNodeProps;
  onNodeUpdate: (node: TreeNodeProps) => void;
  depth?: number;
  onNodeClickCallback: (node: TreeNodeProps) => void;
  paddingLeft?: number;
  paddingRight?: number;
  nodeOffset?: number;
}) => {
  const { addPanel } = useDockviewStore();

  const nodePaddingLeft = useMemo(() => depth * nodeOffset + paddingLeft + 4, [depth, nodeOffset, paddingLeft]);
  const shouldRenderChildNodes = node.isFolder && node.isExpanded;

  const handleFolderClick = () => {
    if (!node.isFolder) return;
    onNodeUpdate({
      ...node,
      isExpanded: !node.isExpanded,
    });
  };

  return (
    <li>
      <button
        style={{ paddingLeft: nodePaddingLeft, paddingRight: paddingRight + 3 }}
        onClick={() => {
          if (node.isFolder) handleFolderClick();
          else addPanel({ id: `${node.id}` });

          onNodeClickCallback?.(node);
        }}
        className="py-0.5background-(--moss-treeNode-bg) hover:background-(--moss-treeNode-bg-hover) focus-within:background-(--moss-treeNode-bg) relative flex w-full min-w-64 cursor-pointer items-center gap-1 dark:hover:text-black"
      >
        <TestCollectionIcon type={node.type} />
        <NodeLabel label={node.id} />
        <span className="DragHandle h-full min-h-4 grow" />
        <Icon
          icon="TreeChevronRightIcon"
          className={cn("ml-auto text-[#717171]", {
            "rotate-90": shouldRenderChildNodes,
            "opacity-0": !node.isFolder,
          })}
        />
      </button>

      {shouldRenderChildNodes && (
        <div className="contents">
          <ul className="h-full">
            {node.childNodes.map((childNode) => (
              <BreadcrumbNode
                onNodeUpdate={onNodeUpdate}
                node={childNode}
                depth={depth + 1}
                paddingLeft={paddingLeft}
                paddingRight={paddingRight}
                nodeOffset={nodeOffset}
                onNodeClickCallback={onNodeClickCallback}
              />
            ))}
          </ul>
        </div>
      )}
    </li>
  );
};
