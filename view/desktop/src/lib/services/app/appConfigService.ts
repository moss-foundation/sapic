import { invokeTauriServiceIpc } from "@/infra/ipc/tauri";
import { DescribeAppOutput } from "@repo/window";

export const appConfigService = {
  describeApp: async () => {
    return await invokeTauriServiceIpc<void, DescribeAppOutput>({ cmd: "describe_app" });
  },
};
