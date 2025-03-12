import { useEffect, useRef, useState } from "react";

import { DropdownMenu, Icon, Input, Scrollbar, Tree } from "@/components";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import TestTreeData from "../assets/testTreeData.json";
import TestTreeData2 from "../assets/testTreeData2.json";
import TestTreeData3 from "../assets/testTreeData3.json";

import "@repo/moss-tabs/assets/styles.css";

import { cn, swapListById } from "@/utils";
import { Edge } from "@atlaskit/pragmatic-drag-and-drop-hitbox/dist/types/types";

import { Collection, CreateNewCollectionFromTreeNodeEvent } from "./Tree/types";
import { getActualDropSourceTarget } from "./Tree/utils";

export const CollectionTreeView = () => {
  const [searchInput, setSearchInput] = useState<string>("");
  const [showCollectionCreationZone, setShowCollectionCreationZone] = useState<boolean>(false);

  const dropTargetToggleRef = useRef<HTMLDivElement>(null);

  const [collections, setCollections] = useState<Collection[]>([
    TestTreeData as Collection,
    TestTreeData2 as Collection,
    TestTreeData3 as Collection,
  ]);

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

        setCollections(
          (prevCollections) => swapListById(sourceCollectionId, locationCollectionId, prevCollections, closestEdge)!
        );
      },
    });
  }, []);

  useEffect(() => {
    const handleCreateNewCollectionFromTreeNode = (event: CustomEvent<CreateNewCollectionFromTreeNodeEvent>) => {
      const { source } = event.detail;
      const newTreeId = `collectionId${collections.length + 1}`;

      setCollections((prevCollections) => {
        return [
          ...prevCollections,
          {
            id: newTreeId,
            type: "collection",
            order: prevCollections.length + 1,
            tree: {
              "id": "New Collection",
              "order": prevCollections.length + 1,
              "type": "folder",
              "isFolder": true,
              "isExpanded": true,
              "isRoot": true,
              "childNodes": [source.node],
            },
          },
        ];
      });
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
  }, [collections.length]);

  return (
    <div className="relative flex h-full flex-col" ref={dropTargetToggleRef}>
      <div className="flex items-center gap-3 py-1.5 pr-2 pl-4">
        <Input
          iconLeft="Search"
          onInput={(e) => setSearchInput((e.target as HTMLInputElement).value)}
          placeholder="Search"
          size="sm"
        />
        <DropdownMenu.Root>
          <DropdownMenu.Trigger className="flex cursor-pointer items-center justify-center rounded p-[5px] text-[#717171] hover:bg-[#EBECF0] hover:text-[#6C707E]">
            <Icon icon="Plus" />
          </DropdownMenu.Trigger>
          <DropdownMenu.Content>
            <DropdownMenu.Item label="Item" />
          </DropdownMenu.Content>
        </DropdownMenu.Root>
      </div>

      <Scrollbar className="h-full">
        {collections.map((collection) => (
          <Tree tree={collection.tree} id={collection.id} key={collection.id} searchInput={searchInput} />
        ))}
      </Scrollbar>
      {showCollectionCreationZone && <CollectionCreationZone />}
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
    <div className={cn("absolute bottom-0 left-0 h-[100px] w-full")} ref={ref}>
      <div className="relative grid h-full w-full place-items-center">
        <div
          className={cn(
            "absolute z-10 h-full w-full bg-white bg-[repeating-linear-gradient(45deg,#000000_0,#000000_6.5px,transparent_0,transparent_50%)] bg-[size:16px_16px] opacity-50",
            {
              "animate-move bg-green-300 opacity-100": canDrop,
              "bg-red-300 opacity-100": canDrop === false,
            }
          )}
        />

        <div className="z-20 w-3/4 rounded bg-white px-2 py-0.5 text-center">
          {canDrop === false ? (
            <span>Cannot create new collection from this</span>
          ) : (
            <span>Drop to create new collection</span>
          )}
        </div>
      </div>
    </div>
  );
};

export default CollectionTreeView;
