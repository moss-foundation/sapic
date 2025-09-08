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

const actionsHoverStyles = cva(["transition-[display,opacity] transition-discrete duration-100"], {
  variants: {
    invisible: {
      false: ["sr-only group-hover/TreeNodeControls:contents group-hover/TreeRootNodeControls:contents"],
      true: ["opacity-0 group-hover/TreeNodeControls:opacity-100 group-hover/TreeRootNodeControls:opacity-100"],
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
