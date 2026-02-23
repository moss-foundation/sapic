import { environmentSummariesCollection } from "@/db/environmentsSummaries/environmentSummaries";
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

interface IEnvironmentService {
  activateEnvironment: (input: ActivateEnvironmentInput) => Promise<ActivateEnvironmentOutput>;

  createEnvironment: (input: CreateEnvironmentInput) => Promise<CreateEnvironmentOutput>;

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

      order: undefined,
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
    const output = await environmentIpc.updateEnvironment({
      id: input.id,
      varsToAdd: input.varsToAdd,
      varsToUpdate: input.varsToUpdate,
      varsToDelete: input.varsToDelete,
    });

    environmentSummariesCollection.update(output.id, (draft) => {
      draft.totalVariables = draft.totalVariables + (input.varsToAdd.length ?? 0) - (input.varsToDelete.length ?? 0);
      if (input.name) draft.name = input.name;
      if (input.color && typeof input.color === "object" && "UPDATE" in input.color) draft.color = input.color.UPDATE;
      if (input.color && input.color === "REMOVE") draft.color = null;
    });

    return output;
  },
  batchUpdateEnvironment: async (input) => {
    const output = await environmentIpc.batchUpdateEnvironment(input);

    output.ids.forEach((id) => {
      environmentSummariesCollection.update(id, (draft) => {
        const item = input.items.find((item) => item.id === id);

        if (item?.varsToAdd.length && item.varsToAdd.length > 0) draft.totalVariables += item.varsToAdd.length;
        if (item?.varsToDelete.length && item.varsToDelete.length > 0) draft.totalVariables -= item.varsToDelete.length;
        if (item?.name) draft.name = item.name;
        if (item?.color && typeof item.color === "object" && "UPDATE" in item.color) draft.color = item.color.UPDATE;
        if (item?.color && item.color === "REMOVE") draft.color = null;
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
