import { resourceSummariesCollection } from "@/db/resourceSummaries/resourceSummariesCollection";
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
  list: (input: ListProjectResourcesInput) => Promise<ListProjectResourcesOutput>;
  describe: (projectId: string, resourceId: string) => Promise<DescribeResourceOutput>;

  create: (projectId: string, input: CreateResourceInput) => Promise<CreateResourceOutput>;
  batchCreate: (projectId: string, input: BatchCreateResourceInput) => Promise<BatchCreateResourceOutput>;

  update: (projectId: string, input: UpdateResourceInput) => Promise<UpdateResourceOutput >;
  batchUpdate: (projectId: string, input: BatchUpdateResourceInput, channelEvent: Channel<BatchUpdateResourceEvent>) => Promise<BatchUpdateResourceOutput>;

  delete: (projectId: string, input: DeleteResourceInput) => Promise<DeleteResourceOutput>;
}

export const resourceService: IResourceService = {
  list: async (input) => {
    return await resourceIpc.list(input);
  },
  describe: async (id, projectId) => {
    return await resourceIpc.describe(id, projectId);
  },

  create: async (projectId, input) => {
    return await resourceIpc.create(projectId, input);
  },
  batchCreate: async (projectId, input) => {
    return await resourceIpc.batchCreate(projectId, input);
  },

  update: async (projectId, input) => {
    return await resourceIpc.update(projectId, input);
  },
  batchUpdate: async (projectId, input, channelEvent) => {
    return await resourceIpc.batchUpdate(projectId, input, channelEvent);
  },

  delete: async (projectId, input) => {
    const output = await resourceIpc.delete(projectId, input);

    //TODO should delete all nested resources summaries too
    resourceSummariesCollection.delete(input.id);

    return output;
  },
};
