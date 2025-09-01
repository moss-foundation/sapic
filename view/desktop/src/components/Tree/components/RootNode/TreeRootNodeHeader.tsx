import { cn } from "@/utils";

import { ActiveNodeIndicator } from "../ActiveNodeIndicator";
import { useTreeContext } from "../TreeContext";

interface TreeRootNodeHeaderProps {
  draggableRootRef: React.RefObject<HTMLDivElement>;
  isActive: boolean;
  children: React.ReactNode;
}

export const TreeRootNodeHeader = ({ draggableRootRef, isActive, children }: TreeRootNodeHeaderProps) => {
  const { treePaddingLeft, treePaddingRight } = useTreeContext();

  return (
    <div
      ref={draggableRootRef}
      className={cn("group/TreeRootNodeHeader relative flex w-full min-w-0 items-center justify-between py-0.75")}
      style={{
        paddingLeft: treePaddingLeft,
        paddingRight: treePaddingRight,
      }}
    >
      <ActiveNodeIndicator isActive={isActive} />
      {children}
    </div>
  );
};
