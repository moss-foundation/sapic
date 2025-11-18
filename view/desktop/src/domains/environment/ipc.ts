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
  streamEnvironments: (channel: Channel<StreamEnvironmentsEvent>) => Promise<StreamEnvironmentsResult>;

  updateEnvironment: (input: UpdateEnvironmentInput) => Promise<UpdateEnvironmentOutput>;
  //TODO is there a need for a output here?
  updateEnvironmentGroup: (input: UpdateEnvironmentGroupInput) => Promise<void>;
}

export interface StreamEnvironmentsResult {
  environments: StreamEnvironmentsEvent[];
  groups: StreamEnvironmentsOutput["groups"];
}
