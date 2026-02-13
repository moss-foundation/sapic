import { resourceIpc } from "@/infra/ipc/resourceIpc";
import { ListProjectResourcesInput, ListProjectResourcesOutput } from "@repo/ipc";
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
  UpdateResourceInput,
  UpdateResourceOutput,
} from "@repo/moss-project";
import { Channel } from "@tauri-apps/api/core";

// prettier-ignore
interface IResourceService {
  batchCreate: (projectId: string, input: BatchCreateResourceInput) => Promise<BatchCreateResourceOutput>;
  batchUpdate: (projectId: string, input: BatchUpdateResourceInput, channelEvent: Channel<BatchUpdateResourceEvent>) => Promise<BatchUpdateResourceOutput>;

  create: (projectId: string, input: CreateResourceInput) => Promise<CreateResourceOutput>;

  delete: (projectId: string, input: DeleteResourceInput) => Promise<DeleteResourceOutput>;

  describe: (projectId: string, resourceId: string) => Promise<DescribeResourceOutput>;

  list: (input: ListProjectResourcesInput) => Promise<ListProjectResourcesOutput>;

  update: (projectId: string, input: UpdateResourceInput) => Promise<UpdateResourceOutput >;
}

export const resourceService: IResourceService = {
  batchCreate: async (projectId, input) => {
    return await resourceIpc.batchCreate(projectId, input);
  },
  batchUpdate: async (projectId, input, channelEvent) => {
    return await resourceIpc.batchUpdate(projectId, input, channelEvent);
  },
  create: async (projectId, input) => {
    return await resourceIpc.create(projectId, input);
  },
  delete: async (projectId, input) => {
    return await resourceIpc.delete(projectId, input);
  },
  describe: async (id, projectId) => {
    return await resourceIpc.describe(id, projectId);
  },
  list: async (input) => {
    return await resourceIpc.list(input);
  },
  update: async (projectId, input) => {
    return await resourceIpc.update(projectId, input);
  },
};
