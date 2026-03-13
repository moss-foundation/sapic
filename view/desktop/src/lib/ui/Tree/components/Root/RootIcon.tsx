import { Icon } from "@/lib/ui/Icon";
import { cn } from "@/utils";

interface RootIconProps {
  handleIconClick: (e: React.MouseEvent<HTMLButtonElement>) => void;
  isFolderExpanded: boolean;
  iconPath?: string;
}

export const RootIcon = ({ handleIconClick, isFolderExpanded, iconPath }: RootIconProps) => {
  return (
    <span className="flex size-5 shrink-0 items-center justify-center">
      <button
        onClick={handleIconClick}
        className="hover:background-(--moss-list-background-hover) flex cursor-pointer items-center justify-center rounded-full"
      >
        <Icon
          icon="ChevronRight"
          className={cn("text-(--moss-list-foreground)", {
            "rotate-90": isFolderExpanded,
            "hidden group-hover/TreeRootTrigger:block": iconPath,
          })}
        />
      </button>

      {iconPath && (
        <div className="h-full w-full rounded group-hover/TreeRootTrigger:hidden">
          <img src={iconPath} className="h-full w-full" />
        </div>
      )}
    </span>
  );
};
