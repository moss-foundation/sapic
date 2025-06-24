/* eslint-disable */
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";

import Icon from "../../lib/ui/Icon";
import NodeLabel from "../CollectionTree/NodeLabel";
import { TestCollectionIcon } from "../CollectionTree/TestCollectionIcon";
import { TreeNodeProps } from "../CollectionTree/types";

interface BreadcrumbNodeProps {
  node: TreeNodeProps;
  onNodeUpdate: (node: TreeNodeProps) => void;
  depth?: number;
  onNodeClickCallback?: (node: TreeNodeProps) => void;
  paddingLeft?: number;
  paddingRight?: number;
  nodeOffset?: number;
}

export const BreadcrumbNode = ({
  node,
  onNodeUpdate,
  depth = 0,
  onNodeClickCallback,
  paddingLeft = 12,
  paddingRight = 8,
  nodeOffset = 12,
}: BreadcrumbNodeProps) => {
  const { addOrFocusPanel } = useTabbedPaneStore();

  const nodePaddingLeft = depth * nodeOffset + paddingLeft + 4;
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
          else
            addOrFocusPanel({
              id: `${node.id}`,
              params: {
                iconType: node.type,
                workspace: true,
              },
              component: "Default",
            });

          onNodeClickCallback?.(node);
        }}
        className="hover:background-(--moss-secondary-background-hover) relative flex w-full cursor-pointer items-center gap-1 rounded-sm py-0.5 dark:hover:text-black"
      >
        <TestCollectionIcon type={node.type} />
        <NodeLabel label={node.id} />
        <span className="DragHandle h-full min-h-4 grow" />
        <Icon
          icon="ChevronRight"
          className={cn("ml-auto text-[#717171]", {
            "rotate-90": shouldRenderChildNodes,
            "opacity-0": !node.isFolder,
          })}
        />
      </button>

      {shouldRenderChildNodes && node.childNodes && node.childNodes.length > 0 && (
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
