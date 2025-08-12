import { CollectionsList } from "@/components/CollectionsList/CollectionsList";
import { WorkspacesList } from "@/components/WorkspacesList/WorkspacesList";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { EnvironmentsListItem } from "./EnvironmentsListItem";
import { EnvironmentsListViewDivider } from "./EnvironmentsListViewDivider";
import { EnvironmentsListViewHeader } from "./EnvironmentsListViewHeader";

export const EnvironmentsListView = () => {
  const { addOrFocusPanel } = useTabbedPaneStore();

  return (
    <div className="flex h-full flex-col">
      <EnvironmentsListViewHeader />

      <div className="h-full">
        <EnvironmentsListItem
          icon="Vault"
          label="Vault"
          title="Vaults coming soon..."
          disabled={true}
          onClick={() => {
            addOrFocusPanel({
              id: "Vault",
              component: "Default",
            });
          }}
        />

        <EnvironmentsListViewDivider />

        <WorkspacesList />

        <EnvironmentsListViewDivider />

        <CollectionsList />
      </div>
    </div>
  );
};

export default EnvironmentsListView;
