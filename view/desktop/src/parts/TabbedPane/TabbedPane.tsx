import { DockviewDidDropEvent, DockviewReact, DockviewReadyEvent, positionToDirection } from "moss-tabs";
import { useRef, useState } from "react";

import { DropNode } from "@/components/ProjectTree/types";
import { useGetLayout } from "@/hooks/sharedStorage/layout/useGetLayout";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { TabbedPaneToolBar, Watermark } from "./components";
import { AddPanelButton } from "./components/AddPanelButton";
import CustomTab from "./components/CustomTab";
import DockviewDebugContainer from "./DebugComponents/DockviewDebugContainer";
import { useTabbedPaneDropTarget } from "./hooks/useDockviewDropTarget";
import { useTabbedPaneEventHandlers } from "./hooks/useDockviewEventHandlers";
import { useTabbedPaneResizeObserver } from "./hooks/useDockviewResizeObserver";
import { useResetGridStateOnWorkspaceChange } from "./hooks/useResetGridStateOnWorkspaceChange";
import { tabbedPaneComponents } from "./TabbedPaneComponents";

const TabbedPane = () => {
  const dockviewRef = useRef<HTMLDivElement>(null);
  const dockviewRefWrapper = useRef<HTMLDivElement>(null);

  const [pragmaticDropElement, setPragmaticDropElement] = useState<DropNode | null>(null);

  const { api, showDebugPanels, addOrFocusPanel, setApi } = useTabbedPaneStore();

  const { canDrop } = useTabbedPaneDropTarget(dockviewRef, setPragmaticDropElement);
  const { data: layout } = useGetLayout();

  useTabbedPaneEventHandlers({ canPragmaticDrop: canDrop });
  //TODO check if this is needed
  useTabbedPaneResizeObserver({ containerRef: dockviewRefWrapper });

  useResetGridStateOnWorkspaceChange();

  const onReady = (event: DockviewReadyEvent) => {
    setApi(event.api);

    if (layout?.tabbedPaneState.gridState) {
      event.api.fromJSON(layout.tabbedPaneState.gridState);
    }
  };

  const onDidDrop = (event: DockviewDidDropEvent) => {
    if (!pragmaticDropElement || !api) return;

    addOrFocusPanel({
      id: pragmaticDropElement.node.id,
      title: pragmaticDropElement.node.name,
      //TODO: this is a hardcoded component, later we we will need to have a more flexible way to handle this
      component: "Endpoint",
      params: {
        projectId: pragmaticDropElement.projectId,
        node: pragmaticDropElement.node,
      },
      position: {
        direction: positionToDirection(event.position),
        referenceGroup: event.group || undefined,
      },
    });
    setPragmaticDropElement(null);
  };

  return (
    <div className="h-full">
      <div className="dockview-demo relative flex h-full w-full grow flex-col rounded">
        {showDebugPanels && <DockviewDebugContainer />}

        <div className="flex h-full">
          <div className="flex grow overflow-hidden">
            <div className="h-full w-full" ref={dockviewRefWrapper}>
              <DockviewReact
                ref={dockviewRef}
                components={tabbedPaneComponents}
                defaultTabComponent={CustomTab}
                rightHeaderActionsComponent={TabbedPaneToolBar}
                leftHeaderActionsComponent={AddPanelButton}
                watermarkComponent={Watermark}
                onReady={onReady}
                onDidDrop={onDidDrop}
                theme={{
                  name: "moss-theme-light",
                  className: "dockview-moss-light",
                  gap: 0,
                }}
                disableAutoResizing
                disableTabsOverflowList
                disableFloatingGroups
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default TabbedPane;
