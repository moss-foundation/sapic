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

export interface IProjectIpc {
  batchCreateProjectResource: (
    projectId: string,
    input: BatchCreateResourceInput
  ) => Promise<BatchCreateResourceOutput>;
  batchUpdateProject: (input: BatchUpdateProjectInput) => Promise<BatchUpdateProjectOutput>;
  batchUpdateProjectResource: (
    projectId: string,
    input: BatchUpdateResourceInput,
    channel: Channel<BatchUpdateResourceEvent>
  ) => Promise<BatchUpdateResourceOutput>;

  createProject: (input: CreateProjectInput) => Promise<CreateProjectOutput>;
  createProjectResource: (projectId: string, input: CreateResourceInput) => Promise<CreateResourceOutput>;

  deleteProject: (input: DeleteProjectInput) => Promise<DeleteProjectOutput>;
  deleteProjectResource: (projectId: string, input: DeleteResourceInput) => Promise<DeleteResourceOutput>;

  describeProjectResource: (projectId: string, resourceId: string) => Promise<DescribeResourceOutput>;

  importProject: (input: ImportProjectInput) => Promise<ImportProjectOutput>;

  streamProjectResources: (projectId: string, channel: Channel<StreamResourcesEvent>, path?: string) => Promise<void>;
  streamProjects: (channel: Channel<StreamProjectsEvent>) => Promise<StreamProjectsEvent[]>;

  updateProject: (input: UpdateProjectInput) => Promise<UpdateProjectOutput>;
  updateProjectResource: (projectId: string, input: UpdateResourceInput) => Promise<UpdateResourceOutput>;
}
