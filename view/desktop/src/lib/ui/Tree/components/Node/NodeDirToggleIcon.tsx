import { HTMLAttributes } from "react";

import { Icon } from "@/lib/ui";
import { cn } from "@/utils";

interface NodeDirToggleIconProps extends HTMLAttributes<HTMLDivElement> {
  handleClickOnDir: (e: React.MouseEvent<HTMLButtonElement>) => void;
  isDir: boolean;
  shouldRenderChildNodes: boolean;
}

export const NodeDirToggleIcon = ({
  handleClickOnDir,
  isDir,
  shouldRenderChildNodes,
  className,
  ...props
}: NodeDirToggleIconProps) => {
  return (
    <div className={cn("flex size-5 shrink-0 items-center justify-center", className)} {...props}>
      <button
        onClick={handleClickOnDir}
        className={cn(
          "hover:background-(--moss-list-toolbarItem-background-hover) flex cursor-pointer items-center justify-center rounded-full",
          {
            "opacity-0": !isDir,
          }
        )}
      >
        <Icon
          icon="ChevronRight"
          className={cn("", {
            "rotate-90": shouldRenderChildNodes,
            "opacity-0": !isDir,
          })}
        />
      </button>
    </div>
  );
};
