import { useMemo, useRef } from "react";

import { useStreamEnvironments } from "@/hooks";
import { Tree } from "@/lib/ui/Tree";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { GlobalEnvironmentsListControls } from "./GlobalEnvironmentsListControls";
import { GlobalEnvironmentsListItemRenamingForm } from "./GlobalEnvironmentsListItemRenamingForm";
import { useDraggableGlobalEnvironmentsList, useGlobalEnvironmentsListRenamingForm } from "./hooks";

interface GlobalEnvironmentsListProps {
  environment: StreamEnvironmentsEvent;
}

export const GlobalEnvironmentsListItem = ({ environment }: GlobalEnvironmentsListProps) => {
  const globalEnvironmentsListRef = useRef<HTMLLIElement>(null);

  const { data: environments } = useStreamEnvironments();
  const { activePanelId } = useTabbedPaneStore();

  const { isEditing, setIsEditing, handleRename, handleCancel } = useGlobalEnvironmentsListRenamingForm({
    environment,
  });

  const { instruction } = useDraggableGlobalEnvironmentsList({
    ref: globalEnvironmentsListRef,
    environment,
  });

  const restrictedNames = useMemo(() => {
    if (!environments) return [];
    return environments.environments.map((environment) => environment.name) ?? [];
  }, [environments]);

  const isActive = activePanelId === environment.id;

  return (
    <Tree.RootNode isChildDropBlocked={false} instruction={instruction} dropIndicatorFullWidth={true}>
      <Tree.RootNodeHeader ref={globalEnvironmentsListRef} isActive={isActive}>
        {isEditing ? (
          <GlobalEnvironmentsListItemRenamingForm
            handleRename={handleRename}
            handleCancel={handleCancel}
            environment={environment}
            restrictedNames={restrictedNames}
          />
        ) : (
          <GlobalEnvironmentsListControls environment={environment} setIsEditing={setIsEditing} />
        )}
      </Tree.RootNodeHeader>
    </Tree.RootNode>
  );
};
