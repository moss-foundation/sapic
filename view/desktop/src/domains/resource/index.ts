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
import { Channel } from "@tauri-apps/api/core";

export interface IResourceIpc {
  create: (projectId: string, input: CreateResourceInput) => Promise<CreateResourceOutput>;

  delete: (projectId: string, input: DeleteResourceInput) => Promise<DeleteResourceOutput>;

  describe: (id: string, projectId: string) => Promise<DescribeResourceOutput>;

  stream: (projectId: string, channel: Channel<StreamResourcesEvent>, path?: string) => Promise<void>;

  update: (projectId: string, input: UpdateResourceInput) => Promise<UpdateResourceOutput>;

  batchCreate: (projectId: string, input: BatchCreateResourceInput) => Promise<BatchCreateResourceOutput>;

  batchUpdate: (
    projectId: string,
    input: BatchUpdateResourceInput,
    channel: Channel<BatchUpdateResourceEvent>
  ) => Promise<BatchUpdateResourceOutput>;
}
