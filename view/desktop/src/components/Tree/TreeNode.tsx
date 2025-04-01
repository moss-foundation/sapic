import { useContext, useMemo, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { useDockviewStore } from "@/store/Dockview";
import { cn } from "@/utils";

import { ContextMenu, Icon, TreeContext } from "..";
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
    paddingLeft,
    paddingRight,
    searchInput,
    onNodeAddCallback,
    onNodeRenameCallback,
    onNodeClickCallback,
    onNodeDoubleClickCallback,
  } = useContext(TreeContext);

  const { currentActivePanelId, currentActiveTreeId, addPanel } = useDockviewStore();

  const nodePaddingLeft = useMemo(() => depth * nodeOffset + paddingLeft + 4, [depth, nodeOffset, paddingLeft]);
  const nodePaddingLeftForAddForm = useMemo(
    () => (depth + 1) * nodeOffset + paddingLeft + 4,
    [depth, nodeOffset, paddingLeft]
  );

  const nodeStyle = useMemo(
    () =>
      cn("flex w-full min-w-0 items-center gap-1 py-0.5", {
        "background-(--moss-treeNode-bg-hover)": currentActivePanelId === node.id && currentActiveTreeId === treeId,
      }),
    [currentActivePanelId, currentActiveTreeId, node.id, treeId]
  );

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
        <div className={nodeStyle} style={{ paddingLeft: nodePaddingLeft }}>
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
        </div>
      ) : (
        <ContextMenu.Root modal={false}>
          <ContextMenu.Trigger asChild>
            <button
              ref={draggableNodeRef}
              style={{
                paddingLeft: nodePaddingLeft,
                paddingRight: paddingRight + 3,
              }}
              onClick={() => {
                if (node.isFolder) handleFolderClick();
                else
                  addPanel({
                    id: `${node.id}`,
                    params: {
                      treeId,
                    },
                  });

                onNodeClickCallback?.(node);
              }}
              onDoubleClick={() => onNodeDoubleClickCallback?.(node)}
              className={cn(nodeStyle, "relative w-full cursor-pointer items-center gap-1 dark:hover:text-black", {
                "hover:background-(--moss-primary-bg-hover)": !isNodeDragging,
              })}
            >
              <TestCollectionIcon type={node.type} />
              <NodeLabel label={node.id} searchInput={searchInput} />
              <span className="DragHandle h-full min-h-4 grow" />
              <Icon
                icon="TreeChevronRightIcon"
                className={cn("ml-auto text-(--moss-icon-primary-text)", {
                  "rotate-90": shouldRenderChildNodes,
                  "opacity-0": !node.isFolder,
                })}
              />
              {preview &&
                createPortal(
                  <ul className="background-(--moss-primary-bg)">
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
                      node={{ ...node, childNodes: [] }}
                      onNodeUpdate={() => {}}
                      depth={0}
                    />
                  </ul>,
                  preview
                )}
            </button>
          </ContextMenu.Trigger>

          <ContextMenu.Portal>
            <ContextMenu.Content>
              {node.isFolder && <ContextMenu.Item label="Add File" onClick={() => setIsAddingFileNode(true)} />}
              {node.isFolder && <ContextMenu.Item label="Add Folder" onClick={() => setIsAddingFolderNode(true)} />}
              <ContextMenu.Item label="Edit" onClick={() => setIsRenamingNode(true)} />
              <ContextMenu.Item label="Item" />
            </ContextMenu.Content>
          </ContextMenu.Portal>
        </ContextMenu.Root>
      )}

      {(isAddingFileNode || isAddingFolderNode) && (
        <div style={{ paddingLeft: nodePaddingLeftForAddForm }} className="flex w-full min-w-0 items-center gap-1">
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
