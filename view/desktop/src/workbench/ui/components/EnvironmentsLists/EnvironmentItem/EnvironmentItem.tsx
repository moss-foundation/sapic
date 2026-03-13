import { useMemo, useRef } from "react";

import { useListWorkspaceEnvironments } from "@/adapters/tanstackQuery/environment/useListWorkspaceEnvironments";
import { useGetProjectEnvironmentsByProjectId } from "@/db/environmentsSummaries/hooks/useGetProjectEnvironmentsByProjectId";
import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { ENVIRONMENT_ITEM_DRAG_TYPE } from "../constants";
import { useDraggableEnvironmentItem } from "../dnd/hooks/useDraggableEnvironmentItem";
import { EnvironmentItemDetails } from "./EnvironmentItemDetails";
import { EnvironmentItemRenamingForm } from "./EnvironmentItemRenamingForm";
import { useEnvironmentItemRenamingForm } from "./hooks/useEnvironmentItemRenamingForm";

interface EnvironmentItemProps {
  environment: EnvironmentSummary;
  offsetLeft?: number;
}

export const EnvironmentItem = ({ environment, offsetLeft }: EnvironmentItemProps) => {
  const environmentItemRef = useRef<HTMLLIElement>(null);

  const { data: workspaceEnvironments } = useListWorkspaceEnvironments();
  const { projectEnvironments } = useGetProjectEnvironmentsByProjectId(environment.projectId);
  const { addOrFocusPanel } = useTabbedPaneStore();

  const envType = environment.projectId ? ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT : ENVIRONMENT_ITEM_DRAG_TYPE.WORKSPACE;

  const restrictedNames = useMemo(() => {
    const { projectId } = environment;

    if (!projectId) {
      return workspaceEnvironments?.items.map((env) => env.name) ?? [];
    }

    return projectEnvironments?.filter((env) => env.projectId === projectId).map((env) => env.name) ?? [];
  }, [environment, projectEnvironments, workspaceEnvironments]);

  const { isEditing, setIsEditing, handleRename, handleCancel } = useEnvironmentItemRenamingForm({
    environment,
  });

  const { isDragging, instruction } = useDraggableEnvironmentItem({
    ref: environmentItemRef,
    environment,
    type: envType,
    canDrag: !isEditing,
  });

  const onClick = () => {
    if (isEditing) return;

    addOrFocusPanel({
      id: environment.id,
      title: environment.name,
      component: "DefaultView",
      params: {
        tabIcon: envType === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT ? "ProjectEnvironment" : "WorkspaceEnvironment",
      },
    });
  };

  return (
    <Tree.Node ref={environmentItemRef} className={cn("cursor-pointer", isDragging && "opacity-50")} onClick={onClick}>
      {isEditing ? (
        <EnvironmentItemRenamingForm
          type={envType}
          environment={environment}
          restrictedNames={restrictedNames}
          handleRename={handleRename}
          handleCancel={handleCancel}
          offsetLeft={offsetLeft}
        />
      ) : (
        <EnvironmentItemDetails
          offsetLeft={offsetLeft}
          type={envType}
          environment={environment}
          instruction={instruction}
          setIsEditing={setIsEditing}
        />
      )}
    </Tree.Node>
  );
};
