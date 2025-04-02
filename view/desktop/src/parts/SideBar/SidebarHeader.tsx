import { Icon } from "@/components";
import { useCollectionsStore } from "@/store/collections";

const CollectionButtonStyle =
  "hover:background-(--moss-icon-primary-background-hover) rounded p-[5px] text-(--moss-icon-primary-text)";

export const SidebarHeader = ({ title }: { title: string }) => {
  const { expandAll, collapseAll } = useCollectionsStore();

  return (
    <div className="background-(--moss-secondary-background) flex items-center justify-between px-2 py-[5px] font-semibold text-(--moss-primary-text) uppercase">
      <div className="font-bold text-(--moss-secondary-text)">{title}</div>

      <div>
        <button className={CollectionButtonStyle} onClick={expandAll}>
          <Icon icon="TreeExpandAll" />
        </button>
        <button className={CollectionButtonStyle} onClick={collapseAll}>
          <Icon icon="TreeCollapseAll" />
        </button>
        <button className={CollectionButtonStyle}>
          <Icon icon="TreeReload" />
        </button>
        <button className={CollectionButtonStyle}>
          <Icon icon="TreePlus" />
        </button>
        <button className={CollectionButtonStyle}>
          <Icon icon="TreeDetail" />
        </button>
      </div>
    </div>
  );
};

export default SidebarHeader;
