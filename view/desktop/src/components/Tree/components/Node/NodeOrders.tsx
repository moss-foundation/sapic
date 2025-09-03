import { cn } from "@/utils/cn";

interface NodeOrderProps {
  order?: number;
}

export const NodeOrder = ({ order }: NodeOrderProps) => {
  return <div className={cn("underline")}>{order ?? "-"}</div>;
};
