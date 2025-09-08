import { cn } from "@/utils/cn";

interface RootNodeOrderProps {
  order?: number;
}

export const RootNodeOrder = ({ order }: RootNodeOrderProps) => {
  return <div className={cn("text-blue-500 underline")}>{order ?? "-"}</div>;
};
