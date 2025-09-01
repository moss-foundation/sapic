import { cn } from "@/utils/cn";

interface TreeRootNodeOrderProps {
  order?: number;
}

export const TreeRootNodeOrder = ({ order }: TreeRootNodeOrderProps) => {
  return <div className={cn("underline")}>{order ?? "-"}</div>;
};
