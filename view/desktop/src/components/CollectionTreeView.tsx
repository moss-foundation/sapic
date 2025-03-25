import { useEffect, useRef, useState } from "react";

import { DropdownMenu, Icon, Input, Scrollbar, Tree } from "@/components";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import AzureDevOpsTestCollection from "../assets/AzureDevOpsTestCollection.json";
import SapicTestCollection from "../assets/SapicTestCollection.json";
import WhatsAppBusinessTestCollection from "../assets/WhatsAppBusinessTestCollection.json";

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
    SapicTestCollection as Collection,
    AzureDevOpsTestCollection as Collection,
    WhatsAppBusinessTestCollection as Collection,
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
    <div className="relative flex h-full flex-col pt-1 select-none" ref={dropTargetToggleRef}>
      <div className="flex items-center gap-[7px] py-1.5 pr-[7px] pl-4">
        <Input
          iconLeft="Search"
          variant="outlined"
          onInput={(e) => setSearchInput((e.target as HTMLInputElement).value)}
          placeholder="Search"
          size="sm"
        />
        <DropdownMenu.Root>
          <DropdownMenu.Trigger className="background-(--moss-treeNodeButton-bg) hover:background-(--moss-treeNodeButton-bg-hover) flex cursor-pointer items-center justify-center rounded p-[5px] text-(--moss-treeNodeButton-text)">
            <Icon icon="Plus" />
          </DropdownMenu.Trigger>
          <DropdownMenu.Content>
            <DropdownMenu.Item label="Edit" />
            <DropdownMenu.Item label="Duplicate" />
            <DropdownMenu.Separator />
            <DropdownMenu.Item label="Archive" />
            <DropdownMenu.Sub>
              <DropdownMenu.SubTrigger label="More" />
              <DropdownMenu.SubContent>
                <DropdownMenu.Item label="Move to project…" />
                <DropdownMenu.Item label="Move to folder…" />

                <DropdownMenu.Separator />
                <DropdownMenu.Item label="Advanced options…" />
              </DropdownMenu.SubContent>
            </DropdownMenu.Sub>
            <DropdownMenu.Separator />
            <DropdownMenu.Item label="Share" />
            <DropdownMenu.Item label="Add to favorites" />
            <DropdownMenu.Separator />
            <DropdownMenu.Item label="Delete" />

            <DropdownMenu.Separator />

            <DropdownMenu.CheckboxItem label="Hide from sidebar" checked />
            <DropdownMenu.CheckboxItem label="Hide from sidebar" />
            <DropdownMenu.CheckboxItem label="Hide from sidebar" checked />

            <DropdownMenu.Separator />
            <DropdownMenu.RadioGroup>
              <DropdownMenu.RadioItem value="1" label="Hide from sidebar" checked={false} />
              <DropdownMenu.RadioItem value="2" label="Hide from sidebar" checked={true} />
              <DropdownMenu.RadioItem value="3" label="Hide from sidebar" checked={false} />
            </DropdownMenu.RadioGroup>
          </DropdownMenu.Content>
        </DropdownMenu.Root>
      </div>

      <Scrollbar className="h-full">
        <div className="flex h-full flex-col">
          {collections.map((collection) => (
            <Tree tree={collection.tree} id={collection.id} key={collection.id} searchInput={searchInput} />
          ))}
          {showCollectionCreationZone && (
            <div className="flex grow flex-col justify-end">
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
    <div ref={ref} className={cn("relative mb-8 grid h-max min-h-32 w-full place-items-center")}>
      <div
        className={cn("animate-move absolute top-0 left-0 z-10 h-full w-full", {
          "bg-white bg-[repeating-linear-gradient(45deg,#edf6ff_0,#edf6ff_6.5px,transparent_0,transparent_50%)] bg-[size:16px_16px]":
            canDrop === true,
          "bg-[#F4F4F4]": canDrop === null,
        })}
      />
      <div className="z-20 flex flex-col items-center justify-center gap-3 rounded p-8 text-center text-(--moss-text)">
        <Icon
          icon="AddCircle"
          className={cn("size-5 text-[#717171]", {
            "text-(--moss-primary)": canDrop,
          })}
        />
        <span className="text-black">Drag & drop selected items here to create a new collection</span>
      </div>
    </div>
  );
};

export default CollectionTreeView;
