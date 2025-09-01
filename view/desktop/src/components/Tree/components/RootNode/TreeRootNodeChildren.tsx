import { cn } from "@/utils";

interface TreeRootNodeChildrenProps {
  children?: React.ReactNode;
  className?: string;
}

export const TreeRootNodeChildren = ({ children, className, ...props }: TreeRootNodeChildrenProps) => {
  return (
    <ul className={cn("relative w-full", className)} {...props}>
      {children}
    </ul>
  );
};
