import "@repo/moss-tabs/assets/styles.css";

import { useEffect, useRef, useState } from "react";

import { CollectionTree, InputPlain } from "@/components";
import { useStreamedCollections } from "@/hooks";
import { useCollectionsTrees } from "@/hooks/collection/derivedHooks/useCollectionsTrees";
import { useCreateCollection } from "@/hooks/collection/useCreateCollection";
import { useCreateCollectionEntry } from "@/hooks/collection/useCreateCollectionEntry";
import { useDeleteCollectionEntry } from "@/hooks/collection/useDeleteCollectionEntry";
import { Icon, Scrollbar } from "@/lib/ui";
import { useRequestModeStore } from "@/store/requestMode";
import { cn } from "@/utils";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { join } from "@tauri-apps/api/path";

import { useCollectionsDragAndDropHandler } from "./CollectionTree/hooks/useHandleCollectionsDragAndDrop";
import { useMonitorForNodeDragAndDrop } from "./CollectionTree/hooks/useMonitorForNodeDragAndDrop";
import {
  convertEntryInfoToCreateInput,
  getAllNestedEntries,
  getSourceTreeNodeData,
  isSourceTreeNode,
} from "./CollectionTree/utils2";

export const CollectionTreeView = () => {
  const dropTargetToggleRef = useRef<HTMLDivElement>(null);

  const { displayMode } = useRequestModeStore();

  useCollectionsDragAndDropHandler();

  const [showCollectionCreationZone, setShowCollectionCreationZone] = useState<boolean>(false);

  useEffect(() => {
    if (!dropTargetToggleRef.current) return;
    const element = dropTargetToggleRef.current;

    return dropTargetForElements({
      element,
      getData: () => ({
        type: "CollectionCreationZone",
      }),
      canDrop({ source }) {
        return source.data.type === "TreeNode";
      },
      onDrop() {
        setShowCollectionCreationZone(false);
      },
      onDragLeave() {
        setShowCollectionCreationZone(false);
      },
      onDragStart() {
        setShowCollectionCreationZone(true);
      },
      onDragEnter() {
        setShowCollectionCreationZone(true);
      },
    });
  }, []);

  const { collectionsTrees, isLoading } = useCollectionsTrees();

  return (
    <div ref={dropTargetToggleRef} className="relative h-[calc(100%-36px)] select-none">
      <Scrollbar className="h-full">
        <div className="flex h-full flex-col">
          <div className="flex shrink items-center gap-[7px] py-1 pr-2.5 pl-2">
            <InputPlain placeholder="Search" size="sm" />
          </div>

          <div className="flex grow flex-col">
            {!isLoading &&
              collectionsTrees
                .sort((a, b) => a.order! - b.order!)
                .map((collection) => (
                  <CollectionTree key={collection.id} tree={collection} displayMode={displayMode} />
                ))}
          </div>

          {showCollectionCreationZone && (
            <div className="flex justify-end p-2">
              <CollectionCreationZone />
            </div>
          )}
        </div>
      </Scrollbar>
    </div>
  );
};

const CollectionCreationZone = () => {
  const ref = useRef<HTMLDivElement>(null);

  const [canDrop, setCanDrop] = useState<boolean | null>(null);

  const { mutateAsync: createCollection } = useCreateCollection();
  const { mutateAsync: createCollectionEntry } = useCreateCollectionEntry();
  const { mutateAsync: deleteCollectionEntry } = useDeleteCollectionEntry();
  const { data: collections } = useStreamedCollections();

  useMonitorForNodeDragAndDrop();

  useEffect(() => {
    const element = ref.current;
    if (!element) return;

    return dropTargetForElements({
      element,
      getData: () => ({
        type: "CollectionCreationZone",
        data: {},
      }),
      canDrop({ source }) {
        return isSourceTreeNode(source);
      },
      onDragEnter() {
        setCanDrop(true);
      },
      onDragLeave() {
        setCanDrop(null);
      },
      onDrop: async ({ source }) => {
        setCanDrop(null);

        const sourceTarget = getSourceTreeNodeData(source);

        if (!sourceTarget) return;

        const entries = getAllNestedEntries(sourceTarget.node);

        if (entries.length === 0) return;

        const rootEntry = entries[0];
        const nestedEntries = entries.slice(1);

        const newCollection = await createCollection({
          name: rootEntry.name,
          order: (collections?.length ?? 0) + 1,
          repository: sourceTarget.repository ?? undefined,
        });

        try {
          for (const entry of entries) {
            await deleteCollectionEntry({
              collectionId: sourceTarget.collectionId,
              input: { id: entry.id },
            });
          }
        } catch (error) {
          console.error("Error during collection creation:", error);
        }

        try {
          for (const [index, entry] of nestedEntries.entries()) {
            const rootEntryName = rootEntry.name;
            let adjustedSegments = entry.path.segments;

            const rootNameIndex = adjustedSegments.findIndex((segment) => segment === rootEntryName);
            if (rootNameIndex !== -1) {
              adjustedSegments = [
                ...adjustedSegments.slice(0, rootNameIndex),
                ...adjustedSegments.slice(rootNameIndex + 1),
              ];
            }

            const parentSegments = adjustedSegments.slice(0, -1);
            const parentPath = parentSegments.length > 0 ? await join(...parentSegments) : "";

            const createInput = convertEntryInfoToCreateInput(entry, parentPath);
            createInput[entry.kind === "Dir" ? "dir" : "item"].order = index + 1;

            await createCollectionEntry({
              collectionId: newCollection.id,
              input: createInput,
            });
          }
        } catch (error) {
          console.error("Error during collection creation:", error);
        }
      },
    });
  }, [collections?.length, createCollection, createCollectionEntry, deleteCollectionEntry]);

  return (
    <div
      ref={ref}
      className={cn(
        "background-(--moss-info-background) grid h-max min-h-32 w-full place-items-center rounded border-2 border-dashed border-(--moss-info-border) transition-[translate] duration-100",
        {
          "background-(--moss-info-background-hover) -translate-y-1": canDrop === true,
        }
      )}
    >
      <div className="animate-stripes flex flex-col items-center justify-center gap-3 bg-[linear-gradient(-45deg,white_5%,transparent_5%_45%,white_45%_55%,transparent_55%_95%,white_95%)] bg-size-[20px_20px] p-8 text-center">
        <Icon icon="AddCircleActive" className={cn("size-5 rounded-full text-(--moss-primary)")} />
        <span>Drag & drop selected items here to create a new collection</span>
      </div>
    </div>
  );
};

export default CollectionTreeView;
