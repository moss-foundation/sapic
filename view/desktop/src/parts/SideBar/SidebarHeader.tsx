interface SidebarHeaderProps {
  title: string;
  actionsContent?: React.ReactNode;
}

export const SidebarHeader = ({ title, actionsContent }: SidebarHeaderProps) => {
  return (
    <div className="relative flex min-h-9 items-center justify-between px-2 text-(--moss-primary-text) uppercase">
      <div className="w-max items-center overflow-hidden text-xs text-ellipsis whitespace-nowrap text-(--moss-secondary-text)">
        {title}
      </div>

      <div className="flex grow justify-end gap-1">{actionsContent}</div>
    </div>
  );
};

export default SidebarHeader;
