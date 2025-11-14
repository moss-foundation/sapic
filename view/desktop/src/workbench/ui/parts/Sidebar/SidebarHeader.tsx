interface SidebarHeaderProps {
  title: string;
  actionsContent?: React.ReactNode;
}

export const SidebarHeader = ({ title, actionsContent }: SidebarHeaderProps) => {
  return (
    <div className="text-(--moss-primary-foreground) relative flex min-h-9 items-center justify-between px-2 uppercase">
      <div className="text-(--moss-secondary-foreground) w-max items-center overflow-hidden text-ellipsis whitespace-nowrap text-xs">
        {title}
      </div>

      <div className="flex grow justify-end gap-1">{actionsContent}</div>
    </div>
  );
};
