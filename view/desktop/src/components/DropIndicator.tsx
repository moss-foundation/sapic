import DropIndicatorFromUI, { DropIndicatorProps } from "@/lib/ui/DropIndicator";
import { cn } from "@/utils";

export const DropIndicator = ({
  edge,
  noTerminal = false,
  gap = 0,
  strokeSize = 2,
  terminalSize = 8,
  className,
}: DropIndicatorProps) => {
  return (
    <DropIndicatorFromUI
      edge={edge}
      noTerminal={noTerminal}
      gap={gap}
      strokeSize={strokeSize}
      terminalSize={terminalSize}
      className={cn("text-(--moss-accent)", className)}
    />
  );
};

export default DropIndicator;
