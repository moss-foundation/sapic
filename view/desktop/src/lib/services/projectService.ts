import { DescribeResourceOutput } from "@repo/moss-project";

import { invokeTauriIpc } from "../../infra/ipc/tauri";

export const ProjectService = {
  describeProjectResource: async ({ projectId, resourceId }: { projectId: string; resourceId: string }) => {
    return await invokeTauriIpc<DescribeResourceOutput>("describe_project_resource", { projectId, resourceId });
  },
};
