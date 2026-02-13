import { useRef } from "react";

import { useGetWorkspaceEnvironments } from "@/db/environmentsSummaries/hooks/useGetWorkspaceEnvironments";
import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { useGetEnvironmentListItemState } from "@/workbench/adapters/tanstackQuery/environmentListItemState/useGetEnvironmentListItemState";

import { WORKSPACE_ENVIRONMENTS_LIST_ID } from "../constants";
import { useDropTargetWorkspaceEnvironmentList } from "../dnd/hooks/useDropTargetWorkspaceEnvironmentList";
import { EnvironmentItem } from "../EnvironmentItem/EnvironmentItem";
import { WorkspaceEnvironmentsListRootControls } from "./WorkspaceEnvironmentsListRootControls";

export const WorkspaceEnvironmentsListRoot = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();

  const workspaceEnvironmentsListRef = useRef<HTMLUListElement>(null);

  const { workspaceEnvironments } = useGetWorkspaceEnvironments();

  const { data: workspaceEnvironmentListItemState } = useGetEnvironmentListItemState(
    WORKSPACE_ENVIRONMENTS_LIST_ID,
    currentWorkspaceId
  );
  const expanded = workspaceEnvironmentListItemState?.expanded ?? false;

  const { instruction } = useDropTargetWorkspaceEnvironmentList({
    refList: workspaceEnvironmentsListRef,
    workspaceEnvironments: workspaceEnvironments ?? [],
  });

  return (
    <Tree.RootNode ref={workspaceEnvironmentsListRef} combineInstruction={instruction} className={cn("cursor-pointer")}>
      <Tree.RootNodeHeader className="cursor-pointer text-sm" disableIndicator={true}>
        <WorkspaceEnvironmentsListRootControls expanded={expanded} />
      </Tree.RootNodeHeader>

      {expanded && (
        <Tree.RootNodeChildren hideDirDepthIndicator>
          {workspaceEnvironments?.map((environment) => (
            <EnvironmentItem key={environment.id} environment={environment} />
          ))}
        </Tree.RootNodeChildren>
      )}
    </Tree.RootNode>
  );
};
