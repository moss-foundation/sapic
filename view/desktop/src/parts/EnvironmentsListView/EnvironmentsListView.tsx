import { GlobalEnvironmentsList } from "@/components/EnvironmentsLists/GlobalEnvironmentsList/GlobalEnvironmentsList";
import { GroupedEnvironmentsList } from "@/components/EnvironmentsLists/GroupedEnvironmentsList/GroupedEnvironmentsList";
import { useStreamEnvironments } from "@/hooks";
import { Scrollbar } from "@/lib/ui";
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

      <Scrollbar className="h-full">
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

        {environments && environments.environments.length > 0 && <EnvironmentsListViewDivider />}

        <GlobalEnvironmentsList />

        {environments && environments.environments.length > 0 && <EnvironmentsListViewDivider />}

        <GroupedEnvironmentsList />
      </Scrollbar>
    </div>
  );
};

export default EnvironmentsListView;
