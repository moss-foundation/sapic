import { Divider, Icon, type Icons } from "@/components";

interface ToolBarButtonProps {
  leftIcon: Icons;
  rightIcon: Icons;
  title: string;
  className?: string;
}

const ToolBarButton = ({ leftIcon, rightIcon, title, className }: ToolBarButtonProps) => {
  return (
    <div
      className={`group cursor-pointer rounded p-1 hover:bg-[var(--moss-icon-primary-background-hover)] ${className}`}
    >
      <div className="flex items-center gap-1">
        <Icon
          icon={leftIcon}
          className="mr-[2px] size-[18px] text-[var(--moss-icon-primary-text)] group-hover:text-black"
        />
        <span className="text-xs text-[var(--moss-icon-primary-text)] opacity-70 group-hover:text-black group-hover:opacity-100">
          {title}
        </span>
        <Icon
          icon={rightIcon}
          className="text-[var(--moss-icon-primary-text)] opacity-70 group-hover:text-black group-hover:opacity-100"
        />
      </div>
    </div>
  );
};

export const ToolBar = () => {
  return (
    <div className="group-control flex h-full items-center px-2 select-none">
      <div className="group cursor-pointer rounded p-1 hover:bg-[var(--moss-icon-primary-background-hover)]">
        <Icon icon="ThreeVerticalDots" className="text-[var(--moss-icon-primary-text)] group-hover:text-black" />
      </div>
      <Divider height="large" className="mr-[10px]" />
      <ToolBarButton leftIcon="ToolBarEnvironment" rightIcon="ChevronDown" title="No environment" />
      <div className="group mr-[10px] ml-[3px] cursor-pointer rounded p-1 hover:bg-[var(--moss-icon-primary-background-hover)]">
        <Icon icon="ToolBarVariables" className="text-[var(--moss-icon-primary-text)] group-hover:text-black" />
      </div>
    </div>
  );
};
