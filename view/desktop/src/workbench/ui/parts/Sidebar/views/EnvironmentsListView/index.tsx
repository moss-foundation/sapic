import ErrorNaughtyDog from "@/assets/images/ErrorNaughtyDog.svg";
import { useGetAllProjectEnvironments } from "@/db/environmentsSummaries/hooks/useGetAllProjectEnvironments";
import { useGetWorkspaceEnvironments } from "@/db/environmentsSummaries/hooks/useGetWorkspaceEnvironments";
import { useSyncEnvironments } from "@/db/environmentsSummaries/hooks/useSyncEnvironments";
import { Icon, Scrollbar } from "@/lib/ui";
import { useTabbedPaneStore } from "@/workbench/store/tabbedPane";
import { ProjectEnvironmentsList } from "@/workbench/ui/components/EnvironmentsLists/ProjectEnvironmentsList/ProjectEnvironmentsList";
import { WorkspaceEnvironmentsList } from "@/workbench/ui/components/EnvironmentsLists/WorkspaceEnvironmentsList/WorkspaceEnvironmentsList";

import { EnvironmentsListItemPlaceholder } from "./EnvironmentsListItemPlaceholder";
import { EnvironmentsListViewDivider } from "./EnvironmentsListViewDivider";
import { EnvironmentsListViewHeader } from "./EnvironmentsListViewHeader";

export const EnvironmentsListView = () => {
  const { addOrFocusPanel } = useTabbedPaneStore();

  const { sortedWorkspaceEnvironmentsByOrder, isLoading: isWorkspaceEnvironmentsLoading } =
    useGetWorkspaceEnvironments();
  const { sortedProjectEnvironmentsByOrder, isLoading: isProjectEnvironmentsLoading } = useGetAllProjectEnvironments();

  const noWorkspaceEnvironments = sortedWorkspaceEnvironmentsByOrder?.length === 0;
  const noProjectEnvironments = sortedProjectEnvironmentsByOrder?.length === 0;

  const noEnvironments =
    noWorkspaceEnvironments &&
    noProjectEnvironments &&
    !isWorkspaceEnvironmentsLoading &&
    !isProjectEnvironmentsLoading;

  useSyncEnvironments();

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

        {isWorkspaceEnvironmentsLoading || isProjectEnvironmentsLoading ? (
          <div className="flex flex-col items-center justify-center gap-2 p-10 text-center">
            <span>Environments are loading... </span>
            <span>Please wait...</span>
            <Icon icon="Loader" className="size-6 animate-spin" />
          </div>
        ) : (
          <>
            {sortedWorkspaceEnvironmentsByOrder && sortedWorkspaceEnvironmentsByOrder.length > 0 && (
              <EnvironmentsListViewDivider />
            )}

            <WorkspaceEnvironmentsList />

            {sortedProjectEnvironmentsByOrder && sortedProjectEnvironmentsByOrder.length > 0 && (
              <EnvironmentsListViewDivider />
            )}

            <ProjectEnvironmentsList />
          </>
        )}

        {noEnvironments && (
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
