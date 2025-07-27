import "./assets/styles.css";

import React from "react";

import { ActionButton, Breadcrumbs, PageContent, PageHeader, PageTabs, PageToolbar, PageView } from "@/components";
import { DropNode, TreeCollectionNode } from "@/components/CollectionTree/types";
import { useUpdateEditorPartState } from "@/hooks/appState/useUpdateEditorPartState";
import { mapEditorPartStateToSerializedDockview } from "@/hooks/appState/utils";
import { useActiveWorkspace } from "@/hooks/workspace/useActiveWorkspace";
import { useDescribeWorkspaceState } from "@/hooks/workspace/useDescribeWorkspaceState";
import { Icon, type Icons } from "@/lib/ui";
import { Scrollbar } from "@/lib/ui/Scrollbar";
import { CollectionSettingsPage, KitchenSink, Logs, Settings, WelcomePage, WorkspaceSettings } from "@/pages";
import { useRequestModeStore } from "@/store/requestMode";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import { EntryKind } from "@repo/moss-collection";
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
  component: React.ComponentType<IDockviewPanelProps>;
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
    if (pageKey === "WorkspaceSettings" && props.api && currentWorkspace?.name) {
      props.api.setTitle(currentWorkspace.name);
    }
  }, [currentWorkspace?.name, props.api, pageKey]);

  // Special case for full-page components (no title)
  if (!config.title) {
    return <PageComponent {...props} />;
  }

  // Standard page structure with header and content
  return (
    <PageView>
      <PageHeader icon={config.icon ? <Icon icon={config.icon} className="size-[18px]" /> : undefined} props={props} />
      <PageContent>
        <PageComponent {...props} />
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
  const activeWorkspace = useActiveWorkspace();

  const [panels, setPanels] = React.useState<string[]>([]);
  const [groups, setGroups] = React.useState<string[]>([]);
  const [activePanel, setActivePanel] = React.useState<string | undefined>();
  const [activeGroup, setActiveGroup] = React.useState<string | undefined>();
  const [pragmaticDropElement, setPragmaticDropElement] = React.useState<DropNode | null>(null);
  const [watermark, setWatermark] = React.useState(false);
  const [showLogs, setShowLogs] = React.useState(false);
  const [debug, setDebug] = React.useState(false);

  const dockviewRef = React.useRef<HTMLDivElement>(null);
  const dockviewRefWrapper = React.useRef<HTMLDivElement>(null);

  // Track when we're restoring layout to prevent automatic state updates
  const isRestoringLayout = React.useRef(false);

  const { canDrop } = useTabbedPaneDropTarget(dockviewRef, setPragmaticDropElement);

  useTabbedPaneEventHandlers(api, setPanels, setGroups, setActivePanel, setActiveGroup, canDrop);
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
        // Set flag to prevent automatic state updates during restoration
        isRestoringLayout.current = true;
        api.fromJSON(mapEditorPartStateToSerializedDockview(layout.editor));
        // Reset flag after a short delay to allow layout change events to settle
        setTimeout(() => {
          isRestoringLayout.current = false;
        }, 100);
      } else if (layout !== undefined) {
        // Layout data has been fetched but no editor state exists
        // This means it's a new workspace - ensure it starts empty
        console.log("Starting with empty TabbedPane for new workspace");
        isRestoringLayout.current = true;
        api.clear();
        setTimeout(() => {
          isRestoringLayout.current = false;
        }, 100);
      }
    } catch (error) {
      console.error("Failed to restore workspace layout:", error);
      isRestoringLayout.current = false;
    }
  }, [api, layout, mode]);

  const onDidDrop = (event: DockviewDidDropEvent) => {
    if (!pragmaticDropElement || !api) return;

    addOrFocusPanel({
      id: pragmaticDropElement.node.id,
      title: pragmaticDropElement.node.name,
      component: "Default",
      params: {
        treeId: pragmaticDropElement.node.id,
        iconType: pragmaticDropElement.node.kind,
        node: pragmaticDropElement.node,
      },
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
      // Only update workspace state if there's an active workspace
      if (activeWorkspace && !isRestoringLayout.current) {
        updateEditorPartState(api.toJSON());
      }
    });

    return () => event.dispose();
  }, [api, updateEditorPartState, activeWorkspace]);

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
    CollectionSettings: {
      title: "CollectionSettings",
      component: CollectionSettingsPage,
    },
  };

  const components = {
    Default: (
      props: IDockviewPanelProps<{
        node?: TreeCollectionNode;
        treeId: string;
        iconType: EntryKind;
        someRandomString: string;
      }>
    ) => {
      const { displayMode } = useRequestModeStore();

      const isDebug = React.useContext(DebugContext);

      let showEndpoint = false;
      let dontShowTabs = true;
      const [activeTab, setActiveTab] = React.useState(showEndpoint ? "endpoint" : "request");
      if (props.params?.node) {
        showEndpoint = displayMode === "DESIGN_FIRST" && props.params.node.class === "Endpoint";
        dontShowTabs =
          props.params.node.kind === "Dir" ||
          props.params.node.class === "Endpoint" ||
          props.params.node.class === "Schema";
      }

      const tabs = (
        <PageTabs>
          {showEndpoint && (
            <button data-active={activeTab === "endpoint"} onClick={() => setActiveTab("endpoint")}>
              Endpoint
            </button>
          )}
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
            icon={<Icon icon="Placeholder" className="size-[18px]" />}
            tabs={dontShowTabs ? null : tabs}
            toolbar={toolbar}
          />
          <PageContent className={cn("relative", isDebug && "border-2 border-dashed border-orange-500")}>
            <Breadcrumbs treeId={props.params?.treeId} nodeId={props.params?.node?.id} />

            <span className="pointer-events-none absolute top-1/2 left-1/2 flex -translate-x-1/2 -translate-y-1/2 transform flex-col text-[42px] opacity-50">
              {props.params?.node ? (
                <div>
                  <span className="text-[18px]">Node name: "{props.params.node.name}"</span>
                  <div className="pointer-events-auto max-h-[70vh] overflow-y-auto text-[12px]">
                    <pre>{JSON.stringify(props.params.node, null, 2)}</pre>
                  </div>
                </div>
              ) : (
                <>
                  <span>{props.api.title}</span>
                  <span>{Math.random().toFixed(2)}</span>
                  {props?.params.someRandomString && (
                    <span className="text-xs">some random string from backend: {props.params.someRandomString}</span>
                  )}
                </>
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
                  defaultTabComponent={CustomTab}
                  rightHeaderActionsComponent={PanelToolbar}
                  leftHeaderActionsComponent={AddPanelButton}
                  watermarkComponent={Watermark}
                  onReady={onReady}
                  className={theme || "dockview-theme-light"}
                  onDidDrop={onDidDrop}
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
