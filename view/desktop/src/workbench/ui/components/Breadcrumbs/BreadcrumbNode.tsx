import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import Icon from "../../../../lib/ui/Icon";
import { ProjectTreeNode } from "../ProjectTree/types";
import { ResourceIcon } from "../ResourceIcon";

interface BreadcrumbNodeProps {
  node: ProjectTreeNode;
  onNodeUpdate: (node: ProjectTreeNode) => void;
  depth?: number;
  paddingLeft?: number;
  paddingRight?: number;
  nodeOffset?: number;
  projectId: string;
}

export const BreadcrumbNode = ({
  node,
  onNodeUpdate,
  depth = 0,
  paddingLeft = 12,
  paddingRight = 8,
  nodeOffset = 12,
  projectId,
}: BreadcrumbNodeProps) => {
  const { addOrFocusPanel } = useTabbedPaneStore();

  const nodePaddingLeft = depth * nodeOffset + paddingLeft + 4;
  const shouldRenderChildNodes = node.kind === "Dir" && node.expanded;

  const handleFolderClick = () => {
    if (node.kind !== "Dir") return;
    onNodeUpdate({
      ...node,
      expanded: !node.expanded,
    });
  };

  return (
    <li>
      <button
        style={{ paddingLeft: nodePaddingLeft, paddingRight: paddingRight + 3 }}
        onClick={() => {
          if (node.kind === "Dir") handleFolderClick();
          else
            addOrFocusPanel({
              id: `${node.id}`,
              title: node.name,
              params: {
                projectId,
                node,
              },
              component: "DefaultView",
            });
        }}
        className="hover:background-(--moss-secondary-background-hover) relative flex w-full cursor-pointer items-center gap-1 rounded-sm py-0.5 dark:hover:text-black"
      >
        <div className="relative size-4">
          <ResourceIcon resource={node} className="absolute right-0 top-0" />
        </div>

        <Tree.NodeLabel label={node.name} />
        <span className="DragHandle h-full min-h-4 grow" />
        <Icon
          icon="ChevronRight"
          className={cn("ml-auto text-[#717171]", {
            "rotate-90": shouldRenderChildNodes,
            "opacity-0": node.kind !== "Dir",
          })}
        />
      </button>

      {shouldRenderChildNodes && node.childNodes && node.childNodes.length > 0 && (
        <div className="contents">
          <ul className="h-full">
            {node.childNodes.map((childNode) => (
              <BreadcrumbNode
                key={childNode.id}
                projectId={projectId}
                onNodeUpdate={onNodeUpdate}
                node={childNode}
                depth={depth + 1}
                paddingLeft={paddingLeft}
                paddingRight={paddingRight}
                nodeOffset={nodeOffset}
              />
            ))}
          </ul>
        </div>
      )}
    </li>
  );
};

export default BreadcrumbNode;
