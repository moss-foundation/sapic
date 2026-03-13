import { useRef } from "react";

import { useCreateEnvironment } from "@/adapters";
import { useGetWorkspaceEnvironments } from "@/db/environmentsSummaries/hooks/useGetWorkspaceEnvironments";
import { useCurrentWorkspace } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { cn, sortObjectsByOrder } from "@/utils";
import { useGetEnvironmentListItemState } from "@/workbench/adapters/tanstackQuery/environmentListItemState/useGetEnvironmentListItemState";

import { NODE_OFFSET, TREE_HEADER_PADDING_LEFT } from "../../ProjectTree/constants";
import { WORKSPACE_ENVIRONMENTS_LIST_ID } from "../constants";
import { useDropTargetWorkspaceEnvironmentList } from "../dnd/hooks/useDropTargetWorkspaceEnvironmentList";
import { WorkspaceEnvironmentAddForm } from "../EnvironmentAddForm/WorkspaceEnvironmentAddForm";
import { EnvironmentItem } from "../EnvironmentItem/EnvironmentItem";
import { WorkspaceEnvironmentsListRootDetails } from "./WorkspaceEnvironmentsListRootDetails";

export const WorkspaceEnvironmentsListRoot = () => {
  const { currentWorkspaceId } = useCurrentWorkspace();
  const { workspaceEnvironments } = useGetWorkspaceEnvironments();

  const { mutateAsync: createEnvironment } = useCreateEnvironment();

  const workspaceEnvironmentsListRef = useRef<HTMLDivElement>(null);
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

  //TODO this is hardcoded for now, we need another way to get the offset
  const listHeaderOffset = TREE_HEADER_PADDING_LEFT;
  const listItemOffset = NODE_OFFSET * 2;

  return (
    <Tree.List ref={workspaceEnvironmentsListRef} combineInstruction={instruction} className={cn("cursor-pointer")}>
      <Tree.ListHeader
        className="text-(--moss-secondary-foreground) cursor-pointer text-sm"
        paddingLeft={listHeaderOffset}
      >
        <WorkspaceEnvironmentsListRootDetails expanded={expanded} />
      </Tree.ListHeader>

      {expanded && (
        <>
          <Tree.RootNodeChildren>
            {sortedWorkspaceEnvironments?.map((environment) => (
              <EnvironmentItem key={environment.id} environment={environment} offsetLeft={listItemOffset} />
            ))}
          </Tree.RootNodeChildren>

          <WorkspaceEnvironmentAddForm
            onSubmit={handleAddEnvironment}
            restrictedNames={restrictedNames}
            offsetLeft={listItemOffset}
          />
        </>
      )}
    </Tree.List>
  );
};
