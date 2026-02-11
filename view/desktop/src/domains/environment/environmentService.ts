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

  updateEnvironment: (input: UpdateEnvironmentInput) => Promise<UpdateEnvironmentOutput>;
  batchUpdateEnvironment: (input: BatchUpdateEnvironmentInput) => Promise<BatchUpdateEnvironmentOutput>;

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

      order: input.order,

      isActive: false,
      totalVariables: 0,
    });

    return output;
  },
  listWorkspaceEnvironments: async () => {
    return await environmentIpc.listWorkspaceEnvironments();
  },
  listProjectEnvironments: async (input) => {
    return await environmentIpc.listProjectEnvironments(input);
  },
  updateEnvironment: async (input) => {
    const output = await environmentIpc.updateEnvironment(input);

    environmentSummariesCollection.update(input.id, (draft) => {
      if (input.name) draft.name = input.name;
      if (input.order) draft.order = input.order;
      if (input.expanded) draft.expanded = input.expanded;
      if (input.projectId) draft.projectId = input.projectId;

      if (input.color) {
        if (input.color === "REMOVE") draft.color = null;
        else if (typeof input.color === "object" && "UPDATE" in input.color) draft.color = input.color.UPDATE;
      }

      if (input.varsToAdd.length > 0) draft.totalVariables += input.varsToAdd.length;
      if (input.varsToDelete.length > 0) draft.totalVariables -= input.varsToDelete.length;
    });

    return output;
  },
  batchUpdateEnvironment: async (input) => {
    return await environmentIpc.batchUpdateEnvironment(input);
  },
  deleteEnvironment: async (input) => {
    const output = await environmentIpc.deleteEnvironment(input);
    environmentSummariesCollection.delete(input.id);
    return output;
  },
};
