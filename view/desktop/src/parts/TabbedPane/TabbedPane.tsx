import "./assets/styles.css";

import React, { useState } from "react";

import {
  ActionButton,
  Breadcrumbs,
  CollectionTree,
  PageContent,
  PageHeader,
  PageTabs,
  PageToolbar,
  PageView,
} from "@/components";
import ButtonPrimary from "@/components/ButtonPrimary";
import { DropNodeElement } from "@/components/CollectionTree/types";
import { useUpdateEditorPartState } from "@/hooks/appState/useUpdateEditorPartState";
import { mapEditorPartStateToSerializedDockview } from "@/hooks/appState/utils";
import { useActiveWorkspace } from "@/hooks/workspace/useActiveWorkspace";
import { useDescribeWorkspaceState } from "@/hooks/workspace/useDescribeWorkspaceState";
import { Icon, type Icons } from "@/lib/ui";
import { Scrollbar } from "@/lib/ui/Scrollbar";
import { KitchenSink, Logs, Settings, WelcomePage, WorkspaceSettings } from "@/pages";
import { useCollectionsStore } from "@/store/collections";
import { useRequestModeStore } from "@/store/requestMode";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import {
  DockviewDidDropEvent,
  DockviewReact,
  DockviewReadyEvent,
  IDockviewHeaderActionsProps,
  IDockviewPanelProps,
  positionToDirection,
} from "@repo/moss-tabs";

import { AddPanelButton } from "./AddPanelButton";
import CustomTab from "./CustomTab";
import DockviewControls from "./DebugComponents/DockviewControls";
import LogsPanel from "./DebugComponents/LogsPanel";
import Metadata from "./DebugComponents/Metadata";
import { useTabbedPaneDropTarget } from "./hooks/useDockviewDropTarget";
import { useTabbedPaneEventHandlers } from "./hooks/useDockviewEventHandlers";
import { useTabbedPaneResizeObserver } from "./hooks/useDockviewResizeObserver";
import ToolBar from "./ToolBar";
import Watermark from "./Watermark";

const DebugContext = React.createContext<boolean>(false);

type PageConfig = {
  title?: string;
  icon?: Icons;
  component: React.ComponentType;
};

// Wrapper component for pages with dynamic titles
const DynamicPageWrapper = ({
  pageKey,
  config,
  props,
}: {
  pageKey: string;
  config: PageConfig;
  props: IDockviewPanelProps;
}) => {
  const PageComponent = config.component;

  // Get fresh workspace data for dynamic title - must be called before any returns
  const currentWorkspace = useActiveWorkspace();

  // Update panel title dynamically for WorkspaceSettings - must be called before any returns
  React.useEffect(() => {
    if (pageKey === "WorkspaceSettings" && props.api && currentWorkspace?.displayName) {
      props.api.setTitle(currentWorkspace.displayName);
    }
  }, [currentWorkspace?.displayName, props.api, pageKey]);

  // Special case for full-page components (no title)
  if (!config.title) {
    return <PageComponent />;
  }

  let displayTitle = config.title;
  if (pageKey === "WorkspaceSettings" && currentWorkspace?.displayName) {
    displayTitle = currentWorkspace.displayName;
  }

  // Standard page structure with header and content
  return (
    <PageView>
      <PageHeader
        title={displayTitle}
        icon={config.icon ? <Icon icon={config.icon} className="size-[18px]" /> : undefined}
      />
      <PageContent>
        <PageComponent />
      </PageContent>
    </PageView>
  );
};

const PanelToolbar = (props: IDockviewHeaderActionsProps) => {
  const { api } = useTabbedPaneStore();
  const panelId = props.group.activePanel?.api.id;

  let isWorkspace = false;

  if (panelId && api) {
    const panel = api.getPanel(panelId);
    if (panel) {
      isWorkspace = panel.params?.workspace === true;
    }
  }

  return <ToolBar workspace={isWorkspace} />;
};

