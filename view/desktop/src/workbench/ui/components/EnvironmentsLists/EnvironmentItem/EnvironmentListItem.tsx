import { useMemo, useRef } from "react";

import { useStreamEnvironments } from "@/adapters/tanstackQuery/environment";
import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { Tree } from "@/lib/ui/Tree";
import { cn } from "@/utils";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";

import { EnvironmentListType } from "../types";
import { EnvironmentItemControls } from "./EnvironmentItemControls";
import { EnvironmentListItemRenamingForm } from "./EnvironmentListItemRenamingForm";
import { useDraggableEnvironmentItem } from "./hooks/useDraggableEnvironmentList";
import { useEnvironmentItemRenamingForm } from "./hooks/useEnvironmentItemRenamingForm";

interface EnvironmentListItemProps {
  environment: EnvironmentSummary;
  type: EnvironmentListType;
}

export const EnvironmentListItem = ({ environment, type }: EnvironmentListItemProps) => {
  const EnvironmentListRef = useRef<HTMLLIElement>(null);

  const { data: environments } = useStreamEnvironments();
  const { addOrFocusPanel } = useTabbedPaneStore();

  const { isEditing, setIsEditing, handleRename, handleCancel } = useEnvironmentItemRenamingForm({
    environment,
  });

  const { isDragging, instruction } = useDraggableEnvironmentItem({
    ref: EnvironmentListRef,
    environment,
    type,
  });

  const onClick = () => {
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
    if (!environments) return [];
    return environments.map((environment) => environment.name) ?? [];
  }, [environments]);

  return (
    <Tree.Node ref={EnvironmentListRef} className={cn("cursor-pointer", isDragging && "opacity-50")} onClick={onClick}>
      {isEditing ? (
        <EnvironmentListItemRenamingForm
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
