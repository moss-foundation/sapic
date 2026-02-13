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
  batchUpdateProject: (input: BatchUpdateProjectInput) => Promise<BatchUpdateProjectOutput>;

  createProject: (input: CreateProjectInput) => Promise<CreateProjectOutput>;
  deleteProject: (input: DeleteProjectInput) => Promise<DeleteProjectOutput>;

  importProject: (input: ImportProjectInput) => Promise<ImportProjectOutput>;

  listProjects: () => Promise<ListProjectsOutput>;

  updateProject: (input: UpdateProjectInput) => Promise<UpdateProjectOutput>;
}
