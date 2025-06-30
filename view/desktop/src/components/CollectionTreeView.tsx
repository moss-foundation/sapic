import "@repo/moss-tabs/assets/styles.css";

import { useEffect, useRef, useState } from "react";

import { CollectionTree, InputPlain } from "@/components";
import { useCollectionsTrees } from "@/hooks/collection/useCollectionsTrees";
import { Icon, Scrollbar } from "@/lib/ui";
import { useCollectionsStore } from "@/store/collections";
import { useRequestModeStore } from "@/store/requestMode";
import { cn } from "@/utils";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { useHandleCollectionsDragAndDrop } from "./CollectionTree/hooks/useHandleCollectionsDragAndDrop";
import { TreeCollectionNode } from "./CollectionTree/types";
import { getActualDropSourceTarget } from "./CollectionTree/utils";

export const CollectionTreeView = () => {
  const dropTargetToggleRef = useRef<HTMLDivElement>(null);

  const [showCollectionCreationZone, setShowCollectionCreationZone] = useState<boolean>(false);
  const { displayMode } = useRequestModeStore();
  const { updateCollectionTree, areCollectionsStreaming, areCollectionEntriesStreaming } = useCollectionsStore();

  useHandleCollectionsDragAndDrop();

  // useEffect(() => {
  //   const handleCreateNewCollectionFromTreeNode = (event: CustomEvent<CreateNewCollectionFromTreeNodeEvent>) => {
  //     const { source } = event.detail;
  //     const newTreeId = `collectionId${collections.length + 1}`;

  //     setCollections([
  //       ...collections,
  //       {
  //         id: newTreeId,
  //         type: "collection",
  //         order: collections.length + 1,
  //         name: "New Collection",
  //         tree: {
  //           "id": "New Collection",
  //           "order": collections.length + 1,
  //           "type": "folder",
  //           "isFolder": true,
  //           "isExpanded": true,
  //           "childNodes": [source.node],
  //         },
  //       },
  //     ]);
  //     setTimeout(() => {
  //       window.dispatchEvent(
  //         new CustomEvent("newCollectionWasCreated", {
  //           detail: {
  //             treeId: newTreeId,
  //           },
  //         })
  //       );
  //     }, 50);
  //   };

  //   window.addEventListener("createNewCollectionFromTreeNode", handleCreateNewCollectionFromTreeNode as EventListener);

  //   return () => {
  //     window.removeEventListener(
  //       "createNewCollectionFromTreeNode",
  //       handleCreateNewCollectionFromTreeNode as EventListener
  //     );
  //   };
  // }, [collections, setCollections]);

  const shouldShowCollectionTree = !areCollectionsStreaming && !areCollectionEntriesStreaming;
  const { collectionsTrees, isLoading } = useCollectionsTrees();

  // useEffect(() => {
  //   console.log("collectionsTrees", collectionsTrees, isLoading);
  // }, [collectionsTrees, isLoading]);

  const handleNodeUpdate = (node: TreeCollectionNode) => {
    console.log("node", node);
  };

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
                <CollectionTree
                  key={collection.id}
                  tree={collection}
                  // onTreeUpdate={updateCollectionTree}
                  // onNodeUpdate={handleNodeUpdate}
                  displayMode={displayMode}
                />
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
  const [canDrop, setCanDrop] = useState<boolean | null>(null);

  const ref = useRef<HTMLDivElement>(null);

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
        return source.data.type === "TreeNode";
      },
      onDragEnter() {
        setCanDrop(true);
      },
      onDragLeave() {
        setCanDrop(null);
      },
      onDrop({ source }) {
        const sourceTarget = getActualDropSourceTarget(source);

        window.dispatchEvent(
          new CustomEvent("createNewCollectionFromTreeNode", {
            detail: {
              source: sourceTarget,
            },
          })
        );

        setCanDrop(null);
      },
    });
  }, []);

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
