import { projectIpc } from "@/infra/ipc/project";
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
} from "@repo/moss-workspace";
import { Channel } from "@tauri-apps/api/core";

// prettier-ignore
interface IProjectService {
  batchUpdateProject: (input: BatchUpdateProjectInput) => Promise<BatchUpdateProjectOutput>;

  createProject: (input: CreateProjectInput) => Promise<CreateProjectOutput>;

  deleteProject: (input: DeleteProjectInput) => Promise<DeleteProjectOutput>;

  importProject: (input: ImportProjectInput) => Promise<ImportProjectOutput>;

  streamProjects: (channelEvent: Channel<StreamProjectsEvent>) => Promise<StreamProjectsEvent[]>;

  updateProject: (input: UpdateProjectInput) => Promise<UpdateProjectOutput>;
}

export const projectService: IProjectService = {
  batchUpdateProject: async (input) => {
    return await projectIpc.batchUpdateProject(input);
  },
  createProject: async (input) => {
    return await projectIpc.createProject(input);
  },
  deleteProject: async (input) => {
    return await projectIpc.deleteProject(input);
  },
  importProject: async (input) => {
    return await projectIpc.importProject(input);
  },
  streamProjects: async (channelEvent) => {
    return await projectIpc.streamProjects(channelEvent);
  },
  updateProject: async (input) => {
    return await projectIpc.updateProject(input);
  },
};
