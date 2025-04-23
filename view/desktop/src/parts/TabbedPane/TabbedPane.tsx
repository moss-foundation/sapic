import "./assets/styles.css";

import React from "react";

import { Breadcrumbs } from "@/components";
import { Scrollbar } from "@/components/Scrollbar";
import { DropNodeElement } from "@/components/Tree/types";
import { Home, Logs, Settings, WelcomePage } from "@/pages";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import {
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
import { useDockviewDropTarget } from "./hooks/useDockviewDropTarget";
import { useDockviewEventHandlers } from "./hooks/useDockviewEventHandlers";
import { useDockviewLogger } from "./hooks/useDockviewLogger";
import { useDockviewResizeObserver } from "./hooks/useDockviewResizeObserver";
import Watermark from "./Watermark";

const DebugContext = React.createContext<boolean>(false);

const TabbedPane = ({ theme }: { theme?: string }) => {
  const { showDebugPanels } = useTabbedPaneStore();
  const { api, addOrFocusPanel, setApi, gridState, sendGridStateToBackend } = useTabbedPaneStore();

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
    event.api?.fromJSON(gridState);
    if (event.api.panels.length === 0) {
      event.api.addPanel({ id: "WelcomePage", component: "Welcome" });
    }
  };

  const onDidDrop = (event: DockviewDidDropEvent) => {
    if (!pragmaticDropElement || !api) return;

    addOrFocusPanel({
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

    const event = api.onDidLayoutChange(() => {
      sendGridStateToBackend(api.toJSON());
    });

    return () => event.dispose();
  }, [api, sendGridStateToBackend]);

  const components = {
    Default: (props: IDockviewPanelProps) => {
      const isDebug = React.useContext(DebugContext);

      return (
        <>
          <Breadcrumbs panelId={props.api.id} />
          <Scrollbar
            className={cn(
              "relative h-full overflow-auto p-1.25",
              isDebug && "border-2 border-dashed border-orange-500"
            )}
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
    Home: () => (
      <Scrollbar className="h-full">
        <Home />
      </Scrollbar>
    ),
    Settings: () => (
      <Scrollbar className="h-full">
        <Settings />
      </Scrollbar>
    ),
    Logs: () => (
      <Scrollbar className="h-full">
        <Logs />
      </Scrollbar>
    ),
    Welcome: () => (
      <Scrollbar className="h-full">
        <WelcomePage />
      </Scrollbar>
    ),
  };

  const headerComponents = {
    default: DockviewDefaultTab,
  };

  return (
    <Scrollbar className="h-full">
      <div className="dockview-demo relative flex h-full w-full grow flex-col rounded">
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
        <div className="flex h-full">
          <Scrollbar className="flex grow overflow-hidden">
            <DebugContext.Provider value={debug}>
              <div className="h-full w-full" ref={dockviewRefWrapper}>
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
              </div>
            </DebugContext.Provider>
          </Scrollbar>
          {showLogs && <LogsPanel logLines={logLines} onClear={() => setLogLines([])} />}
        </div>
      </div>
    </Scrollbar>
  );
};

export default TabbedPane;
