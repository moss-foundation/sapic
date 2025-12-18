import { cn } from "@/utils";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { PageContent, PageHeader, PageView } from "@/workbench/ui/components";
import {
  AccountsView,
  AccountsViewProps,
  EndpointView,
  EndpointViewProps,
  FolderSettingsView,
  FolderSettingsViewProps,
  KitchenSinkView,
  KitchenSinkViewProps,
  LogsView,
  LogsViewProps,
  ProjectSettingsView,
  ProjectSettingsViewProps,
  SettingsView,
  SettingsViewProps,
  WelcomeView,
  WelcomeViewProps,
  WorkspaceSettingsView,
  WorkspaceSettingsViewProps,
} from "@/workbench/views";

import Metadata from "./DebugComponents/Metadata";
import { DefaultViewProps } from "./types";

//NOTE: every component has to have a props type that extends DefaultViewProps for the addOrFocusPanel function's type checking to work correctly
export const tabbedPaneComponents = {
  DefaultView: (props: DefaultViewProps) => {
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
  EndpointView: (props: EndpointViewProps) => <EndpointView {...props} />,
  ProjectSettingsView: (props: ProjectSettingsViewProps) => <ProjectSettingsView {...props} />,
  FolderSettingsView: (props: FolderSettingsViewProps) => <FolderSettingsView {...props} />,
  WelcomeView: (props: WelcomeViewProps) => <WelcomeView {...props} />,
  WorkspaceSettingsView: (props: WorkspaceSettingsViewProps) => <WorkspaceSettingsView {...props} />,
  KitchenSinkView: (props: KitchenSinkViewProps) => <KitchenSinkView {...props} />,
  SettingsView: (props: SettingsViewProps) => <SettingsView {...props} />,
  AccountsView: (props: AccountsViewProps) => <AccountsView {...props} />,
  LogsView: (props: LogsViewProps) => <LogsView {...props} />,
};
