import { StreamEntriesEvent } from "@repo/moss-collection";
import { join } from "@tauri-apps/api/path";

import { TreeCollectionNode } from "../types";

export const getPathWithoutName = async (
  node: TreeCollectionNode | StreamEntriesEvent
): Promise<StreamEntriesEvent["path"]> => {
  const newSegments = node.path.segments.filter((segment) => segment !== node.name);
  const newRaw = newSegments.length > 0 ? await join(...newSegments) : "";

  return {
    segments: newSegments,
    raw: newRaw,
  };
};

export const getPathWithoutParentPath = async (
  path: StreamEntriesEvent["path"],
  parentPath: StreamEntriesEvent["path"]
): Promise<StreamEntriesEvent["path"]> => {
  const newSegments = path.segments.filter((segment) => !parentPath.segments.includes(segment));
  const newRaw = await join(...newSegments);

  return {
    segments: newSegments,
    raw: newRaw,
  };
};

export const removePathBeforeName = async (path: StreamEntriesEvent["path"], name: string) => {
  const nameIndex = path.segments.findIndex((segment) => segment === name);

  if (nameIndex === -1) {
    return {
      segments: path.segments,
      raw: path.raw,
    };
  }

  const newSegments = path.segments.slice(nameIndex);
  const newRaw = await join(...newSegments);

  return {
    segments: newSegments,
    raw: newRaw,
  };
};

export const prepareEntriesForDrop = async (entries: StreamEntriesEvent[]): Promise<StreamEntriesEvent[]> => {
  const rootEntryName = entries[0].name;

  const entriesPreparedForDrop: StreamEntriesEvent[] = [];

  for await (const entry of entries) {
    const newEntryPath = await removePathBeforeName(entry.path, rootEntryName);

    entriesPreparedForDrop.push({
      ...entry,
      path: newEntryPath,
    });
  }

  return entriesPreparedForDrop;
};

export const prepareEntriesForCreation = async (entries: StreamEntriesEvent[]): Promise<StreamEntriesEvent[]> => {
  const rootEntryName = entries[0].name;

  const entriesPreparedForDrop: StreamEntriesEvent[] = [];

  for await (const entry of entries) {
    const newEntryPath = await removePathBeforeName(entry.path, rootEntryName);

    entriesPreparedForDrop.push({
      ...entry,
      path: newEntryPath,
    });
  }

  const entriesWithoutName = await Promise.all(
    entriesPreparedForDrop.map(async (entry) => {
      const pathWithoutName = await getPathWithoutName(entry);

      return {
        ...entry,
        path: pathWithoutName,
      };
    })
  );

  return entriesWithoutName;
};
