import TreeActionButton from "@/components/Tree/TreeActionButton";
import { useCollectionsStore } from "@/store/collections";

export const SidebarHeader = ({ title }: { title: string }) => {
  const { expandAll, collapseAll } = useCollectionsStore();

  return (
    <div className="background-(--moss-secondary-background) relative flex items-center justify-between px-2 py-[5px] font-semibold text-(--moss-primary-text) uppercase">
      <div className="flex items-center text-[12px] leading-2 font-semibold text-(--moss-secondary-text)">{title}</div>

      <div className="flex">
        <TreeActionButton icon="TreeExpandAll" onClick={expandAll} />
        <TreeActionButton icon="TreeCollapseAll" onClick={collapseAll} />

        <TreeActionButton icon="TreeReload" />
        <TreeActionButton icon="TreePlus" />
        <TreeActionButton icon="TreeDetail" />
      </div>
    </div>
  );
};

export default SidebarHeader;
