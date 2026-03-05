import { useContext, useRef } from "react";

import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { useGetResourcesListItemState } from "@/workbench/adapters/tanstackQuery/resourcesListItemState/useGetResourcesListItemState";

import { ProjectTreeContext } from "../ProjectTreeContext";
import { useDropTargetResourcesList } from "./dnd/hooks/useDropTargetResourcesList";
import { useResourcesNodes } from "./hooks/useResourcesNodes";
import { ResourcesTreeChildren } from "./ResourcesTreeChildren";
import { ResourcesTreeHeader } from "./ResourcesTreeHeader";

export const ResourcesTree = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { id, treePaddingLeft } = useContext(ProjectTreeContext);

  const { rootResourcesNodes } = useResourcesNodes(id);

  const projectResourcesListRef = useRef<HTMLDivElement>(null);
  const listHeaderOffset = treePaddingLeft * 2;

  const { data: expanded = false } = useGetResourcesListItemState(id, currentWorkspaceId);

  const { instruction } = useDropTargetResourcesList({ ref: projectResourcesListRef, rootResourcesNodes });

  return (
    <Tree.List ref={projectResourcesListRef} combineInstruction={instruction}>
      <ResourcesTreeHeader expanded={expanded} offsetLeft={listHeaderOffset} />

      {expanded && <ResourcesTreeChildren rootResourcesNodes={rootResourcesNodes} depth={1} />}
    </Tree.List>
  );
};
