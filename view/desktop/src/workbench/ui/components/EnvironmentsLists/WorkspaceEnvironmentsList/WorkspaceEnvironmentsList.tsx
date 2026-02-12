import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { EnvironmentsListItemPlaceholder } from "@/workbench/ui/components/EnvironmentsLists/EnvironmentsListItemPlaceholder";

import { WorkspaceEnvironmentsListRoot } from "./WorkspaceEnvironmentsListRoot";

export const WorkspaceEnvironmentsList = () => {
  const { addOrFocusPanel } = useTabbedPaneStore();

  return (
    <div>
      <EnvironmentsListItemPlaceholder
        icon="Vault"
        label="Vault"
        title="Vaults coming soon..."
        disabled={true}
        onClick={() => {
          addOrFocusPanel({
            id: "Vault",
            component: "DefaultView",
          });
        }}
      />

      <WorkspaceEnvironmentsListRoot />
    </div>
  );
};
