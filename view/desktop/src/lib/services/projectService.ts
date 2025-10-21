import { DescribeResourceOutput } from "@repo/moss-project";

import { invokeTauriIpc } from "../backend/tauri";

export const ProjectService = {
  describeProjectEntry: async ({ projectId, entryId }: { projectId: string; entryId: string }) => {
    return await invokeTauriIpc<DescribeResourceOutput>("describe_project_entry", { projectId, entryId });
  },
};
