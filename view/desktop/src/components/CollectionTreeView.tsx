import "@repo/moss-tabs/assets/styles.css";

import React, { useEffect, useRef, useState } from "react";

import { Icon, Input, Scrollbar, Tree } from "@/components";
import { useCollectionsStore } from "@/store/collections";
import { cn, swapListById } from "@/utils";
import { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/types";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { CreateNewCollectionFromTreeNodeEvent } from "./Tree/types";
import { getActualDropSourceTarget } from "./Tree/utils";

export const CollectionTreeView = () => {
  const [searchInput, setSearchInput] = useState<string>("");
  const [showCollectionCreationZone, setShowCollectionCreationZone] = useState<boolean>(false);

  const dropTargetToggleRef = useRef<HTMLDivElement>(null);

  const { collections, setCollections, updateCollection } = useCollectionsStore();

  useEffect(() => {
    const element = dropTargetToggleRef.current;
    if (!element) return;

    return dropTargetForElements({
      element,
      getData: () => ({
        type: "SidebarCollectionsList",
        data: {},
      }),
      canDrop({ source }) {
        return ["TreeNode", "TreeNodeRoot"].includes(source.data.type as string);
      },
      onDragStart({ source }) {
        if (source.data.type === "TreeNode") setShowCollectionCreationZone(true);
      },
      onDragEnter({ source }) {
        if (source.data.type === "TreeNode") setShowCollectionCreationZone(true);
      },
      onDragLeave() {
        setShowCollectionCreationZone(false);
      },
      onDrop({ location, source }) {
        setShowCollectionCreationZone(false);

        if (location.current.dropTargets.length === 0) return;

        const sourceCollectionId = getActualDropSourceTarget(source).treeId;
        const locationCollectionId = location.current.dropTargets[0].data.treeId as string;
        const closestEdge = location.current.dropTargets[0].data.closestEdge as Edge;

        if (locationCollectionId === sourceCollectionId || !sourceCollectionId || !locationCollectionId) {
          return;
        }

        setCollections(swapListById(sourceCollectionId, locationCollectionId, collections, closestEdge)!);
      },
    });
  }, [collections, setCollections]);

  useEffect(() => {
    const handleCreateNewCollectionFromTreeNode = (event: CustomEvent<CreateNewCollectionFromTreeNodeEvent>) => {
      const { source } = event.detail;
      const newTreeId = `collectionId${collections.length + 1}`;

      setCollections([
        ...collections,
        {
          id: newTreeId,
          type: "collection",
          order: collections.length + 1,
          tree: {
            "id": "New Collection",
            "order": collections.length + 1,
            "type": "folder",
            "isFolder": true,
            "isExpanded": true,
            "childNodes": [source.node],
          },
        },
      ]);
      setTimeout(() => {
        window.dispatchEvent(
          new CustomEvent("newCollectionWasCreated", {
            detail: {
              treeId: newTreeId,
            },
          })
        );
      }, 50);
    };

    window.addEventListener("createNewCollectionFromTreeNode", handleCreateNewCollectionFromTreeNode as EventListener);

    return () => {
      window.removeEventListener(
        "createNewCollectionFromTreeNode",
        handleCreateNewCollectionFromTreeNode as EventListener
      );
    };
  }, [collections, setCollections]);

  return (
    <div className="relative flex h-full flex-col pt-1 select-none" ref={dropTargetToggleRef}>
      <div className="flex items-center gap-[7px] px-2 py-1">
        <Input
          variant="plain"
          onInput={(e) => setSearchInput((e.target as HTMLInputElement).value)}
          placeholder="Search"
          size="sm"
        />
      </div>

      <Scrollbar className="h-full">
        <div className="flex h-full flex-col">
          {collections.map((collection) => (
            <React.Fragment key={collection.id}>
              <Tree
                onTreeUpdate={(tree) => updateCollection({ ...collection, tree })}
                tree={collection.tree}
                id={collection.id}
                searchInput={searchInput}
              />
              <div className="background-(--moss-border-color) h-[1px] w-full" />
            </React.Fragment>
          ))}
          {showCollectionCreationZone && (
            <div className="flex grow flex-col justify-end px-2">
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
      className={cn("grid h-max min-h-32 w-full place-items-center", {
        "background-(--moss-primary-background) border border-dashed border-(--moss-primary)": canDrop === true,
        "background-(--moss-primary-background)/30 border border-dashed border-(--moss-primary)/30": canDrop === null,
      })}
    >
      <div className="flex flex-col items-center justify-center gap-3 p-8 text-center">
        <Icon
          icon="PlusCircle"
          className={cn("size-5 rounded-full", {
            "text-(--moss-primary)": canDrop === true,
            "text-(--moss-primary)/50": canDrop === null,
          })}
        />
        <span>Drag & drop selected items here to create a new collection</span>
      </div>
    </div>
  );
};

export default CollectionTreeView;
