import { useEffect, useRef, useState } from "react";

import { DropdownMenu, Icon, Input, Resizable, ResizablePanel, Tree } from "@/components";
import { dropTargetForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import TestTreeData from "../assets/testTreeData.json";

import "@repo/moss-tabs/assets/styles.css";

import { cn } from "@/utils";

import { CreateNewCollectionFromTreeNodeEvent, NodeProps } from "./Tree/types";
import { getActualDropSourceTarget } from "./Tree/utils";

export const SidebarExplorer = () => {
  const [searchInput, setSearchInput] = useState<string>("");
  const [showCollectionCreationZone, setShowCollectionCreationZone] = useState<boolean>(false);

  const dropTargetToggleRef = useRef<HTMLDivElement>(null);

  const [collections, setCollections] = useState<NodeProps[]>([TestTreeData.tree, TestTreeData.tree]);

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
        return source.data.type === "TreeNode";
      },
      onDragStart() {
        setShowCollectionCreationZone(true);
      },
      onDragEnter() {
        setShowCollectionCreationZone(true);
      },
      onDragLeave() {
        setShowCollectionCreationZone(false);
      },
      onDrop() {
        setShowCollectionCreationZone(false);
      },
    });
  }, []);

  useEffect(() => {
    const handleCreateNewCollectionFromTreeNode = (event: CustomEvent<CreateNewCollectionFromTreeNodeEvent>) => {
      const { source } = event.detail;

      setCollections((prevCollections) => [
        ...prevCollections,
        {
          "id": "New Collection",
          "order": prevCollections.length + 1,
          "type": "folder",
          "isFolder": true,
          "isExpanded": true,
          "isRoot": true,
          "childNodes": [source.node],
        },
      ]);
    };

    window.addEventListener("createNewCollectionFromTreeNode", handleCreateNewCollectionFromTreeNode as EventListener);

    return () => {
      window.removeEventListener(
        "createNewCollectionFromTreeNode",
        handleCreateNewCollectionFromTreeNode as EventListener
      );
    };
  }, []);

  return (
    <div className="flex flex-col h-full relative " ref={dropTargetToggleRef}>
      <div className="py-1.5 pl-4 pr-2 flex items-center gap-3">
        <Input
          iconLeft="Search"
          onInput={(e) => setSearchInput((e.target as HTMLInputElement).value)}
          placeholder="Search"
          size="sm"
        />
        <DropdownMenu.Root>
          <DropdownMenu.Trigger className="text-[#717171] hover:text-[#6C707E] hover:bg-[#EBECF0] p-[5px] rounded flex items-center justify-center cursor-pointer">
            <Icon icon="Plus" />
          </DropdownMenu.Trigger>
          <DropdownMenu.Content>
            <DropdownMenu.Item label="Item" />
          </DropdownMenu.Content>
        </DropdownMenu.Root>
      </div>

      <Resizable vertical className="grow">
        {collections.map((collection) => (
          <ResizablePanel>
            <Tree tree={collection} searchInput={searchInput} />
          </ResizablePanel>
        ))}
      </Resizable>

      {showCollectionCreationZone && <CollectionCreationZone />}
    </div>
  );
};

export default SidebarExplorer;

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
    <div className={cn("absolute bottom-0 left-0 w-full h-[100px]   ")} ref={ref}>
      <div className="relative w-full h-full grid place-items-center">
        <div
          className={cn(
            "absolute w-full h-full opacity-50 bg-[repeating-linear-gradient(45deg,#000000_0,#000000_6.5px,transparent_0,transparent_50%)] bg-[size:16px_16px] z-10 bg-white",
            {
              "bg-green-300 animate-move opacity-100": canDrop,
              "bg-red-300 opacity-100": canDrop === false,
            }
          )}
        />

        <div className="bg-white px-2 py-0.5 rounded z-20 text-center w-3/4">
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
