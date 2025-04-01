import { Icon } from "@/components";

export const SidebarHeader = ({ title }: { title: string }) => {
  return (
    <div className="background-(--moss-secondary-background) flex items-center justify-between px-[15px] py-[10px] font-semibold text-(--moss-primary-text) uppercase">
      <span>{title}</span>
      <button className="hover:background-(--moss-icon-primary-background-hover) rounded p-1 text-(--moss-icon-primary-text)">
        <Icon icon="TreeDetailIcon" />
      </button>
    </div>
  );
};

export default SidebarHeader;
