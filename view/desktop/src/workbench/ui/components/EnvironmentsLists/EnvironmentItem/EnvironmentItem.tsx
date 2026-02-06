import { useMemo, useRef } from "react";

import { useStreamEnvironments } from "@/adapters/tanstackQuery/environment";
import { useGetProjectEnvironments } from "@/db/environmentsSummaries/hooks/useGetProjectEnvironments";
import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { EnvironmentListType } from "../types";
import { EnvironmentItemControls } from "./EnvironmentItemControls";
import { EnvironmentItemRenamingForm } from "./EnvironmentItemRenamingForm";
import { useDraggableEnvironmentItem } from "./hooks/useDraggableEnvironmentList";
import { useEnvironmentItemRenamingForm } from "./hooks/useEnvironmentItemRenamingForm";

interface EnvironmentItemProps {
  environment: EnvironmentSummary;
  type: EnvironmentListType;
}

export const EnvironmentItem = ({ environment, type }: EnvironmentItemProps) => {
  const environmentItemRef = useRef<HTMLLIElement>(null);

  const { data: workspaceEnvironments } = useStreamEnvironments();
  const { projectEnvironments } = useGetProjectEnvironments(environment.projectId);
  const { addOrFocusPanel } = useTabbedPaneStore();

  const { isEditing, setIsEditing, handleRename, handleCancel } = useEnvironmentItemRenamingForm({
    environment,
  });

  const { isDragging, instruction } = useDraggableEnvironmentItem({
    ref: environmentItemRef,
    environment,
    type,
  });

  const onClick = () => {
    if (isEditing) return;

    addOrFocusPanel({
      id: environment.id,
      title: environment.name,
      component: "DefaultView",
      params: {
        tabIcon: type === EnvironmentListType.GLOBAL ? "Environment" : "GroupedEnvironment",
      },
    });
  };

  const restrictedNames = useMemo(() => {
    const { projectId } = environment;

    if (!projectId) {
      return workspaceEnvironments?.map((env) => env.name) ?? [];
    }

    return projectEnvironments?.filter((env) => env.projectId === projectId).map((env) => env.name) ?? [];
  }, [environment, projectEnvironments, workspaceEnvironments]);

  return (
    <Tree.Node ref={environmentItemRef} className={cn("cursor-pointer", isDragging && "opacity-50")} onClick={onClick}>
      {isEditing ? (
        <EnvironmentItemRenamingForm
          handleRename={handleRename}
          handleCancel={handleCancel}
          environment={environment}
          restrictedNames={restrictedNames}
          type={type}
        />
      ) : (
        <EnvironmentItemControls
          environment={environment}
          setIsEditing={setIsEditing}
          type={type}
          instruction={instruction}
        />
      )}
    </Tree.Node>
  );
};
