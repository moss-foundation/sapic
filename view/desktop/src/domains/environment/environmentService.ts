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
    return await environmentIpc.activateEnvironment(input);
  },
  batchUpdateEnvironment: async (input) => {
    return await environmentIpc.batchUpdateEnvironment(input);
  },
  batchUpdateEnvironmentGroup: async (input) => {
    return await environmentIpc.batchUpdateEnvironmentGroup(input);
  },
  createEnvironment: async (input) => {
    return await environmentIpc.createEnvironment(input);
  },
  deleteEnvironment: async (input) => {
    return await environmentIpc.deleteEnvironment(input);
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
    return await environmentIpc.updateEnvironment(input);
  },
  updateEnvironmentGroup: async (input) => {
    return await environmentIpc.updateEnvironmentGroup(input);
  },
};
