import { HTMLAttributes } from "react";

import { CollectionsList } from "@/components/CollectionsList/CollectionsList";
import { WorkspacesList } from "@/components/WorkspacesList/WorkspacesList";
import Icon, { Icons } from "@/lib/ui/Icon";
import { useTabbedPaneStore } from "@/store/tabbedPane";

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

const EnvironmentsListItem = ({
  icon,
  label,
  disabled,
  ...props
}: { icon: Icons; label: string; disabled?: boolean } & HTMLAttributes<HTMLButtonElement>) => {
  return (
    <button
      className="hover:background-(--moss-gray-12) flex w-full cursor-pointer items-center gap-2 px-2 py-1 disabled:cursor-not-allowed disabled:opacity-50 disabled:hover:bg-transparent"
      disabled={disabled}
      onClick={(e) => {
        if (disabled) {
          e.preventDefault();
          return;
        }

        props.onClick?.(e);
      }}
      {...props}
    >
      <Icon icon={icon} />
      <span>{label}</span>
    </button>
  );
};
