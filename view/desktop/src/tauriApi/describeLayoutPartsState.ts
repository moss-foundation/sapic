import { invokeTauriIpc } from "@/lib/backend/tauri";
import { DescribeLayoutPartsStateOutput } from "@repo/moss-workspace";

export const describeLayoutPartsState = async () => {
  return await invokeTauriIpc<DescribeLayoutPartsStateOutput>("describe_layout_parts_state");
};
