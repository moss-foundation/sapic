import "./assets/styles.css";

import React from "react";

import { Breadcrumbs, PageContent, PageHeader, PageView } from "@/components";
import { DropNode, ProjectTreeNode } from "@/components/ProjectTree/types";
import { useUpdateEditorPartState } from "@/hooks/app/useUpdateEditorPartState";
import { mapEditorPartStateToSerializedDockview } from "@/hooks/app/utils";
import { useActiveWorkspace, useDescribeWorkspaceState } from "@/hooks/workspace";
import {
  FolderSettings,
  KitchenSink,
  Logs,
  ProjectSettingsPage,
  RequestPage,
  Settings,
  WelcomePage,
  WorkspaceSettings,
} from "@/pages";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import { EntryKind } from "@repo/moss-project";
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
  const { hasActiveWorkspace } = useActiveWorkspace();

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

    //TODO: this is a hardcoded component, later we we will need to have a more generic way to handle this
    addOrFocusPanel({
      id: pragmaticDropElement.node.id,
      title: pragmaticDropElement.node.name,
      component: "Request",
      params: {
        projectId: pragmaticDropElement.collectionId,
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
      if (hasActiveWorkspace && !isRestoringLayout.current) {
        updateEditorPartState(api.toJSON());
      }
    });

    return () => event.dispose();
  }, [api, updateEditorPartState, hasActiveWorkspace]);

  const components = {
    Default: (
      props: IDockviewPanelProps<{
        node?: ProjectTreeNode;
        collectionId: string;
        iconType: EntryKind;
      }>
    ) => {
      const isDebug = React.useContext(DebugContext);

      return (
        <PageView>
          <PageHeader icon="Placeholder" {...props} />
          <PageContent className={cn("relative", isDebug && "border-2 border-dashed border-orange-500")}>
            {props.params?.collectionId && props.params?.node?.id && (
              <Breadcrumbs collectionId={props.params.collectionId} nodeId={props.params.node.id} />
            )}

            <span className="pointer-events-none absolute top-1/2 left-1/2 flex -translate-x-1/2 -translate-y-1/2 transform flex-col opacity-50">
              <span className="text-[42px] leading-[42px]">Default Page</span>
              <span className="text-sm leading-3">This is a placeholder default page</span>
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
    Request: (
      props: IDockviewPanelProps<{
        node: ProjectTreeNode;
        collectionId: string;
        iconType: EntryKind;
      }>
    ) => <RequestPage {...props} />,
    ProjectSettings: (
      props: IDockviewPanelProps<{
        node: ProjectTreeNode;
        collectionId: string;
        iconType: EntryKind;
      }>
    ) => <ProjectSettingsPage {...props} />,
    FolderSettings: (
      props: IDockviewPanelProps<{
        node: ProjectTreeNode;
        collectionId: string;
        iconType: EntryKind;
      }>
    ) => <FolderSettings {...props} />,
    Welcome: () => <WelcomePage />,
    WorkspaceSettings: (props: IDockviewPanelProps) => <WorkspaceSettings {...props} />,
    KitchenSink: () => <KitchenSink />,
    Settings: () => <Settings />,
    Logs: () => <Logs />,
  };

  return (
    <div className="h-full">
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
          <div className="flex grow overflow-hidden">
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
          </div>
          {showLogs && <LogsPanel />}
        </div>
      </div>
    </div>
  );
};

export default TabbedPane;
