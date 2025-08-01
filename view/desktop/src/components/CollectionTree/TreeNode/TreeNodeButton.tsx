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
import { countNumberOfAllNestedChildNodes } from "../utils";
import TreeNode from "./TreeNode";
import { TreeNodeActions } from "./TreeNodeActions";
import { TreeNodeIcon } from "./TreeNodeIcon";

interface TreeNodeButtonProps {
  node: TreeCollectionNode;
  parentNode: TreeCollectionNode;
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
      parentNode,
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
    const { id, nodeOffset, searchInput, treePaddingRight, treePaddingLeft, showNodeOrders } = useContext(TreeContext);

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
        id: node.id,
        title: node.name,
        params: {
          collectionId: id,
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

    const handleClickOnDir = (e: React.MouseEvent<HTMLButtonElement>) => {
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
    const numberOfAllNestedChildNodes = countNumberOfAllNestedChildNodes(node);

    return (
      <ActionMenu.Root modal={false}>
        <ActionMenu.Trigger asChild openOnRightClick>
          <button
            ref={ref}
            onClick={handleClick}
            className={cn(
              "group/treeNode relative flex h-full w-full min-w-0 cursor-pointer items-center py-0.75 leading-[19px]"
            )}
          >
            <span
              style={{
                width: `calc(100% - ${treePaddingLeft}px - ${treePaddingRight}px)`,
                inset: `0 ${treePaddingLeft}px 0 ${treePaddingRight}px`,
              }}
              className={cn("absolute h-full rounded-sm", {
                "group-hover/treeNode:background-(--moss-secondary-background-hover)":
                  !isDragging && activePanelId !== node.id,
                "background-(--moss-info-background-hover)": activePanelId === node.id && node.id !== "DraggedNode",
              })}
            />

            <span
              className={cn("relative z-10 flex h-full min-h-[22px] w-full items-center gap-1", {
                "background-(--moss-error-background)": canDrop === false,
              })}
              style={{ paddingLeft: nodePaddingLeft, paddingRight: treePaddingRight }}
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
                  paddingRight={treePaddingRight}
                  instruction={instruction}
                  isFolder={false}
                  depth={depth}
                  canDrop={canDrop}
                  gap={-1}
                  isLastChild={isLastChild}
                />
              )}

              <span className="flex size-5 shrink-0 items-center justify-center">
                <button
                  onClick={handleClickOnDir}
                  className="hover:background-(--moss-icon-primary-background-hover) flex cursor-pointer items-center justify-center rounded-full text-(--moss-icon-primary-text)"
                >
                  <Icon
                    icon="ChevronRight"
                    className={cn("text-(--moss-icon-primary-text)", {
                      "rotate-90": shouldRenderChildNodes,
                      "opacity-0": node.kind !== "Dir",
                    })}
                  />
                </button>
              </span>

              {showNodeOrders && <div className="underline">{node.order}</div>}

              <TreeNodeIcon node={node} />

              <NodeLabel label={node.name} searchInput={searchInput} className={cn({ "capitalize": isRootNode })} />

              {node.kind === "Dir" && (
                <div className="text-(--moss-tree-entries-counter)">({numberOfAllNestedChildNodes})</div>
              )}

              <span className="DragHandle h-full min-h-4 grow" />

              {node.kind === "Dir" && (
                <TreeNodeActions
                  node={node}
                  parentNode={parentNode}
                  setIsAddingFileNode={onAddFile}
                  setIsAddingFolderNode={onAddFolder}
                  setIsRenamingNode={onRename}
                />
              )}
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
