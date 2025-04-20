import { Divider, Icon } from "@/components";

export const ToolBar = () => {
  return (
    <div className="group-control flex h-full items-center px-2 select-none">
      <div className="cursor-pointer rounded p-1 hover:bg-[var(--moss-icon-primary-background-hover)]">
        <Icon icon="ThreeVerticalDots" className="text-[var(--moss-icon-primary-text)]" />
      </div>
      <Divider height="20px" />
    </div>
  );
};
