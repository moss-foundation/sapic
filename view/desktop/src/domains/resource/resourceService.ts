import { resourceDetailsCollection } from "@/db/resourceDetails/resourceDetailsCollection";
import { resourceSummariesCollection } from "@/db/resourceSummaries/resourceSummariesCollection";
import { LocalResourceSummary } from "@/db/resourceSummaries/types";
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
import { join, sep } from "@tauri-apps/api/path";

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
    const output = await resourceIpc.list(input);
    const platformSeparator = sep();

    output.items.forEach((resource) => {
      if (resourceSummariesCollection.has(resource.id)) {
        resourceSummariesCollection.update(resource.id, (draft) => {
          Object.assign(draft, {
            ...resource,
            path: { raw: resource.path.raw, segments: resource.path.raw.split(platformSeparator) },
            protocol: resource.protocol ?? undefined,
          });
        });
      } else {
        resourceSummariesCollection.insert({
          ...resource,
          projectId: input.projectId,
          path: { raw: resource.path.raw, segments: resource.path.raw.split(platformSeparator) },
          protocol: resource.protocol ?? undefined,
        });
      }
    });
    return output;
  },
  describe: async (id, projectId) => {
    const output = await resourceIpc.describe(id, projectId);
    if (resourceDetailsCollection.has(id)) {
      resourceDetailsCollection.update(id, (draft) => {
        Object.assign(draft, output);
      });
    } else {
      resourceDetailsCollection.insert({
        ...output,
        id,
        metadata: {
          isDirty: false,
        },
      });
    }
    return output;
  },

  create: async (projectId, input) => {
    const output = await resourceIpc.create(projectId, input);

    const newResourceSummary = await createInputToResourceSummary(projectId, input, output);
    resourceSummariesCollection.insert(newResourceSummary);

    return output;
  },
  batchCreate: async (projectId, input) => {
    const output = await resourceIpc.batchCreate(projectId, input);
    const resourceSummaries = batchCreateInputToResourceSummary(projectId, input, output);

    resourceSummaries.forEach((summary) => {
      resourceSummariesCollection.insert(summary);
    });

    return output;
  },

  update: async (projectId, input) => {
    const output = await resourceIpc.update(projectId, input);

    const id = "ITEM" in input ? input.ITEM.id : input.DIR.id;

    resourceSummariesCollection.update(id, (draft) => {
      if ("ITEM" in input) {
        if (input.ITEM.name) draft.name = input.ITEM.name;
        if (input.ITEM.path) draft.path = { raw: input.ITEM.path, segments: input.ITEM.path.split("/") };
        if (input.ITEM.protocol) draft.protocol = input.ITEM.protocol;
      }
      if ("DIR" in input) {
        if (input.DIR.name) {
          draft.name = input.DIR.name;
          if (!input.DIR.path) {
            const platformSeparator = sep();
            const segments = [...draft.path.segments];
            segments[segments.length - 1] = input.DIR.name;
            draft.path = { raw: segments.join(platformSeparator), segments };
          }
        }
        if (input.DIR.path) draft.path = { raw: input.DIR.path, segments: input.DIR.path.split("/") };
        if (input.DIR.expanded) draft.expanded = input.DIR.expanded;
      }
    });

    return output;
  },
  batchUpdate: async (projectId, input, channelEvent) => {
    const output = await resourceIpc.batchUpdate(projectId, input, channelEvent);
    const resourceSummaries = batchUpdateInputToResourceSummary(projectId, input);
    resourceSummaries.forEach((summary) => {
      resourceSummariesCollection.update(summary.id, (draft) => {
        Object.assign(draft, summary);
      });
    });
    return output;
  },

  delete: async (projectId, input) => {
    const output = await resourceIpc.delete(projectId, input);

    const deletedSummary = resourceSummariesCollection.has(input.id) ? resourceSummariesCollection.get(input.id) : null;
    if (deletedSummary) {
      resourceSummariesCollection.delete(input.id);
      resourceSummariesCollection.forEach((resource) => {
        if (resource.path.segments.length > deletedSummary.path.segments.length) {
          const isNested = deletedSummary.path.segments.every(
            (segment, index) => resource.path.segments[index] === segment
          );
          if (isNested) {
            resourceSummariesCollection.delete(resource.id);
          }
        }
      });
    }

    return output;
  },
};

const createInputToResourceSummary = async (
  projectId: string,
  input: CreateResourceInput,
  output: CreateResourceOutput
): Promise<LocalResourceSummary> => {
  const platformSeparator = sep();

  if ("ITEM" in input) {
    const rawPath = await join(input.ITEM.path, input.ITEM.name);

    return {
      projectId,
      id: output.id,
      name: input.ITEM.name,
      path: { raw: rawPath, segments: rawPath.split(platformSeparator) },
      class: input.ITEM.class,
      protocol: input.ITEM.protocol,
      kind: "Item",
    };
  }

  if ("DIR" in input) {
    const rawPath = await join(input.DIR.path, input.DIR.name);
    return {
      projectId,
      id: output.id,
      name: input.DIR.name,
      path: { raw: rawPath, segments: rawPath.split(platformSeparator) },
      class: input.DIR.class,
      kind: "Dir",
    };
  }

  throw new Error("Invalid input");
};

const batchCreateInputToResourceSummary = (
  projectId: string,
  input: BatchCreateResourceInput,
  output: BatchCreateResourceOutput
): LocalResourceSummary[] => {
  const resourceSummaries: LocalResourceSummary[] = [];
  output.resources.forEach((resource, index) => {
    const summary: LocalResourceSummary = {
      projectId,
      id: resource.id,
      name: resource.name,
      path: resource.path,
      class: input[index].ITEM?.class ?? input[index].DIR?.class,
      kind: input[index].ITEM?.kind ?? input[index].DIR?.kind,
      protocol: input[index].ITEM?.protocol ?? input[index].DIR?.protocol,
      order: undefined,
      expanded: undefined,
    };

    resourceSummaries.push(summary);
  });

  return resourceSummaries;
};

const batchUpdateInputToResourceSummary = (
  projectId: string,
  input: BatchUpdateResourceInput
): LocalResourceSummary[] => {
  const resourceSummaries: LocalResourceSummary[] = [];
  input.resources.forEach((resource, index) => {
    const id = "ITEM" in input[index] ? input[index].ITEM.id : input[index].DIR.id;
    const summary: LocalResourceSummary = {
      projectId,
      id,
      name: input[index].ITEM?.name ?? input[index].DIR?.name,
      path: {
        raw: input[index].ITEM?.path ?? input[index].DIR?.path,
        segments: input[index].ITEM?.path?.split("/") ?? input[index].DIR?.path.split("/"),
      },
      class: input[index].ITEM?.class ?? input[index].DIR?.class,
      kind: input[index].ITEM?.kind ?? input[index].DIR?.kind,
      protocol: input[index].ITEM?.protocol ?? input[index].DIR?.protocol,
    } satisfies LocalResourceSummary;
    resourceSummaries.push(summary);
  });
  return resourceSummaries;
};
