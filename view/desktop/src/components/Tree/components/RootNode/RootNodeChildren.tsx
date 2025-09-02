import { cn } from "@/utils";

import { DirDepthIndicator } from "../DirDepthIndicator";

interface RootNodeChildrenProps {
  children?: React.ReactNode;
  className?: string;
}

export const RootNodeChildren = ({ children, className, ...props }: RootNodeChildrenProps) => {
  return (
    <ul className={cn("relative w-full", className)} {...props}>
      <DirDepthIndicator depth={0} />

      {children}
    </ul>
  );
};
