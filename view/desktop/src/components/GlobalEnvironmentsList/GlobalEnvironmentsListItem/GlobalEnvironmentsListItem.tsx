import { useMemo, useRef } from "react";

import { DropIndicator } from "@/components/DropIndicator";
import { useStreamEnvironments } from "@/hooks";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { StreamEnvironmentsEvent } from "@repo/moss-workspace";

import { GlobalEnvironmentsListActions } from "./GlobalEnvironmentsListActions";
import { GlobalEnvironmentsListButton } from "./GlobalEnvironmentsListButton";
import { GlobalEnvironmentsListIndicator } from "./GlobalEnvironmentsListIndicator";
import { GlobalEnvironmentsListRenamingForm } from "./GlobalEnvironmentsListRenamingForm";
import { useDraggableGlobalEnvironmentsList, useGlobalEnvironmentsListRenamingForm } from "./hooks";

interface GlobalEnvironmentsListProps {
  environment: StreamEnvironmentsEvent;
}

export const GlobalEnvironmentsListItem = ({ environment }: GlobalEnvironmentsListProps) => {
  const globalEnvironmentsListRef = useRef<HTMLDivElement>(null);

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
    <div
      ref={globalEnvironmentsListRef}
      className="group/GlobalEnvironmentsList relative flex min-h-[30px] w-full cursor-pointer items-center justify-between py-1 pr-2 pl-2.5"
      onClick={onClick}
      role="button"
      tabIndex={0}
    >
      <GlobalEnvironmentsListIndicator isActive={isActive} />
      {closestEdge && <DropIndicator noTerminal edge={closestEdge} />}

      {isEditing ? (
        <GlobalEnvironmentsListRenamingForm
          onSubmit={handleRename}
          onCancel={handleCancel}
          currentName={environment.name}
          restrictedNames={restrictedNames}
        />
      ) : (
        <GlobalEnvironmentsListButton environment={environment} />
      )}
      <GlobalEnvironmentsListActions environment={environment} setIsEditing={setIsEditing} />
    </div>
  );
};
