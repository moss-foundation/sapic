import { DescribeEntryOutput } from "@repo/moss-project";

import { invokeTauriIpc } from "../backend/tauri";

export const ProjectService = {
  describeProjectEntry: async ({ projectId, entryId }: { projectId: string; entryId: string }) => {
    return await invokeTauriIpc<DescribeEntryOutput>("describe_project_entry", { projectId, entryId });
  },
};
