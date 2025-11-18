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
  StreamEnvironmentsOutput,
  UpdateEnvironmentGroupInput,
  UpdateEnvironmentInput,
  UpdateEnvironmentOutput,
} from "@repo/moss-workspace";
import { Channel } from "@tauri-apps/api/core";

export interface IEnvironmentIpc {
  activateEnvironment: (input: ActivateEnvironmentInput) => Promise<ActivateEnvironmentOutput>;

  batchUpdateEnvironment: (input: BatchUpdateEnvironmentInput) => Promise<BatchUpdateEnvironmentOutput>;
  batchUpdateEnvironmentGroup: (input: BatchUpdateEnvironmentGroupInput) => Promise<BatchUpdateEnvironmentOutput>;

  createEnvironment: (input: CreateEnvironmentInput) => Promise<CreateEnvironmentOutput>;
  deleteEnvironment: (input: DeleteEnvironmentInput) => Promise<DeleteEnvironmentOutput>;
  streamEnvironments: (channelEvent: Channel<StreamEnvironmentsEvent>) => Promise<StreamEnvironmentsOutput>;

  updateEnvironment: (input: UpdateEnvironmentInput) => Promise<UpdateEnvironmentOutput>;
  updateEnvironmentGroup: (input: UpdateEnvironmentGroupInput) => Promise<void>;
}
