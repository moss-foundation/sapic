import { cva } from "class-variance-authority";

import { cn } from "@/utils";

interface ActionsHoverProps {
  children: React.ReactNode;
  className?: string;
  forceVisible?: boolean;
  props?: React.HTMLAttributes<HTMLDivElement>;
  invisible?: boolean;
  showOnTreeHover?: boolean;
}

const actionsHoverStyles = cva(["transition-discrete transition-[display,opacity] duration-100"], {
  variants: {
    invisible: {
      false: [
        "sr-only group-hover/TreeListActions:contents group-hover/TreeNodeDetails:contents group-hover/TreeRootNodeDetails:contents",
      ],
      true: [
        "opacity-0 group-hover/TreeListActions:opacity-100 group-hover/TreeNodeDetails:opacity-100 group-hover/TreeRootNodeDetails:opacity-100",
      ],
    },
    showOnTreeHover: {
      true: ["group-hover/TreeRootNode:contents"],
      false: [""],
    },
    forceVisible: {
      true: ["contents opacity-100"],
      false: [""],
    },
  },
});

export const ActionsHover = ({
  children,
  className,
  forceVisible,
  invisible = false,
  showOnTreeHover = false,
  ...props
}: ActionsHoverProps) => {
  return (
    <div className={cn(actionsHoverStyles({ invisible, showOnTreeHover, forceVisible }), className)} {...props}>
      {children}
    </div>
  );
};
