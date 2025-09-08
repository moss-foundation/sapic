import { GlobalEnvironmentsList } from "@/components/EnvironmentsLists/GlobalEnvironmentsList/GlobalEnvironmentsList";
import { GroupedEnvironmentsList } from "@/components/EnvironmentsLists/GroupedEnvironmentsList/GroupedEnvironmentsList";
import { useMonitorEnvironmentsLists } from "@/components/EnvironmentsLists/hooks/useMonitorEnvironmentsLists";
import { useStreamEnvironments } from "@/hooks";
import { Scrollbar } from "@/lib/ui";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { EnvironmentsListItem } from "./EnvironmentsListItem";
import { EnvironmentsListViewDivider } from "./EnvironmentsListViewDivider";
import { EnvironmentsListViewHeader } from "./EnvironmentsListViewHeader";

export const EnvironmentsListView = () => {
  const { addOrFocusPanel } = useTabbedPaneStore();
  const { globalEnvironments, collectionEnvironments } = useStreamEnvironments();

  useMonitorEnvironmentsLists();

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

        {globalEnvironments && globalEnvironments.length > 0 && <EnvironmentsListViewDivider />}

        <GlobalEnvironmentsList />

        {collectionEnvironments && collectionEnvironments.length > 0 && <EnvironmentsListViewDivider />}

        <GroupedEnvironmentsList />
      </Scrollbar>
    </div>
  );
};

export default EnvironmentsListView;
