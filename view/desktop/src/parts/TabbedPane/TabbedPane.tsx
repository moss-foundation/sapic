import React from "react";

import { Scrollbar } from "@/components/Scrollbar";
import { Home, Logs, Settings } from "@/pages";
import {
  DockviewApi,
  DockviewDefaultTab,
  DockviewDidDropEvent,
  DockviewReact,
  DockviewReadyEvent,
  IDockviewPanelHeaderProps,
  IDockviewPanelProps,
  positionToDirection,
} from "@repo/moss-tabs";

import { LeftControls, PrefixHeaderControls, RightControls } from "./controls";
import { Table, usePanelApiMetadata } from "./debugPanel";
import { defaultConfig } from "./defaultLayout";
import { GridActions } from "./gridActions";
import { GroupActions } from "./groupActions";
import { PanelActions } from "./panelActions";
import { setGridState } from "./utils";

import "./assets/styles.css";

import { DropNodeElement } from "@/components/Tree/types";
import { getActualDropSourceTarget } from "@/components/Tree/utils";
import { useDockviewStore } from "@/store/Dockview";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

const DebugContext = React.createContext<boolean>(false);

const Option = (props: { title: string; onClick: () => void; value: string }) => {
  return (
    <div>
      <span>{`${props.title}: `}</span>
      <button className="rounded !bg-cyan-300 p-2 !text-black hover:!bg-cyan-500" onClick={props.onClick}>
        {props.value}
      </button>
    </div>
  );
};

