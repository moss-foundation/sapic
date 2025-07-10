import "@repo/moss-tabs/assets/styles.css";

import { useEffect, useRef, useState } from "react";

import { CollectionTree, InputPlain } from "@/components";
import { useStreamedCollections } from "@/hooks";
import { useCollectionsTrees } from "@/hooks/collection/derivedHooks/useCollectionsTrees";
import { useCreateCollection } from "@/hooks/collection/useCreateCollection";
import { Icon, Scrollbar } from "@/lib/ui";
import { useRequestModeStore } from "@/store/requestMode";
import { cn } from "@/utils";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { useHandleCollectionsDragAndDrop } from "./CollectionTree/hooks/useHandleCollectionsDragAndDrop";
import { getAllNestedEntries, getSourceTreeNodeData, isSourceTreeNode } from "./CollectionTree/utils2";

export const CollectionTreeView = () => {
  const dropTargetToggleRef = useRef<HTMLDivElement>(null);

  const { displayMode } = useRequestModeStore();

  useHandleCollectionsDragAndDrop();

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
              collectionsTrees.map((collection) => (
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
  const { data: collections } = useStreamedCollections();

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

        const newCollection = await createCollection({
          name: rootEntry.name,
          order: (collections?.length ?? 0) + 1,
          repo: sourceTarget.repository ?? undefined,
        });
      },
    });
  }, [collections?.length, createCollection]);

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
