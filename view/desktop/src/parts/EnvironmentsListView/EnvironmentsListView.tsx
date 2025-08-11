import { HTMLAttributes } from "react";

import { WorkspaceList } from "@/components/WorkspaceList/WorkspaceList";
import { useStreamEnvironments } from "@/hooks/environment";
import Icon, { Icons } from "@/lib/ui/Icon";
import { useTabbedPaneStore } from "@/store/tabbedPane";

import { EnvironmentsListViewDivider } from "./EnvironmentsListViewDivider";
import { EnvironmentsListViewHeader } from "./EnvironmentsListViewHeader";

export const EnvironmentsListView = () => {
  const { data: environments } = useStreamEnvironments();
  const { addOrFocusPanel } = useTabbedPaneStore();

  return (
    <div className="flex h-full flex-col gap-4">
      <EnvironmentsListViewHeader />

      <div className="h-full">
        <EnvironmentsListItem
          icon="Vault"
          label="Vault"
          onClick={() => {
            addOrFocusPanel({
              id: "Vault",
              component: "Default",
            });
          }}
        />

        <EnvironmentsListViewDivider />

        <WorkspaceList />

        <EnvironmentsListViewDivider />

        <pre>{JSON.stringify(environments, null, 2)}</pre>
      </div>
    </div>
  );
};

export default EnvironmentsListView;

const EnvironmentsListItem = ({
  icon,
  label,
  ...props
}: { icon: Icons; label: string } & HTMLAttributes<HTMLButtonElement>) => {
  return (
    <button
      className="hover:background-(--moss-gray-12) flex w-full cursor-pointer items-center gap-2 px-2 py-1"
      {...props}
    >
      <Icon icon={icon} />
      <span>{label}</span>
    </button>
  );
};
