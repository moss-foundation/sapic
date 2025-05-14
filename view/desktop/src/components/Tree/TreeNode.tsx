import { useContext, useMemo, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { Icon } from "@/lib/ui";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";

import { ActionMenu, TreeContext } from "..";
import { useDraggableNode } from "./hooks/useDraggableNode";
import { useDropTargetNode } from "./hooks/useDropTargetNode";
import { useNodeAddForm } from "./hooks/useNodeAddForm";
import { useNodeRenamingForm } from "./hooks/useNodeRenamingForm";
import { NodeAddForm } from "./NodeAddForm";
import NodeLabel from "./NodeLabel";
import { NodeRenamingForm } from "./NodeRenamingForm";
import { TestCollectionIcon } from "./TestCollectionIcon";
import { TreeNodeComponentProps, TreeNodeProps } from "./types";
import { hasDescendantWithSearchInput } from "./utils";

export const TreeNode = ({ node, onNodeUpdate, depth, parentNode }: TreeNodeComponentProps) => {
  const {
    treeId,
    nodeOffset,
    searchInput,
    onNodeAddCallback,
    onNodeRenameCallback,
    onNodeClickCallback,
    onNodeDoubleClickCallback,
  } = useContext(TreeContext);

  const { addOrFocusPanel, activePanelId } = useTabbedPaneStore();

  const nodePaddingLeft = useMemo(() => depth * nodeOffset, [depth, nodeOffset]);
  const nodePaddingLeftForAddForm = useMemo(() => (depth + 1) * nodeOffset, [depth, nodeOffset]);

  const handleFolderClick = () => {
    if (!node.isFolder || searchInput) return;

    onNodeUpdate({
      ...node,
      isExpanded: !node.isExpanded,
    });
  };

  const {
    isAddingFileNode,
    isAddingFolderNode,
    setIsAddingFileNode,
    setIsAddingFolderNode,
    handleAddFormSubmit,
    handleAddFormCancel,
  } = useNodeAddForm(node, onNodeUpdate);

  const { isRenamingNode, setIsRenamingNode, handleRenamingFormSubmit, handleRenamingFormCancel } = useNodeRenamingForm(
    node,
    onNodeUpdate
  );

  const [preview, setPreview] = useState<HTMLElement | null>(null);
  const draggableNodeRef = useRef<HTMLButtonElement>(null);
  const dropTargetFolderRef = useRef<HTMLDivElement>(null);
  const dropTargetListRef = useRef<HTMLLIElement>(null);

  const { isDragging: isNodeDragging } = useDraggableNode(draggableNodeRef, node, treeId, isRenamingNode, setPreview);
  useDropTargetNode(node, treeId, dropTargetListRef, dropTargetFolderRef);

  const shouldRenderChildNodes =
    !!searchInput || isAddingFileNode || isAddingFolderNode || (!searchInput && node.isFolder && node.isExpanded);

  const filteredChildNodes = searchInput
    ? node.childNodes.filter((childNode) => hasDescendantWithSearchInput(childNode, searchInput))
    : node.childNodes;

  return (
    <li ref={dropTargetListRef}>
      {isRenamingNode ? (
        <div className="w-full min-w-0">
          <span className="flex w-full items-center gap-1 py-0.5" style={{ paddingLeft: nodePaddingLeft }}>
            <Icon
              icon="ChevronRight"
              className={cn("text-(--moss-icon-primary-text)", {
                "rotate-90": shouldRenderChildNodes,
                "opacity-0": !node.isFolder,
              })}
            />
            <TestCollectionIcon type={node.type} />
            <NodeRenamingForm
              onSubmit={(newName) => {
                handleRenamingFormSubmit(newName);
                onNodeRenameCallback?.({ ...node, id: newName });
              }}
              onCancel={handleRenamingFormCancel}
              restrictedNames={parentNode.childNodes.map((childNode) => childNode.id)}
              currentName={node.id}
            />
          </span>
        </div>
      ) : (
        <ActionMenu.Root modal={false}>
          <ActionMenu.Trigger openOnRightClick>
            <div>
              <button
                ref={draggableNodeRef}
                onClick={() => {
                  if (node.isFolder) handleFolderClick();
                  else {
                    addOrFocusPanel({
                      id: `${node.id}`,
                      params: {
                        treeId,
                        iconType: node.type,
                        workspace: true,
                      },
                      component: "Default",
                    });
                  }

                  onNodeClickCallback?.(node);
                }}
                onDoubleClick={() => onNodeDoubleClickCallback?.(node)}
                className={cn(
                  "group/treeNode relative flex h-full w-full min-w-0 cursor-pointer items-center dark:hover:text-black"
                )}
              >
                <span
                  className={cn("absolute inset-x-2 h-full w-[calc(100%-16px)] rounded-sm", {
                    "group-hover/treeNode:background-(--moss-secondary-background-hover)":
                      !isNodeDragging && activePanelId !== node.id,
                    "background-(--moss-info-background-hover)":
                      activePanelId === node.id && node.uniqueId !== "DraggedNode",
                  })}
                />

                <span
                  className={cn("z-10 flex h-full w-full items-center gap-1 py-0.5")}
                  style={{ paddingLeft: nodePaddingLeft }}
                >
                  <Icon
                    icon="ChevronRight"
                    className={cn("text-(--moss-icon-primary-text)", {
                      "rotate-90": shouldRenderChildNodes,
                      "opacity-0": !node.isFolder,
                    })}
                  />
                  <TestCollectionIcon type={node.type} />
                  <NodeLabel label={node.id} searchInput={searchInput} />
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
                        node={{ ...node, uniqueId: "DraggedNode", childNodes: [] }}
                        onNodeUpdate={() => {}}
                        depth={0}
                      />
                      <Icon icon="ChevronRight" className={cn("opacity-0")} />
                    </ul>,
                    preview
                  )}
              </button>
            </div>
          </ActionMenu.Trigger>

          <ActionMenu.Portal>
            <ActionMenu.Content>
              {node.isFolder && <ActionMenu.Item onClick={() => setIsAddingFileNode(true)}>Add File</ActionMenu.Item>}
              {node.isFolder && (
                <ActionMenu.Item onClick={() => setIsAddingFolderNode(true)}>Add Folder</ActionMenu.Item>
              )}
              <ActionMenu.Item onClick={() => setIsRenamingNode(true)}>Edit</ActionMenu.Item>
            </ActionMenu.Content>
          </ActionMenu.Portal>
        </ActionMenu.Root>
      )}

      {(isAddingFileNode || isAddingFolderNode) && (
        <div style={{ paddingLeft: nodePaddingLeftForAddForm }} className="flex w-full min-w-0 items-center gap-1">
          <Icon icon="ChevronRight" className={cn("opacity-0")} />
          <TestCollectionIcon
            type={node.type}
            className={cn("ml-auto", {
              "opacity-0": isAddingFileNode,
            })}
          />
          <NodeAddForm
            isFolder={isAddingFolderNode}
            restrictedNames={node.childNodes.map((childNode) => childNode.id)}
            onSubmit={(newNode) => {
              handleAddFormSubmit(newNode);
              onNodeAddCallback?.({ ...node, childNodes: [...node.childNodes, newNode] } as TreeNodeProps);
            }}
            onCancel={handleAddFormCancel}
          />
        </div>
      )}

      {shouldRenderChildNodes && (
        <div className="contents" ref={dropTargetFolderRef}>
          <ul className="h-full">
            {filteredChildNodes.map((childNode) => (
              <TreeNode
                parentNode={node}
                onNodeUpdate={onNodeUpdate}
                key={childNode.uniqueId}
                node={childNode}
                depth={depth + 1}
              />
            ))}
          </ul>
        </div>
      )}
    </li>
  );
};

export default TreeNode;
