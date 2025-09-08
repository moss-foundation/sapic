import { useMemo, useRef } from "react";

import { useStreamEnvironments } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { ENVIRONMENT_ITEM_DRAG_TYPE } from "../constants";
import { EnvironmentListType } from "../types";
import { EnvironmentItemControls } from "./EnvironmentItemControls";
import { EnvironmentListItemRenamingForm } from "./EnvironmentListItemRenamingForm";
import { useDraggableEnvironmentItem } from "./hooks/useDraggableEnvironmentList";
import { useGlobalEnvironmentsListRenamingForm } from "./hooks/useEnvironmentListRenamingForm";

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
        iconType: type === ENVIRONMENT_ITEM_DRAG_TYPE.GLOBAL ? "Environment" : "GroupedEnvironment",
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
