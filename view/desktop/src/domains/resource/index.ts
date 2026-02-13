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

export interface IResourceIpc {
  create: (projectId: string, input: CreateResourceInput) => Promise<CreateResourceOutput>;

  delete: (projectId: string, input: DeleteResourceInput) => Promise<DeleteResourceOutput>;

  describe: (id: string, projectId: string) => Promise<DescribeResourceOutput>;

  list: (input: ListProjectResourcesInput) => Promise<ListProjectResourcesOutput>;

  update: (projectId: string, input: UpdateResourceInput) => Promise<UpdateResourceOutput>;

  batchCreate: (projectId: string, input: BatchCreateResourceInput) => Promise<BatchCreateResourceOutput>;

  batchUpdate: (
    projectId: string,
    input: BatchUpdateResourceInput,
    channel: Channel<BatchUpdateResourceEvent>
  ) => Promise<BatchUpdateResourceOutput>;
}