const TabbedPane = ({ theme, mode = "auto" }: { theme?: string; mode?: "auto" | "welcome" | "empty" }) => {
  const { showDebugPanels } = useTabbedPaneStore();
  const { api, addOrFocusPanel, setApi } = useTabbedPaneStore();

  const [panels, setPanels] = React.useState<string[]>([]);
  const [groups, setGroups] = React.useState<string[]>([]);
  const [activePanel, setActivePanel] = React.useState<string | undefined>();
  const [activeGroup, setActiveGroup] = React.useState<string | undefined>();
  const [pragmaticDropElement, setPragmaticDropElement] = React.useState<DropNodeElement | null>(null);
  const [watermark, setWatermark] = React.useState(false);
  const [showLogs, setShowLogs] = React.useState(false);
  const [debug, setDebug] = React.useState(false);

  const dockviewRef = React.useRef<HTMLDivElement>(null);
  const dockviewRefWrapper = React.useRef<HTMLDivElement>(null);

  useTabbedPaneEventHandlers(api, setPanels, setGroups, setActivePanel, setActiveGroup);
  const { canDrop, isDragging } = useTabbedPaneDropTarget(dockviewRef, setPragmaticDropElement);
  useTabbedPaneResizeObserver(api, dockviewRefWrapper);

  const { mutate: updateEditorPartState } = useUpdateEditorPartState();

  const shouldFetchWorkspaceState = mode === "auto" || mode === "empty";
  const { data: layout } = useDescribeWorkspaceState({
    enabled: shouldFetchWorkspaceState,
  });

  const onReady = (event: DockviewReadyEvent) => {
    setApi(event.api);

    try {
      if (mode === "welcome") {
        event.api.addPanel({ id: "WelcomePage", component: "Welcome" });
      } else if (mode === "empty") {
        console.log("Starting with empty TabbedPane for workspace");
      }
    } catch (error) {
      console.error("Failed to initialize TabbedPane:", error);

      const panels = [...event.api.panels];
      for (const panel of panels) {
        panel.api.close();
      }

      if (mode === "welcome" || mode === "auto") {
        event.api.addPanel({ id: "WelcomePage", component: "Welcome" });
      }
    }
  };

  React.useEffect(() => {
    if (!api || mode !== "auto") return;

    try {
      if (layout?.editor) {
        api.fromJSON(mapEditorPartStateToSerializedDockview(layout.editor));
      } else if (layout !== undefined) {
        // Layout data has been fetched but no editor state exists
        // This means it's a new workspace - ensure it starts empty
        console.log("Starting with empty TabbedPane for new workspace");
        api.clear();
      }
    } catch (error) {
      console.error("Failed to restore workspace layout:", error);
    }
  }, [api, layout, mode]);

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
      updateEditorPartState(api.toJSON());
    });

    return () => event.dispose();
  }, [api, updateEditorPartState]);

  const pageConfigs: Record<string, PageConfig> = {
    KitchenSink: {
      title: "KitchenSink",
      component: KitchenSink,
    },
    Settings: {
      title: "Settings",
      component: Settings,
    },
    Logs: {
      title: "Logs",
      component: Logs,
    },
    WorkspaceSettings: {
      title: "WorkspaceSettings", // This will be dynamically replaced
      component: WorkspaceSettings,
    },
    Welcome: {
      component: WelcomePage,
    },
  };

  const components = {
    Default: (props: IDockviewPanelProps) => {
      const isDebug = React.useContext(DebugContext);
      const [activeTab, setActiveTab] = React.useState("endpoint");

      const tabs = (
        <PageTabs>
          <button data-active={activeTab === "endpoint"} onClick={() => setActiveTab("endpoint")}>
            Endpoint
          </button>
          <button data-active={activeTab === "request"} onClick={() => setActiveTab("request")}>
            Request
          </button>
          <button data-active={activeTab === "mock"} onClick={() => setActiveTab("mock")}>
            Mock
          </button>
        </PageTabs>
      );

      const toolbar = (
        <PageToolbar>
          <ActionButton icon="MoreHorizontal" />
        </PageToolbar>
      );

      return (
        <PageView>
          <PageHeader
            title={props.api.title ?? "Untitled"}
            icon={<Icon icon="Placeholder" className="size-[18px]" />}
            tabs={tabs}
            toolbar={toolbar}
          />
          <PageContent className={cn("relative", isDebug && "border-2 border-dashed border-orange-500")}>
            <Breadcrumbs panelId={props.api.id} />

            <TestStreamedCollections />

            {/* <span className="pointer-events-none absolute top-1/2 left-1/2 flex -translate-x-1/2 -translate-y-1/2 transform flex-col text-[42px] opacity-50">
              <span>{props.api.title}</span>

              <span>{Math.random().toFixed(2)}</span>
              {props?.params.someRandomString && (
                <span className="text-xs">some random string from backend: {props.params.someRandomString}</span>
              )}
            </span> */}
            {isDebug && (
              <Metadata
                onClick={() => {
                  props.api.setRenderer(props.api.renderer === "always" ? "onlyWhenVisible" : "always");
                }}
                api={props.api}
              />
            )}
          </PageContent>
        </PageView>
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
    ...Object.entries(pageConfigs).reduce(
      (acc, [key, config]) => {
        acc[key] = (props: IDockviewPanelProps) => <DynamicPageWrapper pageKey={key} config={config} props={props} />;
        return acc;
      },
      {} as Record<string, (props: IDockviewPanelProps) => JSX.Element>
    ),
  };

  const headerComponents = {
    default: CustomTab,
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
                  rightHeaderActionsComponent={PanelToolbar}
                  leftHeaderActionsComponent={AddPanelButton}
                  watermarkComponent={Watermark}
                  onReady={onReady}
                  className={theme || "dockview-theme-light"}
                  onDidDrop={onDidDrop}
                  disableDnd={isDragging && canDrop === false}
                />
              </div>
            </DebugContext.Provider>
          </Scrollbar>
          {showLogs && <LogsPanel />}
        </div>
      </div>
    </Scrollbar>
  );
};

export default TabbedPane;

const TestStreamedCollections = () => {
  const {
    streamedCollections,
    collectionsTrees,
    updateCollectionTree,
    areCollectionsStreaming,
    areCollectionEntriesStreaming,
    createCollectionEntry,
  } = useCollectionsStore();
  const [path, setPath] = useState<string>("");
  const [name, setName] = useState<string>("");

  const handleClick = (id: string) => {
    if (!name || !path) return;

    createCollectionEntry({
      collectionId: id,
      input: {
        item: {
          name,
          path,
          configuration: {
            request: {
              http: {
                requestParts: {
                  method: "GET",
                },
              },
            },
          },
        },
      },
    });
  };

  const handleClickToDir = (id: string) => {
    if (!name || !path) return;

    createCollectionEntry({
      collectionId: id,
      input: {
        dir: {
          name,
          path,
          configuration: {
            request: {
              http: {},
            },
          },
        },
      },
    });
  };
  const shouldShowCollectionTree = !areCollectionsStreaming && !areCollectionEntriesStreaming;

  const { displayMode } = useRequestModeStore();

  return (
    <div className="flex max-w-100 flex-col gap-2">
      <div className="flex grow flex-col">
        {shouldShowCollectionTree &&
          collectionsTrees.map((collection) => (
            <CollectionTree
              key={`${collection.id}`}
              tree={collection}
              onTreeUpdate={updateCollectionTree}
              displayMode={displayMode}
            />
          ))}
      </div>

      <hr />

      <div className="grid grid-cols-2 gap-2">
        <input
          className="w-full border-2 border-dashed border-gray-300"
          type="text"
          placeholder="name"
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
        <input
          className="w-full border-2 border-dashed border-gray-300"
          type="text"
          placeholder="path"
          value={path}
          onChange={(e) => setPath(e.target.value)}
        />
      </div>

      <div className={cn("flex gap-2")}>
        <div>loading: {areCollectionsStreaming.toString()}</div>
        <div>Entries loading: {areCollectionEntriesStreaming.toString()}</div>
      </div>

      <div className="grid grid-cols-2 gap-2">
        {streamedCollections?.map((collection) => (
          <React.Fragment key={collection.id}>
            <ButtonPrimary onClick={() => handleClick(collection.id)}>add to "{collection.name}"</ButtonPrimary>
            <ButtonPrimary onClick={() => handleClickToDir(collection.id)}>
              add dir to "{collection.name}"
            </ButtonPrimary>
          </React.Fragment>
        ))}
      </div>

      {/* <div>
        {streamedCollectionEntries?.map((entry) => (
          <ButtonPrimary
            key={entry.id}
            onClick={() => {
              deleteCollectionEntry({
                collectionId: "7e353d76-8894-4007-a6da-2c96d9951eb7",
                input: { id: entry.id, path: entry.path },
              });
            }}
          >
            delete {entry.path}
          </ButtonPrimary>
        ))}
      </div> */}
      <pre className="text-xs">{JSON.stringify(collectionsTrees, null, 2)}</pre>
    </div>
  );
};
