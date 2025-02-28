import { SVGProps, useContext, useEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";

import { cn } from "@/utils";
import { draggable, dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { setCustomNativeDragPreview } from "@atlaskit/pragmatic-drag-and-drop/element/set-custom-native-drag-preview";

import { ContextMenu, TreeContext } from "..";
import RecursiveTree from "./RecursiveTree";
import { NodeProps, TreeNodeProps } from "./types";
import { useNodeRedacting } from "./useNodeRedacting";

export const TreeNode = ({
  node,
  onNodeUpdate,
  onNodeExpand,
  onNodeCollapse,
  depth,
  horizontalPadding,
  nodeOffset,
}: TreeNodeProps) => {
  const TreeContextValues = useContext(TreeContext);

  const paddingLeft = `${depth * nodeOffset + horizontalPadding}px`;
  const paddingRight = `${horizontalPadding}px`;

  const buttonRef = useRef<HTMLButtonElement>(null);
  const treeLiRef = useRef<HTMLLIElement>(null);
  const treeUlRef = useRef<HTMLUListElement>(null);

  const { redacting, setRedacting, inputRef, handleButtonKeyUp, handleInputKeyUp, handleSubmit } = useNodeRedacting(
    node,
    onNodeUpdate
  );

  const handleButtonClick = () => {
    if (!node.isFolder) return;

    const updatedNode = { ...node, isExpanded: !node.isExpanded };
    if (updatedNode.isExpanded) {
      onNodeExpand?.(updatedNode);
    } else {
      onNodeCollapse?.(updatedNode);
    }
    onNodeUpdate(updatedNode);
  };

  const handleChildNodesUpdate = (nodes: NodeProps[]) => {
    onNodeUpdate({ ...node, childNodes: nodes });
  };

  const [preview, setPreview] = useState<HTMLElement | null>(null);

  useEffect(() => {
    const element = buttonRef.current;

    if (!element) return;

    return draggable({
      element: element,
      getInitialData: () => ({ node, TreeId: TreeContextValues.TreeId }),
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
  }, [TreeContextValues.TreeId, node]);

  useEffect(() => {
    const element = treeUlRef.current || treeLiRef.current;

    if (!element) return;

    return dropTargetForElements({
      element,
      getData: () => ({ node, TreeId: TreeContextValues.TreeId, depth }),
      onDrop: ({ source }) => {
        if (
          TreeContextValues.TreeId === TreeContextValues.dropSourceData?.TreeId &&
          TreeContextValues.dropSourceData?.node?.id === node.id
        ) {
          const sourceNode = source.data.node as NodeProps;
          if (node.childNodes.some(({ id }) => id === sourceNode.id)) {
            return;
          }

          window.dispatchEvent(
            new CustomEvent("moveTreeNode", {
              detail: {
                source: {
                  treeId: source.data.TreeId,
                  node: sourceNode,
                },
                target: {
                  treeId: TreeContextValues.TreeId,
                  node,
                },
              },
            })
          );
        }
      },
    });
  }, [TreeContextValues.TreeId, TreeContextValues.dropSourceData, depth, node, onNodeUpdate]);

  if (node.id === "root") {
    return (
      <ul
        ref={treeUlRef}
        className={cn("w-full select-none", {
          "bg-[#ebecf0] dark:bg-[#434343]":
            TreeContextValues.dropSourceData?.node?.id === node.id &&
            TreeContextValues.TreeId === TreeContextValues.dropSourceData?.TreeId,
        })}
      >
        <RecursiveTree
          nodes={node.childNodes}
          onChildNodesUpdate={handleChildNodesUpdate}
          onNodeUpdate={onNodeUpdate}
          onNodeExpand={onNodeExpand}
          onNodeCollapse={onNodeCollapse}
          depth={depth}
          horizontalPadding={horizontalPadding}
          nodeOffset={nodeOffset}
        />
      </ul>
    );
  }

  return (
    <li
      key={node.id}
      className={cn("w-full select-none", {
        "bg-[#ebecf0] dark:bg-[#434343]":
          TreeContextValues.dropSourceData?.node?.id === node.id &&
          TreeContextValues.TreeId === TreeContextValues.dropSourceData?.TreeId,
      })}
      ref={treeLiRef}
    >
      {redacting ? (
        <div
          className="flex w-full min-w-0 items-center gap-1 focus-within:bg-[#ebecf0] dark:focus-within:bg-[#434343]"
          style={{ paddingLeft, paddingRight }}
        >
          {node.isFolder ? <FolderIcon className="min-w-4 min-h-4" /> : <FileIcon className="min-w-4 min-h-4" />}
          <form onSubmit={handleSubmit} className="grow w-full">
            <input
              ref={inputRef}
              className="w-full focus-within:outline-none"
              onKeyUp={handleInputKeyUp}
              onBlur={handleSubmit}
            />
          </form>
          <ChevronRightIcon className="opacity-0 ml-auto min-w-4 min-h-4" />
        </div>
      ) : (
        <ContextMenu.Root>
          <ContextMenu.Trigger asChild>
            <button
              className="flex gap-1 w-full min-w-0 grow items-center cursor-pointer focus-within:outline-none focus-within:bg-[#ebecf0] dark:focus-within:bg-[#434343] relative"
              style={{ paddingLeft, paddingRight }}
              onClick={handleButtonClick}
              onKeyUp={handleButtonKeyUp}
              ref={buttonRef}
            >
              {node.isFolder ? <FolderIcon className="min-w-4 min-h-4" /> : <FileIcon className="min-w-4 min-h-4" />}
              <span className="text-ellipsis whitespace-nowrap w-max overflow-hidden">{node.id}</span>
              <ChevronRightIcon
                className={cn("ml-auto min-w-4 min-h-4", {
                  "rotate-90": node.isExpanded,
                  "opacity-0": !node.isFolder,
                })}
              />
              {preview &&
                createPortal(
                  <TreeNode
                    node={{ ...node, childNodes: [] }}
                    onNodeUpdate={() => {}}
                    depth={0}
                    horizontalPadding={0}
                    nodeOffset={0}
                  />,
                  preview
                )}
            </button>
          </ContextMenu.Trigger>
          <ContextMenu.Content>
            <ContextMenu.Item label="Edit" onClick={() => setRedacting(true)} />
          </ContextMenu.Content>
        </ContextMenu.Root>
      )}
      {node.childNodes && node.isExpanded && (
        <ul>
          <RecursiveTree
            nodes={node.childNodes}
            onChildNodesUpdate={handleChildNodesUpdate}
            onNodeUpdate={onNodeUpdate}
            onNodeExpand={onNodeExpand}
            onNodeCollapse={onNodeCollapse}
            depth={depth + 1}
            horizontalPadding={horizontalPadding}
            nodeOffset={nodeOffset}
          />
        </ul>
      )}
    </li>
  );
};

const FolderIcon = ({ ...props }: SVGProps<SVGSVGElement>) => {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" {...props}>
      <path
        d="M8.10584 4.34613L8.25344 4.5H8.46667H13C13.8284 4.5 14.5 5.17157 14.5 6V12.1333C14.5 12.9529 13.932 13.5 13.3667 13.5H2.63333C2.06804 13.5 1.5 12.9529 1.5 12.1333V3.86667C1.5 3.04707 2.06804 2.5 2.63333 2.5H6.1217C6.25792 2.5 6.38824 2.55557 6.48253 2.65387L8.10584 4.34613Z"
        fill="#EBECF0"
        stroke="#6C707E"
      />
    </svg>
  );
};

const FileIcon = ({ ...props }: SVGProps<SVGSVGElement>) => {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      {...props}
    >
      <path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
      <path d="M14 2v4a2 2 0 0 0 2 2h4" />
    </svg>
  );
};

const ChevronRightIcon = ({ ...props }: SVGProps<SVGSVGElement>) => {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg" {...props}>
      <path d="M6 11.5L9.5 8L6 4.5" stroke="#818594" strokeLinecap="round" />
    </svg>
  );
};

export default TreeNode;
