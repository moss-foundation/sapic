import { forwardRef, useContext } from "react";
import { createPortal } from "react-dom";

import { ActionMenu } from "@/components";
import { DragHandleButton } from "@/components/DragHandleButton";
import { Icon } from "@/lib/ui";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DropIndicatorWithInstruction } from "../DropIndicatorWithInstruction";
import NodeLabel from "../NodeLabel";
import { TestCollectionIcon } from "../TestCollectionIcon";
import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";
import TreeNode from "./TreeNode";

interface TreeNodeButtonProps {
  node: TreeCollectionNode;
  onNodeUpdate: (node: TreeCollectionNode) => void;
  depth: number;
  onAddFile: () => void;
  onAddFolder: () => void;
  onRename: () => void;
  isDragging: boolean;
  canDrop: boolean | null;
  instruction: Instruction | null;
  preview: HTMLElement | null;
  isLastChild: boolean;
}

const TreeNodeButton = forwardRef<HTMLButtonElement, TreeNodeButtonProps>(
  (
    {
      node,
      onNodeUpdate,
      depth,
      onAddFile,
      onAddFolder,
      onRename,
      isDragging,
      canDrop,
      instruction,
      preview,
      isLastChild,
    },
    ref
  ) => {
    const { treeId, nodeOffset, searchInput, onNodeClickCallback, onNodeDoubleClickCallback, paddingRight } =
      useContext(TreeContext);

    const { addOrFocusPanel, activePanelId } = useTabbedPaneStore();

    const handleClick = () => {
      if (node.kind === "Dir") {
        onNodeUpdate({
          ...node,
          expanded: !node.expanded,
        });
      } else {
        addOrFocusPanel({
          id: `${node.id}`,
          params: {
            treeId,
            iconType: node.kind,
            workspace: true,
          },
          component: "Default",
        });
      }
      onNodeClickCallback?.(node);
    };

    const handleDoubleClick = () => onNodeDoubleClickCallback?.(node);

    const nodePaddingLeft = depth * nodeOffset;
    const shouldRenderChildNodes = !!searchInput || (!searchInput && node.kind === "Dir" && node.expanded);

    return (
      <ActionMenu.Root modal={false}>
        <ActionMenu.Trigger asChild openOnRightClick>
          <button
            ref={ref}
            onClick={handleClick}
            onDoubleClick={handleDoubleClick}
            className={cn("group/treeNode relative flex h-full w-full min-w-0 cursor-pointer items-center")}
          >
            <span
              className={cn("absolute inset-x-2 h-full w-[calc(100%-16px)] rounded-sm", {
                "group-hover/treeNode:background-(--moss-secondary-background-hover)":
                  !isDragging && activePanelId !== node.id,
                "background-(--moss-info-background-hover)": activePanelId === node.id && node.id !== "DraggedNode",
              })}
            />
            <span
              className={cn("relative z-10 flex h-full w-full items-center gap-1 py-0.5", {
                "background-(--moss-error-background)": canDrop === false,
              })}
              style={{ paddingLeft: nodePaddingLeft }}
            >
              <DragHandleButton
                className="absolute top-1/2 left-[1px] -translate-y-1/2 opacity-0 transition-all duration-0 group-hover/treeNode:opacity-100 group-hover/treeNode:delay-400 group-hover/treeNode:duration-150"
                slim
              />

              {node.kind === "Dir" && instruction !== null && canDrop === true && (
                <DropIndicatorWithInstruction
                  paddingLeft={nodePaddingLeft}
                  paddingRight={paddingRight}
                  instruction={instruction}
                  isFolder={node.kind === "Dir"}
                  depth={depth}
                  isLastChild={isLastChild}
                />
              )}

              <Icon
                icon="ChevronRight"
                className={cn("text-(--moss-icon-primary-text)", {
                  "rotate-90": shouldRenderChildNodes,
                  "opacity-0": node.kind !== "Dir",
                })}
              />

              <TestCollectionIcon kind={node.kind} />
              <NodeLabel label={node.name} searchInput={searchInput} />
              <span className="DragHandle h-full min-h-4 grow" />
            </span>
            {preview &&
              createPortal(
                <ul className="background-(--moss-primary-background) flex gap-1 rounded-sm">
                  <TreeNode
                    parentNode={{
                      uniqueId: "-",
                      childNodes: [],
                      type: "",
                      order: 0,
                      isFolder: false,
                      isExpanded: false,
                      id: "-",
                      isRoot: false,
                    }}
                    isLastChild={false}
                    node={{ ...node, id: "DraggedNode", childNodes: [] }}
                    onNodeUpdate={() => {}}
                    depth={0}
                  />
                  <Icon icon="ChevronRight" className={cn("opacity-0")} />
                </ul>,
                preview
              )}
          </button>
        </ActionMenu.Trigger>
        <ActionMenu.Portal>
          <ActionMenu.Content>
            {node.kind === "Dir" && <ActionMenu.Item onClick={onAddFile}>Add File</ActionMenu.Item>}
            {node.kind === "Dir" && <ActionMenu.Item onClick={onAddFolder}>Add Folder</ActionMenu.Item>}
            <ActionMenu.Item onClick={onRename}>Edit</ActionMenu.Item>
          </ActionMenu.Content>
        </ActionMenu.Portal>
      </ActionMenu.Root>
    );
  }
);

export default TreeNodeButton;
