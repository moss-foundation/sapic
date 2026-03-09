import {
  BatchUpdateProjectInput,
  BatchUpdateProjectOutput,
  CreateProjectInput,
  CreateProjectOutput,
  DeleteProjectInput,
  DeleteProjectOutput,
  ImportProjectInput,
  ImportProjectOutput,
  ListProjectsOutput,
  UpdateProjectInput,
  UpdateProjectOutput,
} from "@repo/ipc";

export interface IProjectIpc {
  batchUpdate: (input: BatchUpdateProjectInput) => Promise<BatchUpdateProjectOutput>;

  create: (input: CreateProjectInput) => Promise<CreateProjectOutput>;
  delete: (input: DeleteProjectInput) => Promise<DeleteProjectOutput>;

  import: (input: ImportProjectInput) => Promise<ImportProjectOutput>;

  list: () => Promise<ListProjectsOutput>;

  update: (input: UpdateProjectInput) => Promise<UpdateProjectOutput>;
}
