import { useMemo, useRef } from "react";

import { useStreamEnvironments } from "@/adapters/tanstackQuery/environment";
import { useGetProjectEnvironments } from "@/db/environmentsSummaries/hooks/useGetProjectEnvironments";
import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { ENVIRONMENT_ITEM_DRAG_TYPE } from "./constants";
import { EnvironmentItemControls } from "./EnvironmentItemControls";
import { EnvironmentItemRenamingForm } from "./EnvironmentItemRenamingForm";
import { useDraggableEnvironmentItem } from "./hooks/useDraggableEnvironmentItem";
import { useEnvironmentItemRenamingForm } from "./hooks/useEnvironmentItemRenamingForm";

interface EnvironmentItemProps {
  environment: EnvironmentSummary;
  type: ENVIRONMENT_ITEM_DRAG_TYPE;
}

export const EnvironmentItem = ({ environment, type }: EnvironmentItemProps) => {
  const environmentItemRef = useRef<HTMLLIElement>(null);

  const { data: workspaceEnvironments } = useStreamEnvironments();
  const { projectEnvironments } = useGetProjectEnvironments(environment.projectId);
  const { addOrFocusPanel } = useTabbedPaneStore();

  const restrictedNames = useMemo(() => {
    const { projectId } = environment;

    if (!projectId) {
      return workspaceEnvironments?.map((env) => env.name) ?? [];
    }

    return projectEnvironments?.filter((env) => env.projectId === projectId).map((env) => env.name) ?? [];
  }, [environment, projectEnvironments, workspaceEnvironments]);

  const { isEditing, setIsEditing, handleRename, handleCancel } = useEnvironmentItemRenamingForm({
    environment,
  });

  const { isDragging, instruction } = useDraggableEnvironmentItem({
    ref: environmentItemRef,
    environment,
    type,
    canDrag: !isEditing,
  });

  const onClick = () => {
    if (isEditing) return;

    addOrFocusPanel({
      id: environment.id,
      title: environment.name,
      component: "DefaultView",
      params: {
        tabIcon: type === ENVIRONMENT_ITEM_DRAG_TYPE.PROJECT ? "ProjectEnvironment" : "WorkspaceEnvironment",
      },
    });
  };

  return (
    <Tree.Node ref={environmentItemRef} className={cn("cursor-pointer", isDragging && "opacity-50")} onClick={onClick}>
      {isEditing ? (
        <EnvironmentItemRenamingForm
          type={type}
          environment={environment}
          restrictedNames={restrictedNames}
          handleRename={handleRename}
          handleCancel={handleCancel}
        />
      ) : (
        <EnvironmentItemControls
          type={type}
          environment={environment}
          instruction={instruction}
          setIsEditing={setIsEditing}
        />
      )}
    </Tree.Node>
  );
};
