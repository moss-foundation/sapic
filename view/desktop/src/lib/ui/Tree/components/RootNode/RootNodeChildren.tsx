import { cn } from "@/utils";

import { DirDepthIndicator } from "../DirDepthIndicator";

interface RootNodeChildrenProps {
  children?: React.ReactNode;
  className?: string;
  hideDirDepthIndicator?: boolean;
}

export const RootNodeChildren = ({ children, className, hideDirDepthIndicator, ...props }: RootNodeChildrenProps) => {
  return (
    <ul className={cn("relative w-full", className)} {...props}>
      {!hideDirDepthIndicator && <DirDepthIndicator depth={0} />}

      {children}
    </ul>
  );
};
