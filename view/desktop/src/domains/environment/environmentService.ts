import { environmentIpc } from "@/infra/ipc/environment";
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
  UpdateEnvironmentGroupInput,
  UpdateEnvironmentInput,
  UpdateEnvironmentOutput,
} from "@repo/moss-workspace";
import { Channel } from "@tauri-apps/api/core";

import { StreamEnvironmentsResult } from "./types";

interface IEnvironmentService {
  activateEnvironment: (input: ActivateEnvironmentInput) => Promise<ActivateEnvironmentOutput>;

  batchUpdateEnvironment: (input: BatchUpdateEnvironmentInput) => Promise<BatchUpdateEnvironmentOutput>;
  batchUpdateEnvironmentGroup: (input: BatchUpdateEnvironmentGroupInput) => Promise<BatchUpdateEnvironmentOutput>;

  createEnvironment: (input: CreateEnvironmentInput) => Promise<CreateEnvironmentOutput>;

  deleteEnvironment: (input: DeleteEnvironmentInput) => Promise<DeleteEnvironmentOutput>;

  streamEnvironments: () => Promise<StreamEnvironmentsResult>;

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
  streamEnvironments: async (): Promise<StreamEnvironmentsResult> => {
    const environments: StreamEnvironmentsEvent[] = [];

    const environmentsEvent = new Channel<StreamEnvironmentsEvent>();
    environmentsEvent.onmessage = (environmentGroup) => {
      environments.push(environmentGroup);
    };

    const { groups } = await environmentIpc.streamEnvironments(environmentsEvent);

    return { environments, groups };
  },
  updateEnvironment: async (input) => {
    return await environmentIpc.updateEnvironment(input);
  },
  updateEnvironmentGroup: async (input) => {
    return await environmentIpc.updateEnvironmentGroup(input);
  },
};
