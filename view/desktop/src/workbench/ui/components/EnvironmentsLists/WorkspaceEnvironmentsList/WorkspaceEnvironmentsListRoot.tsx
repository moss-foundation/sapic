import { useRef } from "react";

import { useCreateEnvironment } from "@/adapters";
import { useGetWorkspaceEnvironments } from "@/db/environmentsSummaries/hooks/useGetWorkspaceEnvironments";
import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { cn, sortObjectsByOrder } from "@/utils";
import { useGetEnvironmentListItemState } from "@/workbench/adapters/tanstackQuery/environmentListItemState/useGetEnvironmentListItemState";

import { WORKSPACE_ENVIRONMENTS_LIST_ID } from "../constants";
import { useDropTargetWorkspaceEnvironmentList } from "../dnd/hooks/useDropTargetWorkspaceEnvironmentList";
import { EnvironmentAddForm } from "../EnvironmentAddForm/EnvironmentAddForm";
import { EnvironmentItem } from "../EnvironmentItem/EnvironmentItem";
import { WorkspaceEnvironmentsListRootControls } from "./WorkspaceEnvironmentsListRootControls";

export const WorkspaceEnvironmentsListRoot = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { workspaceEnvironments } = useGetWorkspaceEnvironments();
  const { mutateAsync: createEnvironment } = useCreateEnvironment();

  const workspaceEnvironmentsListRef = useRef<HTMLUListElement>(null);
  const { data: expanded = false } = useGetEnvironmentListItemState(WORKSPACE_ENVIRONMENTS_LIST_ID, currentWorkspaceId);

  const { instruction } = useDropTargetWorkspaceEnvironmentList({
    refList: workspaceEnvironmentsListRef,
    workspaceEnvironments: workspaceEnvironments ?? [],
  });

  const handleAddEnvironment = async (name: string) => {
    await createEnvironment({
      name,
      color: undefined,
      variables: [],
      order: workspaceEnvironments.length + 1,
      expanded: false,
    });
  };

  const restrictedNames = workspaceEnvironments?.map((environment) => environment.name) ?? [];

  const sortedWorkspaceEnvironments = sortObjectsByOrder(workspaceEnvironments, "name");

  return (
    <Tree.RootNode ref={workspaceEnvironmentsListRef} combineInstruction={instruction} className={cn("cursor-pointer")}>
      <Tree.RootNodeHeader
        className="text-(--moss-secondary-foreground) cursor-pointer text-sm"
        disableIndicator={true}
      >
        <WorkspaceEnvironmentsListRootControls expanded={expanded} />
      </Tree.RootNodeHeader>

      {expanded && (
        <Tree.RootNodeChildren hideDirDepthIndicator>
          {sortedWorkspaceEnvironments?.map((environment) => (
            <EnvironmentItem key={environment.id} environment={environment} />
          ))}
        </Tree.RootNodeChildren>
      )}

      <EnvironmentAddForm onSubmit={handleAddEnvironment} restrictedNames={restrictedNames} />
    </Tree.RootNode>
  );
};