const components = {
  Default: (props: IDockviewPanelProps) => {
    const isDebug = React.useContext(DebugContext);
    const metadata = usePanelApiMetadata(props.api);

    return (
      <Scrollbar
        className={`relative h-full overflow-auto p-1.25 ${isDebug ? "border-2 border-dashed border-orange-500" : ""}`}
      >
        <span className="pointer-events-none absolute top-1/2 left-1/2 flex -translate-x-1/2 -translate-y-1/2 transform flex-col text-[42px] opacity-50">
          {props.api.title}
          {props?.params.someRandomString && (
            <span className="text-xs">some random string from backend: {props.params.someRandomString}</span>
          )}
        </span>

        {isDebug && (
          <div className="text-sm">
            <Option
              title="Panel Rendering Mode"
              value={metadata.renderer.value}
              onClick={() => {
                props.api.setRenderer(props.api.renderer === "always" ? "onlyWhenVisible" : "always");
              }}
            />

            <Table data={metadata} />
          </div>
        )}
      </Scrollbar>
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
  Home: (props: IDockviewPanelProps) => {
    return RenderPage(props, Home);
  },
  Settings: (props: IDockviewPanelProps) => {
    return RenderPage(props, Settings);
  },
  Logs: (props: IDockviewPanelProps) => {
    return RenderPage(props, Logs);
  },
};

function RenderPage(props: IDockviewPanelProps, page: React.FC) {
  const isDebug = React.useContext(DebugContext);
  const metadata = usePanelApiMetadata(props.api);

  React.useEffect(() => {
    setGridState(props.containerApi);
  }, [props.containerApi]);

  return (
    <Scrollbar
      className={`relative h-full overflow-auto p-1.25 ${isDebug ? "border-2 border-dashed border-orange-500" : ""}`}
    >
      <span>{React.createElement(page)}</span>

      {isDebug && (
        <div className="text-sm">
          <Option
            title="Panel Rendering Mode"
            value={metadata.renderer.value}
            onClick={() => {
              props.api.setRenderer(props.api.renderer === "always" ? "onlyWhenVisible" : "always");
            }}
          />

          <Table data={metadata} />
        </div>
      )}
    </Scrollbar>
  );
}

const headerComponents = {
  default: (props: IDockviewPanelHeaderProps) => {
    const onContextMenu = (event: React.MouseEvent) => {
      event.preventDefault();
      alert("context menu");
    };
    return <DockviewDefaultTab onContextMenu={onContextMenu} {...props} />;
  },
};

const colors = [
  "rgba(255,0,0,0.2)",
  "rgba(0,255,0,0.2)",
  "rgba(0,0,255,0.2)",
  "rgba(255,255,0,0.2)",
  "rgba(0,255,255,0.2)",
  "rgba(255,0,255,0.2)",
];
let count = 0;

const WatermarkComponent = () => {
  return <div className="bg-red-200">Custom Watermark</div>;
};

const TabbedPane = (props: { theme?: string }) => {
  const [logLines, setLogLines] = React.useState<{ text: string; timestamp?: Date; backgroundColor?: string }[]>([]);
  const { showDebugPanels } = useTabbedPaneStore();

  const dockviewStore = useDockviewStore();

  const [panels, setPanels] = React.useState<string[]>([]);
  const [groups, setGroups] = React.useState<string[]>([]);
  const [api, setApi] = React.useState<DockviewApi>();

  const [activePanel, setActivePanel] = React.useState<string>();
  const [activeGroup, setActiveGroup] = React.useState<string>();

  const [pending, setPending] = React.useState<{ text: string; timestamp?: Date }[]>([]);

  const addLogLine = (message: string) => {
    setPending((line) => [{ text: message, timestamp: new Date() }, ...line]);
  };

  React.useLayoutEffect(() => {
    if (pending.length === 0) {
      return;
    }
    const color = colors[count++ % colors.length];
    setLogLines((lines) => [...pending.map((_) => ({ ..._, backgroundColor: color })), ...lines]);
    setPending([]);
  }, [pending]);

  React.useEffect(() => {
    if (!api) {
      return;
    }

    const disposables = [
      api.onDidAddPanel((event) => {
        setPanels((_) => [..._, event.id]);
        addLogLine(`Panel Added ${event.id}`);
      }),
      api.onDidActivePanelChange((event) => {
        console.log("active panel change", event);
        dockviewStore.setCurrentActivePanelId(event?.id || undefined);
        addLogLine(`Panel Activated ${event?.id}`);
      }),
      api.onDidRemovePanel((event) => {
        setPanels((_) => {
          const next = [..._];
          next.splice(
            next.findIndex((x) => x === event.id),
            1
          );

          return next;
        });
        addLogLine(`Panel Removed ${event.id}`);
      }),

      api.onDidAddGroup((event) => {
        setGroups((_) => [..._, event.id]);
        addLogLine(`Group Added ${event.id}`);
      }),

      api.onDidMovePanel((event) => {
        addLogLine(`Panel Moved ${event.panel.id}`);
      }),

      api.onDidMaximizedGroupChange((event) => {
        addLogLine(`Group Maximized Changed ${event.group.api.id} [${event.isMaximized}]`);
      }),

      api.onDidRemoveGroup((event) => {
        setGroups((_) => {
          const next = [..._];
          next.splice(
            next.findIndex((x) => x === event.id),
            1
          );

          return next;
        });
        addLogLine(`Group Removed ${event.id}`);
      }),

      api.onDidActiveGroupChange((event) => {
        setActiveGroup(event?.id);
        addLogLine(`Group Activated ${event?.id}`);
      }),
      api.onUnhandledDragOverEvent((event) => {
        event.accept();
      }),
    ];
    const loadLayout = () => {
      defaultConfig(api);
      api.clear();
      defaultConfig(api);
    };

    loadLayout();
    return () => {
      disposables.forEach((disposable) => disposable.dispose());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api]);

  const onReady = (event: DockviewReadyEvent) => {
    setApi(event.api);
    dockviewStore.setApi(event.api);
    useTabbedPaneStore.getState().setApi(event.api);
  };

  const dockviewRef = React.useRef<HTMLDivElement | null>(null);
  const [pragmaticDropElement, setPragmaticDropElement] = React.useState<DropNodeElement | null>(null);

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
    if (!dockviewRef.current) {
      return;
    }
    return dropTargetForElements({
      element: dockviewRef.current,
      onDragEnter({ source }) {
        const sourceTarget = getActualDropSourceTarget(source);
        if (source) setPragmaticDropElement(sourceTarget);
      },
      canDrop({ source }) {
        console.log(source);
        return source?.data?.type === "TreeNode";
      },

      onDragLeave() {
        setPragmaticDropElement(null);
      },
    });
  }, [dockviewRef, dockviewStore]);

  const [watermark, setWatermark] = React.useState<boolean>(false);

  const [gapCheck, setGapCheck] = React.useState<boolean>(false);

  const css = React.useMemo(() => {
    if (!gapCheck) {
      return {};
    }

    return {
      "--moss-group-gap-size": "0.5rem",
      "--demo-border": "5px dashed purple",
    } as React.CSSProperties;
  }, [gapCheck]);

  const [showLogs, setShowLogs] = React.useState<boolean>(false);
  const [debug, setDebug] = React.useState<boolean>(false);

  return (
    <div
      className="dockview-demo relative flex h-full grow flex-col rounded bg-[rgba(0,0,50,0.25)]"
      style={{
        ...css,
      }}
    >
      {showDebugPanels && (
        <>
          <div>
            <GridActions
              api={api}
              toggleCustomWatermark={() => setWatermark(!watermark)}
              hasCustomWatermark={watermark}
            />
            {api && <PanelActions api={api} panels={panels} activePanel={activePanel} />}
            {api && <GroupActions api={api} groups={groups} activeGroup={activeGroup} />}
            {/* <div>
                  <button
                      onClick={() => {
                          setGapCheck(!gapCheck);
                      }}
                  >
                      {gapCheck ? 'Disable Gap Check' : 'Enable Gap Check'}
                  </button>
              </div> */}
          </div>
          <div className="action-container mb-2 flex items-center justify-end p-1">
            <button
              className="mr-2 rounded"
              onClick={() => {
                setDebug(!debug);
              }}
            >
              <span className="material-symbols-outlined">engineering</span>
            </button>
            {showLogs && (
              <button
                className="mr-1 rounded"
                onClick={() => {
                  setLogLines([]);
                }}
              >
                <span className="material-symbols-outlined">undo</span>
              </button>
            )}
            <button
              className="rounded p-1"
              onClick={() => {
                setShowLogs(!showLogs);
              }}
            >
              <span className="pr-1">{`${showLogs ? "Hide" : "Show"} Events Log`}</span>
              <span className="material-symbols-outlined">terminal</span>
            </button>
          </div>
        </>
      )}
      <div className="flex h-0 grow">
        <Scrollbar className="flex h-full grow overflow-hidden">
          <DebugContext.Provider value={debug}>
            <DockviewReact
              ref={dockviewRef}
              components={components}
              defaultTabComponent={headerComponents.default}
              rightHeaderActionsComponent={RightControls}
              leftHeaderActionsComponent={LeftControls}
              prefixHeaderActionsComponent={PrefixHeaderControls}
              watermarkComponent={watermark ? WatermarkComponent : undefined}
              onReady={onReady}
              className={props.theme || "dockview-theme-light"}
              onDidDrop={onDidDrop}
            />
          </DebugContext.Provider>
        </Scrollbar>

        {showLogs && (
          <div className="ml-2 flex w-[400px] shrink-0 flex-col overflow-hidden bg-black font-mono text-white">
            <Scrollbar className="grow overflow-auto">
              {logLines.map((line, i) => {
                return (
                  <div
                    className="flex h-[30px] items-center overflow-hidden text-[13px] text-ellipsis whitespace-nowrap"
                    style={{
                      backgroundColor: line.backgroundColor,
                    }}
                    key={i}
                  >
                    <span className="mr-1 flex h-full max-w-[20px] min-w-[20px] items-center border-r border-gray-500 pl-1 text-gray-500">
                      {logLines.length - i}
                    </span>
                    <span>
                      {line.timestamp && (
                        <span className="px-[2px] text-[0.7em]">{line.timestamp.toISOString().substring(11, 23)}</span>
                      )}
                      <span>{line.text}</span>
                    </span>
                  </div>
                );
              })}
            </Scrollbar>
            <Scrollbar className="flex justify-end p-1">
              <button
                onClick={() => {
                  setLogLines([]);
                }}
              >
                Clear
              </button>
            </Scrollbar>
          </div>
        )}
      </div>
    </div>
  );
};

export default TabbedPane;
