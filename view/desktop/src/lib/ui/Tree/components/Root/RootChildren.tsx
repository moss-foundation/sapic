import { cn } from "@/utils";

import { DirDepthIndicator } from "../DirDepthIndicator";

interface RootChildrenProps {
  children?: React.ReactNode;
  className?: string;
  hideDirDepthIndicator?: boolean;
  offset?: number;
  depth?: number;
  treeOffset?: number;
}

export const RootChildren = ({
  children,
  className,
  hideDirDepthIndicator,
  offset,
  depth,
  treeOffset,
  ...props
}: RootChildrenProps) => {
  const dirDepthIndicatorOffset = depth && offset && treeOffset ? treeOffset + depth * offset : 0;
  return (
    <ul className={cn("relative w-full", className)} {...props}>
      {!hideDirDepthIndicator && <DirDepthIndicator offset={dirDepthIndicatorOffset} />}

      {children}
    </ul>
  );
};
