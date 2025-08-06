import { forwardRef, useContext } from "react";
import { createPortal } from "react-dom";

import { ActionMenu } from "@/components";
import { DragHandleButton } from "@/components/DragHandleButton";
import { useUpdateCollectionEntry } from "@/hooks/collection/useUpdateCollectionEntry";
import { Icon } from "@/lib/ui";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { EntryIcon } from "../../EntryIcon";
import { ActiveNodeIndicator } from "../ActiveNodeIndicator";
import { DropIndicatorForTrigger } from "../DropIndicatorForTrigger";
import NodeLabel from "../NodeLabel";
import { TreeContext } from "../Tree";
import { TreeCollectionNode } from "../types";
import { countNumberOfAllNestedChildNodes } from "../utils";
import TreeNode from "./TreeNode";
import { TreeNodeActions } from "./TreeNodeActions";

interface TreeNodeButtonProps {
  node: TreeCollectionNode;
  parentNode: TreeCollectionNode;
  depth: number;
  onAddFile: () => void;
  onAddFolder: () => void;
  onRename: () => void;
  onDelete: () => void;
  isDragging: boolean;
  instruction: Instruction | null;
  preview: HTMLElement | null;
  isLastChild: boolean;
  isRootNode: boolean;
  isChildDropBlocked: boolean | null;
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
      preview,
      isRootNode,
      instruction,
      isLastChild,
      isChildDropBlocked,
    },
    ref
  ) => {
    const { id, nodeOffset, searchInput, treePaddingRight, treePaddingLeft, showNodeOrders } = useContext(TreeContext);

    const { addOrFocusPanel, activePanelId, api } = useTabbedPaneStore();

    const { mutateAsync: updateCollectionEntry } = useUpdateCollectionEntry();

    const handleLabelClick = () => {
      if (node.kind === "Dir" || node.kind === "Case") {
        const panel = api?.getPanel(node.id);

        if (!panel) {
          addOrFocusPanel({
            id: node.id,
            title: `${node.name} Settings`,
            params: {
              collectionId: id,
              iconType: node.kind,
              node: {
                ...node,
                expanded: true,
              },
            },
            component: "FolderSettings",
          });

          updateCollectionEntry({
            collectionId: id,
            updatedEntry: {
              DIR: {
                id: node.id,
                expanded: true,
              },
            },
          });
        } else {
          updateCollectionEntry({
            collectionId: id,
            updatedEntry: {
              DIR: {
                id: node.id,
                expanded: !node.expanded,
              },
            },
          });
        }
        return;
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
        component: node.class === "Request" ? "Request" : "Default",
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

    const nodePaddingLeft = depth * nodeOffset + treePaddingLeft;
    const shouldRenderChildNodes = !!searchInput || (!searchInput && node.kind === "Dir" && node.expanded);
    const numberOfAllNestedChildNodes = countNumberOfAllNestedChildNodes(node);

    return (
      <ActionMenu.Root modal={false}>
        <ActionMenu.Trigger asChild openOnRightClick>
          <button
            ref={ref}
            onClick={handleLabelClick}
            className={cn("group/TreeNode relative flex min-h-[28px] w-full min-w-0 cursor-pointer items-center")}
          >
            {isChildDropBlocked === null && <ActiveNodeIndicator isActive={activePanelId === node.id} />}

            <DropIndicatorForTrigger
              paddingLeft={nodePaddingLeft}
              paddingRight={treePaddingRight}
              instruction={instruction}
              depth={depth}
              isLastChild={isLastChild}
            />

            <span
              className={cn("relative z-10 flex h-full w-full items-center gap-1")}
              style={{ paddingLeft: nodePaddingLeft, paddingRight: treePaddingRight }}
            >
              {!isRootNode && (
                <DragHandleButton
                  className="absolute top-1/2 left-[1px] -translate-y-1/2 opacity-0 transition-all duration-0 group-hover/TreeNode:opacity-100 group-hover/TreeNode:delay-400 group-hover/TreeNode:duration-150"
                  slim
                  ghost
                />
              )}

              <span className="flex size-5 shrink-0 items-center justify-center">
                <button
                  onClick={handleClickOnDir}
                  className={cn(
                    "hover:background-(--moss-icon-primary-background-hover) flex cursor-pointer items-center justify-center rounded-full text-(--moss-icon-primary-text)",
                    {
                      "opacity-0": node.kind !== "Dir",
                    }
                  )}
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

              <EntryIcon entry={node} />

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
