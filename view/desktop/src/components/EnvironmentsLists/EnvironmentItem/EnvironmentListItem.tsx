import { useMemo, useRef } from "react";

import { useStreamEnvironments } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { EnvironmentItemControls } from "./EnvironmentItemControls";
import { EnvironmentListItemRenamingForm } from "./EnvironmentListItemRenamingForm";
import { useDraggableEnvironmentItem } from "./hooks/useDraggableEnvironmentList";
import { useGlobalEnvironmentsListRenamingForm } from "./hooks/useEnvironmentListRenamingForm";
import { EnvironmentListType } from "./types";

interface EnvironmentListItemProps {
  environment: StreamEnvironmentsEvent;
  type: EnvironmentListType;
}

export const EnvironmentListItem = ({ environment, type }: EnvironmentListItemProps) => {
  const EnvironmentListRef = useRef<HTMLLIElement>(null);

  const { data: environments } = useStreamEnvironments();
  const { addOrFocusPanel } = useTabbedPaneStore();

  const { isEditing, setIsEditing, handleRename, handleCancel } = useGlobalEnvironmentsListRenamingForm({
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
      component: "Default",
      title: environment.name,
      params: {
        iconType: type === "GlobalEnvironmentItem" ? "Environment" : "GroupedEnvironment",
      },
    });
  };

  const restrictedNames = useMemo(() => {
    if (!environments) return [];
    return environments.environments.map((environment) => environment.name) ?? [];
  }, [environments]);

  return (
    <Tree.Node ref={EnvironmentListRef} className={cn("cursor-pointer", isDragging && "opacity-50")} onClick={onClick}>
      {isEditing ? (
        <EnvironmentListItemRenamingForm
          handleRename={handleRename}
          handleCancel={handleCancel}
          environment={environment}
          restrictedNames={restrictedNames}
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
