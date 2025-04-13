import "./assets/styles.css";

import React from "react";

import { Breadcrumbs } from "@/components";
import { Scrollbar } from "@/components/Scrollbar";
import { DropNodeElement } from "@/components/Tree/types";
import { Home, Logs, Settings } from "@/pages";
import { useDockviewStore } from "@/store/Dockview";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import {
  DockviewApi,
  DockviewDefaultTab,
  DockviewDidDropEvent,
  DockviewReact,
  DockviewReadyEvent,
  IDockviewPanelProps,
  positionToDirection,
} from "@repo/moss-tabs";

import { LeftControls, PrefixHeaderControls, RightControls } from "./DebugComponents/controls";
import DockviewControls from "./DebugComponents/DockviewControls";
import LogsPanel from "./DebugComponents/LogsPanel";
import Metadata from "./DebugComponents/Metadata";
import { defaultConfig } from "./defaultLayout";
import { useDockviewDropTarget } from "./hooks/useDockviewDropTarget";
import { useDockviewEventHandlers } from "./hooks/useDockviewEventHandlers";
import { useDockviewLogger } from "./hooks/useDockviewLogger";
import { useDockviewResizeObserver } from "./hooks/useDockviewResizeObserver";
import Watermark from "./Watermark";

const DebugContext = React.createContext<boolean>(false);

const TabbedPane = ({ theme }: { theme?: string }) => {
  const { showDebugPanels } = useTabbedPaneStore();
  const dockviewStore = useDockviewStore();

  const [api, setApi] = React.useState<DockviewApi>();
  const [panels, setPanels] = React.useState<string[]>([]);
  const [groups, setGroups] = React.useState<string[]>([]);
  const [activePanel, setActivePanel] = React.useState<string | undefined>();
  const [activeGroup, setActiveGroup] = React.useState<string | undefined>();
  const [pragmaticDropElement, setPragmaticDropElement] = React.useState<DropNodeElement | null>(null);
  const [watermark, setWatermark] = React.useState(false);
  const [showLogs, setShowLogs] = React.useState(false);
  const [debug, setDebug] = React.useState(false);

  const { logLines, addLogLine, setLogLines } = useDockviewLogger();

  const dockviewRef = React.useRef<HTMLDivElement>(null);
  const dockviewRefWrapper = React.useRef<HTMLDivElement>(null);

  useDockviewEventHandlers(api, addLogLine, setPanels, setGroups, setActivePanel, setActiveGroup);
  useDockviewDropTarget(dockviewRef, setPragmaticDropElement);
  useDockviewResizeObserver(api, dockviewRefWrapper);

  const onReady = (event: DockviewReadyEvent) => {
    setApi(event.api);
    dockviewStore.setApi(event.api);
    useTabbedPaneStore.getState().setApi(event.api);
  };

  const onDidDrop = (event: DockviewDidDropEvent) => {
    if (!pragmaticDropElement) return;

    dockviewStore.addPanel({
      id: String(pragmaticDropElement.node.id),
      component: "Default",
      position: {
        direction: positionToDirection(event.position),
        referenceGroup: event.group || undefined,
      },
    });
    setPragmaticDropElement(null);
  };

  React.useEffect(() => {
    if (!api) return;

    const initializeLayout = async () => {
      try {
        await new Promise((resolve) => setTimeout(resolve, 100));
        api?.clear();
        await new Promise((resolve) => setTimeout(resolve, 100));
        defaultConfig(api);
      } catch (e) {
        console.warn("Failed to initialize layout:", e);
      }
    };

    const timeoutId = setTimeout(initializeLayout, 0);
    return () => clearTimeout(timeoutId);
  }, [api]);

  const components = {
    Default: (props: IDockviewPanelProps) => {
      const isDebug = React.useContext(DebugContext);

      return (
        <>
          <Breadcrumbs panelId={props.api.id} />
          <Scrollbar
            className={`relative h-full overflow-auto p-1.25 ${isDebug ? "border-2 border-dashed border-orange-500" : ""} ${
              props.api.isActive ? "select-text" : "select-none"
            }`}
          >
            <span className="pointer-events-none absolute top-1/2 left-1/2 flex -translate-x-1/2 -translate-y-1/2 transform flex-col text-[42px] opacity-50">
              <span>{props.api.title}</span>

              <span>{Math.random().toFixed(2)}</span>
              {props?.params.someRandomString && (
                <span className="text-xs">some random string from backend: {props.params.someRandomString}</span>
              )}
            </span>

            {isDebug && (
              <Metadata
                onClick={() => {
                  props.api.setRenderer(props.api.renderer === "always" ? "onlyWhenVisible" : "always");
                }}
                api={props.api}
              />
            )}
          </Scrollbar>
        </>
      );
    },
    nested: () => {
      return (
        <DockviewReact
          components={components}
          onReady={(event: DockviewReadyEvent) => {
            event.api.addPanel({ id: "panel_1", component: "Default" });
            event.api.addPanel({ id: "panel_2", component: "Default" });
            event.api.addPanel({
              id: "panel_3",
              component: "Default",
            });

            event.api.onDidRemovePanel((e) => {
              console.log("remove", e);
            });
          }}
          className={"dockview-theme-light"}
        />
      );
    },
    iframe: (props: IDockviewPanelProps) => {
      return (
        <iframe
          onMouseDown={() => {
            if (!props.api.isActive) {
              props.api.setActive();
            }
          }}
          className="h-full w-full"
        />
      );
    },
    Home: Home,
    Settings: Settings,
    Logs: Logs,
  };

  const headerComponents = {
    default: DockviewDefaultTab,
  };

  return (
    <div className="dockview-demo relative flex h-full w-full grow flex-col rounded" ref={dockviewRefWrapper}>
      {showDebugPanels && (
        <DockviewControls
          api={api}
          panels={panels}
          activePanel={activePanel}
          groups={groups}
          activeGroup={activeGroup}
          toggleCustomWatermark={() => setWatermark(!watermark)}
          hasCustomWatermark={watermark}
          toggleDebug={() => setDebug(!debug)}
          toggleLogs={() => setShowLogs(!showLogs)}
          showLogs={showLogs}
        />
      )}
      <div className="h-full">
        <Scrollbar className="overflow-hidden">
          <DebugContext.Provider value={debug}>
            <DockviewReact
              disableAutoResizing
              ref={dockviewRef}
              components={components}
              defaultTabComponent={headerComponents.default}
              rightHeaderActionsComponent={RightControls}
              leftHeaderActionsComponent={LeftControls}
              prefixHeaderActionsComponent={PrefixHeaderControls}
              watermarkComponent={watermark ? Watermark : undefined}
              onReady={onReady}
              className={theme || "dockview-theme-light"}
              onDidDrop={onDidDrop}
            />
          </DebugContext.Provider>
        </Scrollbar>
        {showLogs && <LogsPanel logLines={logLines} onClear={() => setLogLines([])} />}
      </div>
    </div>
  );
};

export default TabbedPane;
