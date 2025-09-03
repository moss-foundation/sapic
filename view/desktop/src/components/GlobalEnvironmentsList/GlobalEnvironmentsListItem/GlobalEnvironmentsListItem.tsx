import { useMemo, useRef } from "react";

import { DropIndicator } from "@/components/DropIndicator";
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
  const { addOrFocusPanel, activePanelId } = useTabbedPaneStore();

  const { isEditing, setIsEditing, handleRename, handleCancel } = useGlobalEnvironmentsListRenamingForm({
    environment,
  });

  const { closestEdge } = useDraggableGlobalEnvironmentsList({ ref: globalEnvironmentsListRef, environment });

  const onClick = () => {
    addOrFocusPanel({
      id: environment.id,
      component: "Default",
      title: environment.name,
      params: {
        iconType: "Environment",
      },
    });
  };

  const isActive = activePanelId === environment.id;

  const restrictedNames = useMemo(() => {
    if (!environments) return [];
    return environments.map((environment) => environment.name) ?? [];
  }, [environments]);

  return (
    <Tree.RootNodeHeader ref={globalEnvironmentsListRef} isActive={isActive} onClick={onClick}>
      {closestEdge && <DropIndicator noTerminal edge={closestEdge} />}

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
  );
};
