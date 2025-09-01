import { Icon } from "@/lib/ui/Icon";
import { cn } from "@/utils";

interface TreeRootNodeIconProps {
  handleIconClick: (e: React.MouseEvent<HTMLButtonElement>) => void;
  areChildrenShown: boolean;
  iconPath?: string;
}

export const TreeRootNodeIcon = ({ handleIconClick, areChildrenShown, iconPath }: TreeRootNodeIconProps) => {
  return (
    <span className="flex size-5 shrink-0 items-center justify-center">
      <button
        onClick={handleIconClick}
        className="hover:background-(--moss-icon-primary-background-hover) flex cursor-pointer items-center justify-center rounded-full"
      >
        <Icon
          icon="ChevronRight"
          className={cn("text-(--moss-icon-primary-text)", {
            "rotate-90": areChildrenShown,
            "hidden group-hover/treeRootNodeTrigger:block": iconPath,
          })}
        />
      </button>

      {iconPath && (
        <div className="h-full w-full rounded group-hover/treeRootNodeTrigger:hidden">
          <img src={iconPath} className="h-full w-full" />
        </div>
      )}
    </span>
  );
};
