import { projectIpc } from "@/infra/ipc/project";
import {
  BatchCreateResourceInput,
  BatchCreateResourceOutput,
  BatchUpdateResourceEvent,
  BatchUpdateResourceInput,
  BatchUpdateResourceOutput,
  CreateResourceInput,
  CreateResourceOutput,
  DeleteResourceInput,
  DeleteResourceOutput,
  DescribeResourceOutput,
  StreamResourcesEvent,
  UpdateResourceInput,
  UpdateResourceOutput,
} from "@repo/moss-project";
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

export * from "@/infra/ipc/project";

interface IProjectService {
  batchCreateProjectResource: (
    projectId: string,
    input: BatchCreateResourceInput
  ) => Promise<BatchCreateResourceOutput>;
  batchUpdateProject: (input: BatchUpdateProjectInput) => Promise<BatchUpdateProjectOutput>;
  batchUpdateProjectResource: (
    projectId: string,
    input: BatchUpdateResourceInput,
    channelEvent: Channel<BatchUpdateResourceEvent>
  ) => Promise<BatchUpdateResourceOutput>;

  createProject: (input: CreateProjectInput) => Promise<CreateProjectOutput>;
  createProjectResource: (projectId: string, input: CreateResourceInput) => Promise<CreateResourceOutput>;

  deleteProject: (input: DeleteProjectInput) => Promise<DeleteProjectOutput>;
  deleteProjectResource: (projectId: string, input: DeleteResourceInput) => Promise<DeleteResourceOutput>;

  describeProjectResource: (projectId: string, resourceId: string) => Promise<DescribeResourceOutput>;

  importProject: (input: ImportProjectInput) => Promise<ImportProjectOutput>;

  streamProjectResources: (
    projectId: string,
    channelEvent: Channel<StreamResourcesEvent>,
    path?: string
  ) => Promise<void>;
  streamProjects: (channelEvent: Channel<StreamProjectsEvent>) => Promise<StreamProjectsEvent[]>;

  updateProject: (input: UpdateProjectInput) => Promise<UpdateProjectOutput>;
  updateProjectResource: (projectId: string, input: UpdateResourceInput) => Promise<UpdateResourceOutput>;
}

export const projectService: IProjectService = {
  batchCreateProjectResource: async (projectId, input) => {
    return await projectIpc.batchCreateProjectResource(projectId, input);
  },
  batchUpdateProject: async (input) => {
    return await projectIpc.batchUpdateProject(input);
  },
  batchUpdateProjectResource: async (projectId, input, channelEvent) => {
    return await projectIpc.batchUpdateProjectResource(projectId, input, channelEvent);
  },
  createProject: async (input) => {
    return await projectIpc.createProject(input);
  },
  createProjectResource: async (projectId, input) => {
    return await projectIpc.createProjectResource(projectId, input);
  },
  deleteProject: async (input) => {
    return await projectIpc.deleteProject(input);
  },
  deleteProjectResource: async (projectId, input) => {
    return await projectIpc.deleteProjectResource(projectId, input);
  },
  describeProjectResource: async (projectId, resourceId) => {
    return await projectIpc.describeProjectResource(projectId, resourceId);
  },
  importProject: async (input) => {
    return await projectIpc.importProject(input);
  },
  streamProjects: async (channelEvent) => {
    return await projectIpc.streamProjects(channelEvent);
  },
  streamProjectResources: async (projectId, channelEvent, path) => {
    return await projectIpc.streamProjectResources(projectId, channelEvent, path);
  },
  updateProject: async (input) => {
    return await projectIpc.updateProject(input);
  },
  updateProjectResource: async (projectId, input) => {
    return await projectIpc.updateProjectResource(projectId, input);
  },
};
