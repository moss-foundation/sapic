import { cn } from "@/utils/cn";

interface RootNodeOrderProps {
  order?: number;
}

export const RootNodeOrder = ({ order }: RootNodeOrderProps) => {
  return <div className={cn("underline")}>{order ?? "-"}</div>;
};
