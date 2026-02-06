import { environmentSummariesCollection } from "@/db/environmentsSummaries/environmentSummaries";
import { environmentIpc } from "@/infra/ipc/environmentIpc";
import {
  ActivateEnvironmentInput,
  ActivateEnvironmentOutput,
  BatchUpdateEnvironmentGroupInput,
  BatchUpdateEnvironmentInput,
  BatchUpdateEnvironmentOutput,
  CreateEnvironmentInput,
  CreateEnvironmentOutput,
  DeleteEnvironmentInput,
  DeleteEnvironmentOutput,
  StreamEnvironmentsEvent,
  StreamProjectEnvironmentsInput,
  UpdateEnvironmentGroupInput,
  UpdateEnvironmentInput,
  UpdateEnvironmentOutput,
} from "@repo/ipc";
import { Channel } from "@tauri-apps/api/core";

interface IEnvironmentService {
  activateEnvironment: (input: ActivateEnvironmentInput) => Promise<ActivateEnvironmentOutput>;

  batchUpdateEnvironment: (input: BatchUpdateEnvironmentInput) => Promise<BatchUpdateEnvironmentOutput>;
  batchUpdateEnvironmentGroup: (input: BatchUpdateEnvironmentGroupInput) => Promise<BatchUpdateEnvironmentOutput>;

  createEnvironment: (input: CreateEnvironmentInput) => Promise<CreateEnvironmentOutput>;

  deleteEnvironment: (input: DeleteEnvironmentInput) => Promise<DeleteEnvironmentOutput>;

  streamEnvironments: () => Promise<StreamEnvironmentsEvent[]>;
  streamProjectEnvironments: (input: StreamProjectEnvironmentsInput) => Promise<StreamEnvironmentsEvent[]>;

  updateEnvironment: (input: UpdateEnvironmentInput) => Promise<UpdateEnvironmentOutput>;
  updateEnvironmentGroup: (input: UpdateEnvironmentGroupInput) => Promise<void>;
}

export const environmentService: IEnvironmentService = {
  activateEnvironment: async (input) => {
    const output = await environmentIpc.activateEnvironment(input);

    const isWorkspaceEnvironment = input.projectId === null || input.projectId === undefined;
    const isProjectEnvironment = input.projectId !== null && input.projectId !== undefined;

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
        if (
          environment.projectId === undefined ||
          environment.projectId === null ||
          environment.projectId !== input.projectId
        )
          return;

        environmentSummariesCollection.update(environment.id, (draft) => {
          draft.isActive = environment.id === input.environmentId;
        });
      });
    }

    return output;
  },
  batchUpdateEnvironment: async (input) => {
    return await environmentIpc.batchUpdateEnvironment(input);
  },
  batchUpdateEnvironmentGroup: async (input) => {
    return await environmentIpc.batchUpdateEnvironmentGroup(input);
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

      metadata: {
        isDirty: false,
      },
    });

    return output;
  },
  deleteEnvironment: async (input) => {
    const output = await environmentIpc.deleteEnvironment(input);
    environmentSummariesCollection.delete(output.id);
    return output;
  },
  streamEnvironments: async () => {
    const environments: StreamEnvironmentsEvent[] = [];

    const environmentsEvent = new Channel<StreamEnvironmentsEvent>();
    environmentsEvent.onmessage = (environmentGroup) => {
      environments.push(environmentGroup);
    };

    await environmentIpc.streamEnvironments(environmentsEvent);

    return environments;
  },
  streamProjectEnvironments: async (input) => {
    const projectEnvironments: StreamEnvironmentsEvent[] = [];

    const environmentsEvent = new Channel<StreamEnvironmentsEvent>();
    environmentsEvent.onmessage = (environmentGroup) => {
      projectEnvironments.push(environmentGroup);
    };

    await environmentIpc.streamProjectEnvironments(input, environmentsEvent);

    return projectEnvironments;
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
  updateEnvironmentGroup: async (input) => {
    return await environmentIpc.updateEnvironmentGroup(input);
  },
};
