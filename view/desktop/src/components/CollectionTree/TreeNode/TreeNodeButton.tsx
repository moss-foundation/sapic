import { forwardRef, useContext } from "react";
import { createPortal } from "react-dom";

import { ActionMenu } from "@/components";
import { DragHandleButton } from "@/components/DragHandleButton";
import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";
import { Icon } from "@/lib/ui";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { DropIndicatorWithInstruction } from "../DropIndicatorWithInstruction";
import NodeLabel from "../NodeLabel";
import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";
import TreeNode from "./TreeNode";
import { TreeNodeIcon } from "./TreeNodeIcon";

interface TreeNodeButtonProps {
  node: TreeCollectionNode;
  depth: number;
  onAddFile: () => void;
  onAddFolder: () => void;
  onRename: () => void;
  onDelete: () => void;
  isDragging: boolean;
  canDrop: boolean | null;
  instruction: Instruction | null;
  preview: HTMLElement | null;
  isLastChild: boolean;
  isRootNode: boolean;
}

const TreeNodeButton = forwardRef<HTMLButtonElement, TreeNodeButtonProps>(
  (
    {
      node,
      depth,
      onAddFile,
      onAddFolder,
      onRename,
      onDelete,
      isDragging,
      canDrop,
      instruction,
      preview,
      isLastChild,
      isRootNode,
    },
    ref
  ) => {
    const { id, nodeOffset, searchInput, paddingLeft, paddingRight, rootOffset } = useContext(TreeContext);

    const { addOrFocusPanel, activePanelId } = useTabbedPaneStore();

    const { mutateAsync: updateCollectionEntry } = useUpdateCollectionEntry();

    const handleClick = () => {
      if (node.kind === "Dir" || node.kind === "Case") {
        updateCollectionEntry({
          collectionId: id,
          updatedEntry: {
            DIR: {
              id: node.id,
              expanded: true,
            },
          },
        });
      }

      addOrFocusPanel({
        id: `${node.id}`,
        title: node.name,
        params: {
          treeId: id,
          iconType: node.kind,
          node: {
            ...node,
            expanded: true,
          },
          someRandomString: "someRandomString",
        },
        component: "Default",
      });
    };

    const handleClickOnDir = (e: React.MouseEvent<HTMLDivElement>) => {
      e.stopPropagation();
      if (node.kind === "Item") return;

      updateCollectionEntry({
        collectionId: id,
        updatedEntry: {
          DIR: {
            id: node.id,
            expanded: !node.expanded,
          },
        },
      });
    };

    const nodePaddingLeft = depth * nodeOffset;
    const shouldRenderChildNodes = !!searchInput || (!searchInput && node.kind === "Dir" && node.expanded);

    return (
      <ActionMenu.Root modal={false}>
        <ActionMenu.Trigger asChild openOnRightClick>
          <button
            ref={ref}
            onClick={handleClick}
            className={cn("group/treeNode relative flex h-full w-full min-w-0 cursor-pointer items-center")}
          >
            <span
              style={{
                width: `calc(100% - ${rootOffset + paddingRight}px)`,
              }}
              className={cn("absolute inset-x-2 h-full rounded-sm", {
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
              {!isRootNode && (
                <DragHandleButton
                  className="absolute top-1/2 left-[1px] -translate-y-1/2 opacity-0 transition-all duration-0 group-hover/treeNode:opacity-100 group-hover/treeNode:delay-400 group-hover/treeNode:duration-150"
                  slim
                />
              )}

              {node.kind !== "Dir" && instruction !== null && canDrop === true && (
                <DropIndicatorWithInstruction
                  paddingLeft={nodePaddingLeft}
                  paddingRight={paddingRight}
                  instruction={instruction}
                  isFolder={false}
                  depth={depth}
                  canDrop={canDrop}
                  gap={-2}
                  isLastChild={isLastChild}
                />
              )}
              <div
                onClick={handleClickOnDir}
                className={cn(
                  "hover:background-(--moss-icon-primary-background-hover) flex items-center justify-center rounded-full text-(--moss-icon-primary-text)",
                  {
                    "rotate-90": shouldRenderChildNodes,
                    "opacity-0": node.kind !== "Dir",
                  }
                )}
              >
                <Icon icon="ChevronRight" />
              </div>

              <div className="underline">{node.order}</div>
              <TreeNodeIcon node={node} isRootNode={isRootNode} />

              <NodeLabel label={node.name} searchInput={searchInput} className={cn({ "capitalize": isRootNode })} />
              <span className="DragHandle h-full min-h-4 grow" />
            </span>

            {preview &&
              createPortal(
                <ul className="background-(--moss-primary-background) flex gap-1 rounded-sm">
                  <TreeNode
                    isRootNode={isRootNode}
                    parentNode={{
                      ...node,
                      id: "-",
                      name: "DraggedNode",
                      order: undefined,
                      expanded: false,
                      childNodes: [],
                    }}
                    isLastChild={false}
                    node={{ ...node, id: "DraggedNode", childNodes: [] }}
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
            {!isRootNode && <ActionMenu.Item onClick={onRename}>Edit</ActionMenu.Item>}
            {!isRootNode && <ActionMenu.Item onClick={onDelete}>Delete</ActionMenu.Item>}
          </ActionMenu.Content>
        </ActionMenu.Portal>
      </ActionMenu.Root>
    );
  }
);

export default TreeNodeButton;
