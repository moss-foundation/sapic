import { IDockviewPanelProps } from "moss-tabs";

import { cn } from "@/utils";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { PageContent, PageHeader, PageView } from "@/workbench/ui/components";
import { ProjectTreeNode } from "@/workbench/ui/components/ProjectTree/types";
import {
  EndpointView,
  FolderSettingsView,
  KitchenSinkView,
  LogsView,
  ProfileView,
  ProjectSettingsView,
  SettingsView,
  WelcomeView,
  WorkspaceSettingsView,
} from "@/workbench/views";
import { ResourceKind } from "@repo/moss-project";

import Metadata from "./DebugComponents/Metadata";

export const tabbedPaneComponents = {
  Default: (
    props: IDockviewPanelProps<{
      node?: ProjectTreeNode;
      projectId: string;
      iconType: ResourceKind;
    }>
  ) => {
    const { showDebugPanels } = useTabbedPaneStore();

    return (
      <PageView>
        <PageHeader icon="Placeholder" {...props} />
        <PageContent className={cn("relative", showDebugPanels && "border-2 border-dashed border-orange-500")}>
          <span className="pointer-events-none absolute left-1/2 top-1/2 flex -translate-x-1/2 -translate-y-1/2 transform flex-col opacity-50">
            <span className="text-[42px] leading-[42px]">Default Page</span>
            <span className="text-sm leading-3">This is a placeholder default page</span>
          </span>

          {showDebugPanels && (
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
  Endpoint: (
    props: IDockviewPanelProps<{
      node: ProjectTreeNode;
      projectId: string;
      iconType: ResourceKind;
    }>
  ) => <EndpointView {...props} />,
  ProjectSettings: (
    props: IDockviewPanelProps<{
      node: ProjectTreeNode;
      projectId: string;
      iconType: ResourceKind;
    }>
  ) => <ProjectSettingsView {...props} />,
  FolderSettings: (
    props: IDockviewPanelProps<{
      node: ProjectTreeNode;
      projectId: string;
      iconType: ResourceKind;
    }>
  ) => <FolderSettingsView {...props} />,
  Welcome: () => <WelcomeView />,
  WorkspaceSettings: (props: IDockviewPanelProps) => <WorkspaceSettingsView {...props} />,
  KitchenSink: () => <KitchenSinkView />,
  Settings: () => <SettingsView />,
  Profile: (props: IDockviewPanelProps) => <ProfileView {...props} />,
  Logs: () => <LogsView />,
};
