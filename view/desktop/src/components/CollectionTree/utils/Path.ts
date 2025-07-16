import { EntryInfo } from "@repo/moss-collection";
import { join } from "@tauri-apps/api/path";

import { TreeCollectionNode } from "../types";

export const getPathWithoutName = async (node: TreeCollectionNode | EntryInfo): Promise<EntryInfo["path"]> => {
  const newSegments = node.path.segments.filter((segment) => segment !== node.name);
  const newRaw = await join(...newSegments);

  return {
    segments: newSegments,
    raw: newRaw,
  };
};

export const getPathWithoutParentPath = async (
  path: EntryInfo["path"],
  parentPath: EntryInfo["path"]
): Promise<EntryInfo["path"]> => {
  const newSegments = path.segments.filter((segment) => !parentPath.segments.includes(segment));
  const newRaw = await join(...newSegments);

  return {
    segments: newSegments,
    raw: newRaw,
  };
};

export const removePathBeforeName = async (path: EntryInfo["path"], name: string) => {
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

export const prepareEntriesForDrop = async (entries: EntryInfo[]): Promise<EntryInfo[]> => {
  const rootEntryName = entries[0].name;

  const entriesPreparedForDrop: EntryInfo[] = [];

  for await (const entry of entries) {
    const newEntryPath = await removePathBeforeName(entry.path, rootEntryName);

    entriesPreparedForDrop.push({
      ...entry,
      path: newEntryPath,
    });
  }

  return entriesPreparedForDrop;
};
