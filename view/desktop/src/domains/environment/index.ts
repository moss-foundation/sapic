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

export interface IEnvironmentIpc {
  activateEnvironment: (input: ActivateEnvironmentInput) => Promise<ActivateEnvironmentOutput>;

  listWorkspaceEnvironments: () => Promise<ListWorkspaceEnvironmentsOutput>;
  listProjectEnvironments: (input: ListProjectEnvironmentsInput) => Promise<ListProjectEnvironmentsOutput>;

  updateEnvironment: (input: UpdateEnvironmentInput) => Promise<UpdateEnvironmentOutput>;
  batchUpdateEnvironment: (input: BatchUpdateEnvironmentInput) => Promise<BatchUpdateEnvironmentOutput>;

  createEnvironment: (input: CreateEnvironmentInput) => Promise<CreateEnvironmentOutput>;
  deleteEnvironment: (input: DeleteEnvironmentInput) => Promise<DeleteEnvironmentOutput>;
}
