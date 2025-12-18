import { forwardRef, useContext } from "react";
import { createPortal } from "react-dom";

import { useUpdateProjectResource } from "@/adapters";
import { Icon } from "@/lib/ui";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { ActionMenu } from "@/workbench/ui/components";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { ResourceIcon } from "../../ResourceIcon";
import { ProjectTreeContext } from "../ProjectTreeContext";
import { ProjectTreeNode, ProjectTreeRootNode } from "../types";
import { countNumberOfAllNestedChildNodes } from "../utils";
import TreeNode from "./TreeNode";
import { TreeNodeActions } from "./TreeNodeActions";

interface TreeNodeControlsProps {
  node: ProjectTreeNode;
  parentNode: ProjectTreeNode | ProjectTreeRootNode;
  depth: number;
  onAddFile: () => void;
  onAddFolder: () => void;
  onRename: () => void;
  onDelete: () => void;
  isDragging: boolean;
  preview: HTMLElement | null;
  isChildDropBlocked: boolean | null;
  instruction: Instruction | null;
  isLastChild: boolean;
}

const TreeNodeControls = forwardRef<HTMLDivElement, TreeNodeControlsProps>(
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
      isChildDropBlocked,
      instruction,
      isLastChild,
    },
    ref
  ) => {
    const { id, searchInput, showOrders } = useContext(ProjectTreeContext);

    const { addOrFocusPanel, activePanelId } = useTabbedPaneStore();

    const { mutateAsync: updateProjectResource } = useUpdateProjectResource();

    const handleControlsClick = () => {
      if (node.kind === "Dir") {
        addOrFocusPanel({
          id: node.id,
          title: node.name,
          params: {
            projectId: id,
            node: {
              ...node,
              expanded: true,
            },
          },
          component: "FolderSettingsView",
        });

        if (!node.expanded) {
          updateProjectResource({
            projectId: id,
            updatedResource: {
              DIR: {
                id: node.id,
                expanded: true,
              },
            },
          });
        }
      }
      if (node.kind === "Item") {
        if (node.class === "endpoint") {
          addOrFocusPanel({
            id: node.id,
            title: node.name,
            component: "EndpointView",
            params: {
              resourceId: node.id,
              projectId: id,
              tabIcon: "Http",
            },
          });
        } else {
          addOrFocusPanel({
            id: node.id,
            title: node.name,
            component: "DefaultView",
          });
        }
      }
    };

    const handleClickOnDir = (e: React.MouseEvent<HTMLButtonElement>) => {
      e.stopPropagation();
      if (node.kind === "Item") return;

      updateProjectResource({
        projectId: id,
        updatedResource: {
          DIR: {
            id: node.id,
            expanded: !node.expanded,
          },
        },
      });
    };

    const shouldRenderChildNodes = !!searchInput || (!searchInput && node.kind === "Dir" && node.expanded);
    const numberOfAllNestedChildNodes = countNumberOfAllNestedChildNodes(node);

    return (
      <ActionMenu.Root modal={false}>
        <ActionMenu.Trigger asChild openOnRightClick>
          <Tree.NodeControls
            instruction={instruction}
            isLastChild={isLastChild}
            ref={ref}
            depth={depth}
            isChildDropBlocked={isChildDropBlocked}
            isActive={activePanelId === node.id}
          >
            <Tree.NodeTriggers onClick={handleControlsClick} className="overflow-hidden">
              <Tree.NodeDirToggleIcon
                handleClickOnDir={handleClickOnDir}
                isDir={node.kind === "Dir"}
                shouldRenderChildNodes={shouldRenderChildNodes}
              />
              {showOrders && <Tree.NodeOrder order={node.order} />}
              <ResourceIcon resource={node} />
              <Tree.NodeLabel label={node.name} />
              {node.kind === "Dir" && <Tree.NodeDirCount count={numberOfAllNestedChildNodes} />}
            </Tree.NodeTriggers>

            {node.kind === "Dir" && (
              <TreeNodeActions
                node={node}
                parentNode={parentNode}
                setIsAddingFileNode={onAddFile}
                setIsAddingFolderNode={onAddFolder}
                setIsRenamingNode={onRename}
                className="ml-auto"
              />
            )}

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
                    depth={0}
                  />
                  <Icon icon="ChevronRight" className={cn("opacity-0")} />
                </ul>,
                preview
              )}
          </Tree.NodeControls>
        </ActionMenu.Trigger>
        <ActionMenu.Portal>
          <ActionMenu.Content>
            {node.kind === "Dir" && <ActionMenu.Item onClick={onAddFile}>Add File</ActionMenu.Item>}
            {node.kind === "Dir" && <ActionMenu.Item onClick={onAddFolder}>Add Folder</ActionMenu.Item>}
            <ActionMenu.Item onClick={onRename}>Edit</ActionMenu.Item>
            <ActionMenu.Item onClick={onDelete}>Delete</ActionMenu.Item>
          </ActionMenu.Content>
        </ActionMenu.Portal>
      </ActionMenu.Root>
    );
  }
);

export default TreeNodeControls;
