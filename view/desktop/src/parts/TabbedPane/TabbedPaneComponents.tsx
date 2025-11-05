import { IDockviewPanelProps } from "moss-tabs";

import { PageContent, PageHeader, PageView } from "@/components";
import { ProjectTreeNode } from "@/components/ProjectTree/types";
import {
  EndpointPage,
  FolderSettings,
  KitchenSink,
  Logs,
  ProfilePage,
  ProjectSettingsPage,
  Settings,
  WelcomePage,
  WorkspaceSettings,
} from "@/pages";
import { useTabbedPaneStore } from "@/store/tabbedPane";
import { cn } from "@/utils";
import { ResourceKind } from "@repo/moss-project";

import Metadata from "./DebugComponents/Metadata";

export const TabbedPaneComponents = {
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
  ) => <EndpointPage {...props} />,
  ProjectSettings: (
    props: IDockviewPanelProps<{
      node: ProjectTreeNode;
      projectId: string;
      iconType: ResourceKind;
    }>
  ) => <ProjectSettingsPage {...props} />,
  FolderSettings: (
    props: IDockviewPanelProps<{
      node: ProjectTreeNode;
      projectId: string;
      iconType: ResourceKind;
    }>
  ) => <FolderSettings {...props} />,
  Welcome: () => <WelcomePage />,
  WorkspaceSettings: (props: IDockviewPanelProps) => <WorkspaceSettings {...props} />,
  KitchenSink: () => <KitchenSink />,
  Settings: () => <Settings />,
  Profile: (props: IDockviewPanelProps) => <ProfilePage {...props} />,
  Logs: () => <Logs />,
};
