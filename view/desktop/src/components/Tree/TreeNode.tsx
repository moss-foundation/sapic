import { useContext, useEffect, useMemo, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { cn } from "@/utils";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import { ContextMenu, Icon, TreeContext } from "..";
import { NodeRenamingForm } from "./NodeRenamingForm";
import { TreeNodeComponentProps } from "./types";
import { canDrop, collapseAllNodes, expandAllNodes, getActualDropSourceTarget, getActualDropTarget } from "./utils";

export const TreeNode = ({
  node,
  onNodeUpdate,
  depth,

  parentNode,
}: TreeNodeComponentProps) => {
  const { treeId, horizontalPadding, nodeOffset, allFoldersAreCollapsed, allFoldersAreExpanded } =
    useContext(TreeContext);

  const paddingLeft = useMemo(
    () => `${depth * nodeOffset + horizontalPadding}px`,
    [depth, nodeOffset, horizontalPadding]
  );
  const paddingRight = useMemo(() => `${horizontalPadding}px`, [horizontalPadding]);

  const draggableRef = useRef<HTMLButtonElement>(null);
  const dropTargetFolderRef = useRef<HTMLUListElement>(null);
  const dropTargetListRef = useRef<HTMLLIElement>(null);

  const [renaming, setRenaming] = useState(false);

  const [preview, setPreview] = useState<HTMLElement | null>(null);

  useEffect(() => {
    const element = draggableRef.current;
    if (!element || renaming) return;

    return draggable({
      element,
      getInitialData: () => ({
        type: "TreeNode",
        data: {
          node,
          treeId,
        },
      }),
      onDrop: () => {
        setPreview(null);
      },
      onGenerateDragPreview({ nativeSetDragImage }) {
        setCustomNativeDragPreview({
          nativeSetDragImage,
          render({ container }) {
            setPreview((prev) => (prev === container ? prev : container));
          },
        });
      },
    });
  }, [treeId, node, renaming]);

  useEffect(() => {
    const element = dropTargetListRef.current || dropTargetFolderRef.current;
    if (!element) return;

    return dropTargetForElements({
      element,
      getData: () => ({
        type: "TreeNode",
        data: {
          treeId,
          node,
        },
      }),
      onDragLeave() {
        element.classList.remove("bg-green-600", "bg-red-600");
      },
      onDrag({ location, source }) {
        if (location.current.dropTargets[0].data.type !== "TreeNode" || location.current?.dropTargets.length === 0) {
          return;
        }

        const sourceTarget = getActualDropSourceTarget(source);
        const dropTarget = getActualDropTarget(location);

        if (!dropTarget || !sourceTarget || dropTarget?.node.uniqueId !== node.uniqueId) {
          element.classList.remove("bg-green-600", "bg-red-600");
          return;
        }
        if (canDrop(sourceTarget, dropTarget, node)) {
          element.classList.add("bg-green-600");
        } else {
          element.classList.add("bg-red-600");
        }
      },
      onDrop({ location, source }) {
        if (location.current?.dropTargets.length === 0 || location.current.dropTargets[0].data.type !== "TreeNode") {
          return;
        }

        const sourceTarget = getActualDropSourceTarget(source);
        const dropTarget = getActualDropTarget(location);

        if (dropTarget?.node.uniqueId !== node.uniqueId) {
          element.classList.remove("bg-green-600", "bg-red-600");
          return;
        }

        if (canDrop(sourceTarget, dropTarget, node)) {
          window.dispatchEvent(
            new CustomEvent("moveTreeNode", {
              detail: {
                source: sourceTarget,
                target: dropTarget,
              },
            })
          );
        }

        element.classList.remove("bg-green-600", "bg-red-600");
      },
    });
  }, [node, treeId]);

  const handleFolderClick = () => {
    if (!node.isFolder) return;

    onNodeUpdate({
      ...node,
      isExpanded: !node.isExpanded,
    });
  };

  const handleFormSubmit = (newId: string) => {
    onNodeUpdate({ ...node, id: newId });
    setRenaming(false);
  };

  const handleFormCancel = () => {
    setRenaming(false);
  };

  const handleExpandAll = () => {
    const newNode = expandAllNodes(node);
    onNodeUpdate({
      ...node,
      childNodes: newNode.childNodes,
    });
  };

  const handleCollapseAll = () => {
    const newNode = collapseAllNodes(node);
    onNodeUpdate({
      ...node,
      childNodes: newNode.childNodes,
    });
  };

  if (node.id === "root") {
    return (
      <div className="h-full">
        <div className="flex w-full min-w-0 py-1 pr-2 items-center justify-between gap-1 focus-within:bg-[#ebecf0] dark:focus-within:bg-[#434343] ">
          <button className="flex gap-1 items-center grow cursor-pointer" onClick={handleFolderClick}>
            <Icon
              icon="TreeChevronRightIcon"
              className={cn("text-[#717171]", {
                "rotate-90": node.isExpanded,
              })}
            />

            <span className="text-ellipsis whitespace-nowrap w-max overflow-hidden">{node.id}</span>
          </button>

          <div className="flex gap-1 items-center">
            {node.isExpanded && (
              <>
                {!allFoldersAreExpanded && (
                  <button
                    className="size-[22px] text-[#717171] hover:text-[#6C707E] hover:bg-[#EBECF0] hover:dark:bg-black/30  flex items-center justify-center rounded-[3px] cursor-pointer"
                    onClick={handleExpandAll}
                  >
                    <Icon icon="TreeExpandAllIcon" />
                  </button>
                )}

                {!allFoldersAreCollapsed && (
                  <button
                    className="size-[22px] text-[#717171] hover:text-[#6C707E] hover:bg-[#EBECF0] hover:dark:bg-black/30  flex items-center justify-center rounded-[3px] cursor-pointer"
                    onClick={handleCollapseAll}
                  >
                    <Icon icon="TreeCollapseAllIcon" />
                  </button>
                )}
              </>
            )}
            <button className="size-[22px] text-[#717171] hover:text-[#6C707E] hover:bg-[#EBECF0] hover:dark:bg-black/30  flex items-center justify-center rounded-[3px] cursor-pointer">
              <Icon icon="TreeDetailIcon" />
            </button>
          </div>
        </div>

        <ul ref={dropTargetFolderRef} className="h-full">
          {node.isExpanded &&
            node.childNodes.map((childNode) => {
              return (
                <TreeNode
                  parentNode={node}
                  onNodeUpdate={onNodeUpdate}
                  key={childNode.uniqueId}
                  node={childNode}
                  depth={0}
                />
              );
            })}
        </ul>
      </div>
    );
  }

  return (
    <li ref={dropTargetListRef}>
      {renaming ? (
        <div className="flex w-full min-w-0 items-center gap-1" style={{ paddingLeft }}>
          <Icon icon={node.isFolder ? "TreeFolderIcon" : "TreeFileIcon"} className="ml-auto" />
          <NodeRenamingForm
            onSubmit={handleFormSubmit}
            onCancel={handleFormCancel}
            restrictedNames={parentNode.childNodes.map((childNode) => childNode.id)}
            currentName={node.id}
          />
        </div>
      ) : (
        <ContextMenu.Root modal={false}>
          <ContextMenu.Trigger asChild>
            <button
              ref={draggableRef}
              style={{ paddingLeft, paddingRight }}
              onClick={node.isFolder ? handleFolderClick : undefined}
              className="flex gap-1 w-full min-w-0 grow items-center cursor-pointer focus-within:outline-none focus-within:bg-[#ebecf0] dark:focus-within:bg-[#747474] relative hover:bg-[#ebecf0] dark:hover:bg-[#434343]"
            >
              <Icon icon={node.isFolder ? "TreeFolderIcon" : "TreeFileIcon"} />

              <span className="text-ellipsis whitespace-nowrap w-max overflow-hidden">{node.id}</span>

              <span className="DragHandle h-full min-h-4 grow" />

              <Icon
                icon="TreeChevronRightIcon"
                className={cn("ml-auto text-[#717171]", {
                  "rotate-90": node.isExpanded,
                  "opacity-0": !node.isFolder,
                })}
              />

              {preview &&
                createPortal(
                  <ul className="bg-[#ebecf0] dark:bg-[#434343]">
                    <TreeNode
                      parentNode={{
                        uniqueId: "-",
                        childNodes: [],
                        type: "",
                        order: 0,
                        isFolder: false,
                        isExpanded: false,
                        id: "-",
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
            <ContextMenu.Content className="text-white">
              <ContextMenu.Item label="Edit" onClick={() => setRenaming(true)} />
              <ContextMenu.Item label="Item 2" />
              <ContextMenu.Item label="Item 3" />
            </ContextMenu.Content>
          </ContextMenu.Portal>
        </ContextMenu.Root>
      )}
      {node.isFolder && node.isExpanded && (
        <ul>
          {node.childNodes.map((childNode) => {
            return (
              <TreeNode
                parentNode={node}
                key={childNode.uniqueId}
                node={childNode}
                depth={depth + 1}
                onNodeUpdate={onNodeUpdate}
              />
            );
          })}
        </ul>
      )}
    </li>
  );
};

export default TreeNode;
