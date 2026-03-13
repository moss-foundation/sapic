import { cn } from "@/utils/cn";

interface RootOrderProps {
  order?: number;
}

export const RootOrder = ({ order }: RootOrderProps) => {
  return <div className={cn("text-blue-700 underline")}>{order ?? "-"}</div>;
};
