import { cn } from "@/utils/cn";

interface RootNodeOrderProps {
  order?: number;
}

export const RootNodeOrder = ({ order }: RootNodeOrderProps) => {
  return <div className={cn("text-blue-700 underline")}>{order ?? "-"}</div>;
};
