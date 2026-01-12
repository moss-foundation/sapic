import { useGetLayout } from "@/workbench/adapters";
import { useActivityBarStore } from "@/workbench/store/activityBar";

interface SidebarHeaderProps {
  toolbar?: React.ReactNode;
}

export const SidebarHeader = ({ toolbar }: SidebarHeaderProps) => {
  const { data: layout } = useGetLayout();
  const { items } = useActivityBarStore();
  const activeContainerId = layout?.activitybarState.activeContainerId;

  const activeItem = items.find((item) => item.id === activeContainerId);
  const title = activeItem?.title || "";

  return (
    <div className="text-(--moss-primary-foreground) relative flex min-h-9 items-center px-2">
      <div className="text-(--moss-secondary-foreground) min-w-0 flex-1 overflow-hidden text-ellipsis whitespace-nowrap text-xs uppercase">
        {title}
      </div>
      {toolbar && (
        <div className="background-(--moss-primary-background) relative z-10 flex shrink-0 items-center gap-1 pl-2">
          {toolbar}
        </div>
      )}
    </div>
  );
};
