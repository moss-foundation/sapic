import { useMemo } from "react";

import { useDockviewStore } from "@/store/Dockview";
import { cn } from "@/utils";

import Icon from "../Icon";
import NodeLabel from "../Tree/NodeLabel";
import { TestCollectionIcon } from "../Tree/TestCollectionIcon";
import { TreeNodeProps } from "../Tree/types";

export const BreadcrumbNode = ({
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
                key={childNode.id}
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
export default BreadcrumbNode;
