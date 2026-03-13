import { useContext } from "react";

import { useGetLocalResourceDetails } from "@/db/resourceDetails/hooks/useGetLocalResourceDetails";
import { resourceSummariesCollection } from "@/db/resourceSummaries/resourceSummariesCollection";
import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { treeItemStateService } from "@/workbench/services/treeItemStateService";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { ActionMenu } from "@/workbench/ui/components";
import { Instruction } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/list-item";

import { ResourceIcon } from "../../ResourceIcon";
import { NODE_OFFSET, NODE_PADDING_RIGHT } from "../constants";
import { ProjectTreeContext } from "../ProjectTreeContext";
import { ResourcesTreeRoot } from "../TreeRoot/types";
import { countNumberOfAllNestedChildNodes } from "../utils";
import { ResourceNodePreview } from "./dnd/ResourceNodePreview";
import { ResourcesTreeNodeActions } from "./ResourcesTreeNodeActions";
import { ResourceNode } from "./types";

interface ResourcesTreeNodeDetailsProps {
  ref?: React.Ref<HTMLDivElement>;
  node: ResourceNode;
  parentNode: ResourceNode | ResourcesTreeRoot;
  depth: number;
  onAddFile: () => void;
  onAddFolder: () => void;
  onRename: () => void;
  onDelete: () => void;
  isDragging: boolean;
  preview: HTMLElement | null;
  reorderInstruction: Instruction | null;
  shouldRenderChildNodes: boolean;
}

const ResourcesTreeNodeDetails = ({
  ref,
  node,
  parentNode,
  depth,
  onAddFile,
  onAddFolder,
  onRename,
  onDelete,
  preview,
  reorderInstruction,
  shouldRenderChildNodes,
}: ResourcesTreeNodeDetailsProps) => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const { id, showOrders } = useContext(ProjectTreeContext);

  const { addOrFocusPanel, activePanelId } = useTabbedPaneStore();

  const localResourceSummary = useGetLocalResourceDetails(node.id);

  const numberOfAllNestedChildNodes = countNumberOfAllNestedChildNodes(node);

  const handleDetailsClick = async () => {
    if (node.kind === "Dir") {
      addOrFocusPanel({
        id: node.id,
        title: node.name,
        component: "FolderSettingsView",
        params: {
          projectId: id,
          node: {
            ...node,
            expanded: true,
          },
        },
      });

      if (!node.expanded) {
        await treeItemStateService.putExpanded(node.id, true, currentWorkspaceId);
        resourceSummariesCollection.update(node.id, (draft) => {
          draft.expanded = true;
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

  const handleClickOnDir = async (e: React.MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    if (node.kind === "Item") return;

    await treeItemStateService.putExpanded(node.id, !node.expanded, currentWorkspaceId);

    resourceSummariesCollection.update(node.id, (draft) => {
      draft.expanded = !node.expanded;
    });
  };

  return (
    <ActionMenu.Root modal={false}>
      <ActionMenu.Trigger asChild openOnRightClick>
        <Tree.NodeDetails
          ref={ref}
          reorderInstruction={reorderInstruction}
          depth={depth}
          nodeOffset={NODE_OFFSET}
          paddingRight={NODE_PADDING_RIGHT}
          isActive={activePanelId === node.id}
          isDirty={localResourceSummary?.metadata.isDirty ?? false}
        >
          <Tree.NodeTriggers onClick={handleDetailsClick} className="overflow-hidden">
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
            <ResourcesTreeNodeActions
              node={node}
              parentNode={parentNode}
              setIsAddingFileNode={onAddFile}
              setIsAddingFolderNode={onAddFolder}
              setIsRenamingNode={onRename}
              className="ml-auto"
            />
          )}

          {preview && <ResourceNodePreview node={node} preview={preview} />}
        </Tree.NodeDetails>
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
};

export default ResourcesTreeNodeDetails;
