import { CollectionEnvironmentsList } from "@/components/CollectionEnvironmentsList/CollectionEnvironmentsList";
import { GlobalEnvironmentsList } from "@/components/GlobalEnvironmentsList/GlobalEnvironmentsList";
import { useStreamEnvironments } from "@/hooks/environment";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { EnvironmentsListItem } from "./EnvironmentsListItem";
import { EnvironmentsListViewDivider } from "./EnvironmentsListViewDivider";
import { EnvironmentsListViewHeader } from "./EnvironmentsListViewHeader";

export const EnvironmentsListView = () => {
  const { addOrFocusPanel } = useTabbedPaneStore();
  const { data: environments } = useStreamEnvironments();

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

        {environments && environments.length > 0 && <EnvironmentsListViewDivider />}

        <GlobalEnvironmentsList />

        {environments && environments.length > 0 && <EnvironmentsListViewDivider />}

        <CollectionEnvironmentsList />
      </div>
    </div>
  );
};

export default EnvironmentsListView;
