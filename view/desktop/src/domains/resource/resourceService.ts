import { resourceDetailsCollection } from "@/db/resourceDetails/resourceDetailsCollection";
import { ResourceDetails } from "@/db/resourceDetails/types";
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
  describe: async (projectId, resourceId) => {
    const output = await resourceIpc.describe(projectId, resourceId);

    const sanitized = {
      ...output,
      protocol: output.protocol ?? undefined,
      url: output.url ?? undefined,
      body: output.body ?? undefined,
      queryParams: output.queryParams?.map((p) => ({
        ...p,
        description: p.description ?? undefined,
      })),
      headers: output.headers?.map((h) => ({
        ...h,
        description: h.description ?? undefined,
      })),
      pathParams: output.pathParams?.map((p) => ({
        ...p,
        description: p.description ?? undefined,
      })),
    } satisfies Omit<ResourceDetails, "id" | "metadata">;

    if (resourceDetailsCollection.has(resourceId)) {
      resourceDetailsCollection.update(resourceId, (draft) => {
        if (draft.metadata.isDirty) {
          draft.name = sanitized.name;
          draft.class = sanitized.class;
          draft.kind = sanitized.kind;
          draft.protocol = sanitized.protocol;
          draft.url = sanitized.url;
          return;
        }
        Object.assign(draft, sanitized);
      });
    } else {
      resourceDetailsCollection.insert({
        ...sanitized,
        id: resourceId,
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

    const resourceSummaries = await batchCreateInputToResourceSummary(projectId, input, output);
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
    const resourceSummaries = await batchUpdateInputToResourceSummary(projectId, input);
    resourceSummaries.forEach((summary) => {
      if (!summary.id) return;
      resourceSummariesCollection.update(summary.id, (draft) => {
        for (const [key, value] of Object.entries(summary)) {
          if (value !== undefined) {
            (draft as Record<string, unknown>)[key] = value;
          }
        }
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

const batchCreateInputToResourceSummary = async (
  projectId: string,
  input: BatchCreateResourceInput,
  output: BatchCreateResourceOutput
): Promise<LocalResourceSummary[]> => {
  const platformSeparator = sep();

  return await Promise.all(
    output.resources.map(async (resource) => {
      const inputResource = input.resources.find((r) => {
        const params = "ITEM" in r ? r.ITEM : r.DIR;
        return params.path === resource.path.raw && params.name === resource.name;
      });

      if (!inputResource) {
        throw new Error(`No matching input found for output resource: ${resource.name}`);
      }

      const isItem = "ITEM" in inputResource;
      const params = isItem ? inputResource.ITEM : inputResource.DIR;
      const rawPath = await join(params.path, params.name);

      return {
        projectId,
        id: resource.id,
        name: resource.name,
        path: { raw: rawPath, segments: rawPath.split(platformSeparator) },
        class: params.class,
        kind: isItem ? "Item" : "Dir",
        protocol: isItem ? inputResource.ITEM.protocol : undefined,
        order: undefined,
        expanded: undefined,
      } satisfies LocalResourceSummary;
    })
  );
};

const batchUpdateInputToResourceSummary = async (
  projectId: string,
  input: BatchUpdateResourceInput
): Promise<Partial<LocalResourceSummary>[]> => {
  const platformSeparator = sep();

  return await Promise.all(
    input.resources.map(async (resource) => {
      const isItem = "ITEM" in resource;
      const params = isItem ? resource.ITEM : resource.DIR;
      const rawPath = params.path && params.name ? await join(params.path, params.name) : undefined;
      return {
        projectId,
        id: params.id,
        name: params.name,
        path: rawPath ? { raw: rawPath, segments: rawPath.split(platformSeparator) } : undefined,
        kind: isItem ? "Item" : "Dir",
        protocol: isItem ? resource.ITEM.protocol : undefined,
      } satisfies Partial<LocalResourceSummary>;
    })
  );
};
