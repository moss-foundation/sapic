import { cn } from "@/utils/cn";

import { useTreeContext } from "../TreeContext";

interface RootNodeOrderProps {
  order?: number;
}

export const RootNodeOrder = ({ order }: RootNodeOrderProps) => {
  const { showNodeOrders } = useTreeContext();

  if (!showNodeOrders) return null;

  return <div className={cn("underline")}>{order ?? "-"}</div>;
};
