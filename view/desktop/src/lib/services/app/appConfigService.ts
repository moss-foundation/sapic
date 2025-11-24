import { invokeTauriServiceIpc } from "@/infra/ipc/tauri";
import { DescribeAppOutput, UpdateConfigurationInput } from "@repo/window";

export const appConfigService = {
  describeApp: async () => {
    return await invokeTauriServiceIpc<void, DescribeAppOutput>({ cmd: "describe_app" });
  },
  updateConfiguration: async (configuration: UpdateConfigurationInput) => {
    return await invokeTauriServiceIpc<UpdateConfigurationInput, void>({
      cmd: "update_configuration",
      args: {
        input: {
          ...configuration,
        },
      },
    });
  },
};
