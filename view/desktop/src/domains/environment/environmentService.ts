import { environmentSummariesCollection } from "@/db/environmentsSummaries/environmentSummaries";
import { EnvironmentSummary } from "@/db/environmentsSummaries/types";
import { environmentIpc } from "@/infra/ipc/environmentIpc";
import {
  ActivateEnvironmentInput,
  ActivateEnvironmentOutput,
  BatchUpdateEnvironmentInput,
  BatchUpdateEnvironmentOutput,
  CreateEnvironmentInput,
  CreateEnvironmentOutput,
  DeleteEnvironmentInput,
  DeleteEnvironmentOutput,
  ListProjectEnvironmentsInput,
  ListProjectEnvironmentsOutput,
  ListWorkspaceEnvironmentsOutput,
  UpdateEnvironmentInput,
  UpdateEnvironmentOutput,
} from "@repo/ipc";

export interface CreateEnvironmentParams extends CreateEnvironmentInput {
  order: number;
  expanded: boolean;
}

interface IEnvironmentService {
  activateEnvironment: (input: ActivateEnvironmentInput) => Promise<ActivateEnvironmentOutput>;

  createEnvironment: (input: CreateEnvironmentParams) => Promise<CreateEnvironmentOutput>;

  listWorkspaceEnvironments: () => Promise<ListWorkspaceEnvironmentsOutput>;
  listProjectEnvironments: (input: ListProjectEnvironmentsInput) => Promise<ListProjectEnvironmentsOutput>;

  updateEnvironment(input: UpdateEnvironmentInput): Promise<UpdateEnvironmentOutput>;
  batchUpdateEnvironment(input: BatchUpdateEnvironmentInput): Promise<BatchUpdateEnvironmentOutput>;

  deleteEnvironment: (input: DeleteEnvironmentInput) => Promise<DeleteEnvironmentOutput>;
}

export const environmentService: IEnvironmentService = {
  activateEnvironment: async (input) => {
    const output = await environmentIpc.activateEnvironment(input);

    const isWorkspaceEnvironment = !input.projectId;
    const isProjectEnvironment = !!input.projectId;

    if (isWorkspaceEnvironment) {
      environmentSummariesCollection.forEach((environment) => {
        if (environment.projectId) return;

        environmentSummariesCollection.update(environment.id, (draft) => {
          draft.isActive = environment.id === input.environmentId;
        });
      });
    }

    if (isProjectEnvironment) {
      environmentSummariesCollection.forEach((environment) => {
        if (!environment.projectId || environment.projectId !== input.projectId) return;

        environmentSummariesCollection.update(environment.id, (draft) => {
          draft.isActive = environment.id === input.environmentId;
        });
      });
    }

    return output;
  },
  createEnvironment: async (input) => {
    const output = await environmentIpc.createEnvironment(input);

    environmentSummariesCollection.insert({
      id: output.id,
      projectId: output.projectId,
      name: output.name,
      color: output.color,

      isActive: false,
      totalVariables: 0,

      order: input.order,
      expanded: input.expanded,
    });

    return output;
  },
  listWorkspaceEnvironments: async () => {
    return await environmentIpc.listWorkspaceEnvironments();
  },
  listProjectEnvironments: async (input) => {
    return await environmentIpc.listProjectEnvironments(input);
  },

  updateEnvironment: async (input: UpdateEnvironmentInput) => {
    const output = await environmentIpc.updateEnvironment(input);

    environmentSummariesCollection.update(output.id, (draft) => {
      applyEnvironmentUpdate(draft, input);
    });

    return output;
  },
  batchUpdateEnvironment: async (input) => {
    const output = await environmentIpc.batchUpdateEnvironment(input);

    output.ids.forEach((id) => {
      environmentSummariesCollection.update(id, (draft) => {
        const item = input.items.find((item) => item.id === id);
        if (!item) return;

        applyEnvironmentUpdate(draft, item);
      });
    });

    return output;
  },

  deleteEnvironment: async (input) => {
    const output = await environmentIpc.deleteEnvironment(input);
    environmentSummariesCollection.delete(input.id);
    return output;
  },
};

const applyEnvironmentUpdate = (draft: EnvironmentSummary, data: Partial<UpdateEnvironmentInput>) => {
  if (data.name) draft.name = data.name;

  if (data.color === "REMOVE") {
    draft.color = null;
  } else if (data.color && typeof data.color === "object" && "UPDATE" in data.color) {
    draft.color = data.color.UPDATE;
  }

  if (data.varsToAdd?.length) draft.totalVariables += data.varsToAdd.length;
  if (data.varsToDelete?.length) draft.totalVariables -= data.varsToDelete.length;
};
