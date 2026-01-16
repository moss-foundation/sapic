import { useStreamEnvironments } from "@/adapters";
import { useAllStreamedProjectEnvironments } from "@/adapters/tanstackQuery/environment/derived/useAllStreamedProjectEnvironments";
import ErrorNaughtyDog from "@/assets/images/ErrorNaughtyDog.svg";
import { Icon, Scrollbar } from "@/lib/ui";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { GlobalEnvironmentsList } from "@/workbench/ui/components/EnvironmentsLists/GlobalEnvironmentsList/GlobalEnvironmentsList";
import { GroupedEnvironmentsList } from "@/workbench/ui/components/EnvironmentsLists/GroupedEnvironmentsList/GroupedEnvironmentsList";
import { useMonitorEnvironmentsItems } from "@/workbench/ui/components/EnvironmentsLists/hooks/useMonitorEnvironmentsItems";
import { useMonitorEnvironmentsLists } from "@/workbench/ui/components/EnvironmentsLists/hooks/useMonitorEnvironmentsLists";

import { EnvironmentsListItemPlaceholder } from "./EnvironmentsListItemPlaceholder";
import { EnvironmentsListViewDivider } from "./EnvironmentsListViewDivider";
import { EnvironmentsListViewHeader } from "./EnvironmentsListViewHeader";

export const EnvironmentsListView = () => {
  const { addOrFocusPanel } = useTabbedPaneStore();
  const {
    data: workspaceEnvironments,
    isLoading: isWorkspaceEnvironmentsLoading,
    isFetched: isWorkspaceEnvironmentsFetched,
  } = useStreamEnvironments();
  const { allProjectEnvironments } = useAllStreamedProjectEnvironments();

  useMonitorEnvironmentsLists();
  useMonitorEnvironmentsItems();

  const noWorkspaceEnvironments = workspaceEnvironments?.length === 0;
  const noProjectEnvironments = allProjectEnvironments?.length === 0;

  const noEnvironments = noWorkspaceEnvironments && noProjectEnvironments;

  return (
    <div className="flex h-full flex-col">
      <EnvironmentsListViewHeader />

      <Scrollbar className="h-full">
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

        {isWorkspaceEnvironmentsLoading ? (
          <div className="flex flex-col items-center justify-center gap-2 p-10 text-center">
            <span>Environments are loading... </span>
            <span>Please wait...</span>
            <Icon icon="Loader" className="size-6 animate-spin" />
          </div>
        ) : (
          <>
            {workspaceEnvironments && workspaceEnvironments.length > 0 && <EnvironmentsListViewDivider />}

            <GlobalEnvironmentsList />

            {allProjectEnvironments && allProjectEnvironments.length > 0 && <EnvironmentsListViewDivider />}

            <GroupedEnvironmentsList />
          </>
        )}

        {isWorkspaceEnvironmentsFetched && noEnvironments && (
          <div className="px-2">
            <img src={ErrorNaughtyDog} className="pointer-events-none mx-auto h-auto w-full max-w-[200px]" />
            <p className="text-(--moss-secondary-foreground) text-center">You have no environments yet</p>
          </div>
        )}
      </Scrollbar>
    </div>
  );
};

export default EnvironmentsListView;
