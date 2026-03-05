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

  const projectResourcesHeaderRef = useRef<HTMLHeadingElement>(null);
  const listHeaderOffset = treePaddingLeft * 2;

  const { data: expanded = false } = useGetResourcesListItemState(id, currentWorkspaceId);

  const { instruction } = useDropTargetResourcesList({ ref: projectResourcesHeaderRef, rootResourcesNodes });

  return (
    <Tree.List combineInstruction={instruction}>
      <ResourcesTreeHeader expanded={expanded} offsetLeft={listHeaderOffset} ref={projectResourcesHeaderRef} />

      {expanded && <ResourcesTreeChildren rootResourcesNodes={rootResourcesNodes} depth={1} />}
    </Tree.List>
  );
};
