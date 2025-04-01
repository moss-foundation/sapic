import { Icon } from "@/components";

export const SidebarHeader = ({ title }: { title: string }) => {
  return (
    <div className="flex items-center justify-between bg-[var(--moss-sideBar-header-background)] px-[15px] py-[10px] font-semibold text-[var(--moss-sideBar-header-text)] uppercase">
      <span>{title}</span>
      <button className="rounded p-1 hover:bg-[var(--moss-sideBar-header-button-hover)]">
        <Icon icon="TreeDetailIcon" />
      </button>
    </div>
  );
};

export default SidebarHeader;
