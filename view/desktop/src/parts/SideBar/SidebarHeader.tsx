import TreeActionButton from "@/components/Tree/TreeActionButton";
import { useCollectionsStore } from "@/store/collections";

export const SidebarHeader = ({ title }: { title: string }) => {
  const { collapseAll } = useCollectionsStore();

  return (
    <div className="background-(--moss-secondary-background) relative flex items-center justify-between px-2 py-[5px] font-semibold text-(--moss-primary-text) uppercase">
      <div className="w-max items-center overflow-hidden text-[12px] font-semibold text-ellipsis whitespace-nowrap text-(--moss-secondary-text)">
        {title}
      </div>

      <div className="flex grow justify-end">
        <TreeActionButton icon="TreeCollapseAll" onClick={collapseAll} />

        <TreeActionButton icon="TreeReload" />
        <TreeActionButton icon="Plus" />
        <TreeActionButton icon="ThreeVerticalDots" />
      </div>
    </div>
  );
};

export default SidebarHeader;
