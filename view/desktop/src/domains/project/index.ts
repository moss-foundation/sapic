import {
  BatchUpdateProjectInput,
  BatchUpdateProjectOutput,
  CreateProjectInput,
  CreateProjectOutput,
  DeleteProjectInput,
  DeleteProjectOutput,
  ImportProjectInput,
  ImportProjectOutput,
  StreamProjectsEvent,
  UpdateProjectInput,
  UpdateProjectOutput,
} from "@repo/ipc";
import { Channel } from "@tauri-apps/api/core";

export interface IProjectIpc {
  batchUpdateProject: (input: BatchUpdateProjectInput) => Promise<BatchUpdateProjectOutput>;

  createProject: (input: CreateProjectInput) => Promise<CreateProjectOutput>;
  deleteProject: (input: DeleteProjectInput) => Promise<DeleteProjectOutput>;

  importProject: (input: ImportProjectInput) => Promise<ImportProjectOutput>;

  streamProjects: (channel: Channel<StreamProjectsEvent>) => Promise<StreamProjectsEvent[]>;

  updateProject: (input: UpdateProjectInput) => Promise<UpdateProjectOutput>;
}
