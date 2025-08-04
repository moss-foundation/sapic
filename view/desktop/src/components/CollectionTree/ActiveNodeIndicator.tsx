import { cn } from "@/utils";

interface ActiveNodeIndicatorProps {
  isActive: boolean;
}

export const ActiveNodeIndicator = ({ isActive }: ActiveNodeIndicatorProps) => {
  return (
    <div
      //prettier-ignore
      className={cn(`
          absolute top-0 left-0 
          h-full w-full 
          group-hover/TreeNode:background-(--moss-secondary-background-hover)
        `,
        {
          "background-(--moss-secondary-background-hover) border-l border-l-(--moss-primary)": isActive,
        }
      )}
    />
  );
};
