import {
  AccountsView,
  DefaultView,
  EndpointView,
  FolderSettingsView,
  KitchenSinkView,
  LogsView,
  ProjectSettingsView,
  SettingsView,
  WelcomeView,
  WorkspaceSettingsView,
} from "@/workbench/views";

//NOTE: every View component has to have a props type that extends DefaultViewProps
//for the addOrFocusPanel function's type checking to work correctly
export const tabbedPaneComponents = {
  DefaultView,
  EndpointView,
  ProjectSettingsView,
  FolderSettingsView,
  WelcomeView,
  WorkspaceSettingsView,
  KitchenSinkView,
  SettingsView,
  AccountsView,
  LogsView,
};
