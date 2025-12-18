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
import { DefaultView } from "@/workbench/views/DefaultView/DefaultView";

import { DefaultViewProps } from "./types";

//NOTE: every component has to have a props type that extends DefaultViewProps for the addOrFocusPanel function's type checking to work correctly
export const tabbedPaneComponents = {
  DefaultView: (props: DefaultViewProps) => <DefaultView {...props} />,
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
