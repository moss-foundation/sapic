import { forwardRef, useContext } from "react";
import { createPortal } from "react-dom";

import { ActionMenu } from "@/components";
import { DragHandleButton } from "@/components/DragHandleButton";
import { Icon } from "@/lib/ui";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";
import { EntryKind, EntryProtocol } from "@repo/moss-collection";

import { DropIndicatorWithInstruction } from "../DropIndicatorWithInstruction";
import NodeLabel from "../NodeLabel";
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
      onNodeUpdate,
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
    const { treeId, nodeOffset, searchInput, paddingRight } = useContext(TreeContext);

    const { addOrFocusPanel, activePanelId } = useTabbedPaneStore();

    const handleClick = () => {
      if (node.kind === "Dir" || node.kind === "Case") {
        onNodeUpdate({
          ...node,
          expanded: true,
        });
      }

      addOrFocusPanel({
        id: `${node.id}`,
        title: node.name,
        params: {
          treeId,
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

      onNodeUpdate({
        ...node,
        expanded: !node.expanded,
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
              <button
                onClick={handleClickOnDir}
                className={cn(
                  "hover:background-(--moss-icon-primary-background-hover) cursor-pointer rounded-full text-(--moss-icon-primary-text)",
                  {
                    "rotate-90": shouldRenderChildNodes,
                    "opacity-0": node.kind !== "Dir",
                  }
                )}
              >
                <Icon icon="ChevronRight" />
              </button>

              {/* <TestCollectionIcon type={node.kind} /> */}

              <DebugCollectionIconPlaceholder protocol={node.protocol} type={node.kind} />

              <NodeLabel label={node.name} searchInput={searchInput} />
              <span className="DragHandle h-full min-h-4 grow" />
            </span>
            {preview &&
              createPortal(
                <ul className="background-(--moss-primary-background) flex gap-1 rounded-sm">
                  <TreeNode
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
            {!isRootNode && <ActionMenu.Item onClick={onRename}>Edit</ActionMenu.Item>}
            {!isRootNode && <ActionMenu.Item onClick={onDelete}>Delete</ActionMenu.Item>}
          </ActionMenu.Content>
        </ActionMenu.Portal>
      </ActionMenu.Root>
    );
  }
);

export default TreeNodeButton;

// TODO: Remove this when we have real icons for the collections
const DebugCollectionIconPlaceholder = ({
  protocol,
  type,
}: {
  protocol: EntryProtocol | undefined;
  type: EntryKind;
}) => {
  if (type === "Dir")
    return (
      <svg
        className={cn("min-h-4 min-w-4")}
        width="16"
        height="16"
        viewBox="0 0 16 16"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M8.10584 4.34613L8.25344 4.5H8.46667H13C13.8284 4.5 14.5 5.17157 14.5 6V12.1333C14.5 12.9529 13.932 13.5 13.3667 13.5H2.63333C2.06804 13.5 1.5 12.9529 1.5 12.1333V3.86667C1.5 3.04707 2.06804 2.5 2.63333 2.5H6.1217C6.25792 2.5 6.38824 2.55557 6.48253 2.65387L8.10584 4.34613Z"
          fill="#EBECF0"
          stroke="#6C707E"
        />
      </svg>
    );

  let color = "";

  switch (protocol) {
    case "Get":
      color = "text-blue-500";
      break;
    case "Post":
      color = "text-green-500";
      break;
    case "Put":
      color = "text-yellow-500";
      break;
    case "Delete":
      color = "text-red-500";
      break;
    case "WebSocket":
      color = "text-purple-500";
      break;
    case "Graphql":
      color = "text-pink-500";
      break;
    case "Grpc":
      color = "text-indigo-500";
      break;
    default:
      color = "text-gray-500";
      break;
  }

  return <div className={cn("text-sm lowercase", color)}>{protocol}</div>;
};
