import { ActionButton } from "@/components";
import { useCollectionsStore } from "@/store/collections";

export const SidebarHeader = ({ title }: { title: string }) => {
  const { collapseAll } = useCollectionsStore();

  return (
    <div className="background-(--moss-secondary-background) relative flex items-center justify-between px-2 py-[5px] text-(--moss-primary-text) uppercase">
      <div className="w-max items-center overflow-hidden text-xs text-ellipsis whitespace-nowrap text-(--moss-secondary-text)">
        {title}
      </div>

      <div className="flex grow justify-end">
        <ActionButton icon="PlusButton" />
        <ActionButton icon="TreeCollapseAll" onClick={collapseAll} />
        <ActionButton icon="Import" />
        <ActionButton icon="ThreeVerticalDots" />
      </div>
    </div>
  );
};

export default SidebarHeader;
