import { useEffect, useState } from "react";

import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import TestTreeData from "./assets/testTreeData.json";
import { Resizable, ResizablePanel, Scrollbar, Tree } from "./components";
import Tabs from "./components/Tabs";
import { NodeProps } from "./components/Tree/types";
import { swapObjectsById } from "./utils";

interface ListItem {
  id: number;
  label: string;
  isActive: boolean;
}

const initialList = Array.from({ length: 5 }, (_, i) => {
  if (i === 0)
    return {
      id: i + 1,
      label: `Explorer`,
      isActive: i === 0,
    };

  if (i === 1)
    return {
      id: i + 1,
      label: `Issues`,
      isActive: false,
    };

  if (i === 2)
    return {
      id: i + 1,
      label: `History`,
      isActive: false,
    };

  return {
    id: i + 1,
    label: `Panel ${i + 1}`,
    isActive: i === 0,
  };
});

function App() {
  const [theme, setTheme] = useState("dark");

  const toggleTheme = () => {
    setTheme(theme === "dark" ? "light" : "dark");
    document.querySelector("html")!.setAttribute("data-theme", theme === "dark" ? "light" : "dark");
  };

  const [DNDList, setDNDList] = useState<ListItem[]>(initialList);

  const handleSetActive = (id: number) => {
    setDNDList([...DNDList.map((item) => ({ ...item, isActive: item.id === id }))]);
  };

  useEffect(() => {
    return monitorForElements({
      onDrop({ location, source }) {
        const target = location.current.dropTargets[0];
        if (!target || target.data.draggableType !== "WidgetBarButton") return;

        const sourceData = source.data as unknown as ListItem;
        const targetData = target.data as unknown as ListItem;

        if (!sourceData || !targetData) return;

        const updatedItems = swapObjectsById(sourceData, targetData, DNDList);

        if (!updatedItems) return;

        setDNDList(updatedItems);
      },
    });
  }, [DNDList]);

  return (
    <div className="flex w-full h-full">
      <Resizable>
        <ResizablePanel preferredSize={270} minSize={100} maxSize={400} snap>
          <Tabs>
            <Tabs.List>
              {DNDList.map((item) => (
                <Tabs.Tab
                  {...item}
                  key={item.id}
                  isDraggable
                  onClick={() => handleSetActive(item.id)}
                  draggableType="WidgetBarButton"
                />
              ))}
            </Tabs.List>

            <Tabs.Panels className="text-black dark:text-white">
              {DNDList.map((item) => (
                <Tabs.Panel {...item} key={item.id}>
                  {item.id === 1 ? <IsolatedTreeComponent /> : <div>{`Panel ${item.id}`}</div>}
                </Tabs.Panel>
              ))}
            </Tabs.Panels>
          </Tabs>
        </ResizablePanel>
        <ResizablePanel>
          <main className="h-screen flex grow flex-col justify-center text-center bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-gray-100 font-sans transition">
            <Scrollbar>
              {Array.from({ length: 100 }, (_, i) => (
                <div key={i} className="h-10 mb-1 w-full bg-gray-200 dark:bg-[#131313]">
                  {i + 1}
                </div>
              ))}
            </Scrollbar>
          </main>
        </ResizablePanel>
      </Resizable>

      <div className="absolute -top-3 -right-3 p-4 flex">
        <div className="" />
        <button onClick={toggleTheme} className="cursor-pointer">
          {theme === "light" ? (
            <svg
              className="size-9 text-black hover:text-gray-500 "
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="1.5"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" />
            </svg>
          ) : (
            <svg
              className="size-9 text-white hover:text-black/50"
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="1.5"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <circle cx="12" cy="12" r="4" />
              <path d="M12 2v2" />
              <path d="M12 20v2" />
              <path d="m4.93 4.93 1.41 1.41" />
              <path d="m17.66 17.66 1.41 1.41" />
              <path d="M2 12h2" />
              <path d="M20 12h2" />
              <path d="m6.34 17.66-1.41 1.41" />
              <path d="m19.07 4.93-1.41 1.41" />
            </svg>
          )}
        </button>
      </div>

      {/* <TestDropTarget /> */}
    </div>
  );
}

export default App;

const IsolatedTreeComponent = () => {
  const [treeItems, setTreeItems] = useState<NodeProps[]>(TestTreeData.items);

  const handleNodeUpdate = (node: NodeProps) => {
    console.log("Node updated:", node);
  };

  const handleNodeExpand = (node: NodeProps) => {
    // console.log("Node expanded:", node);
  };

  const handleNodeCollapse = (node: NodeProps) => {
    // console.log("Node collapsed:", node);
  };

  const handleTreeUpdate = (updatedTree: NodeProps[]) => {
    setTreeItems(updatedTree);
  };

  return (
    <>
      <Tree
        nodes={treeItems}
        onNodeUpdate={handleNodeUpdate}
        onNodeExpand={handleNodeExpand}
        onNodeCollapse={handleNodeCollapse}
        onTreeUpdate={handleTreeUpdate}
      />
      {/* <div className="absolute h-screen -top-3 right-0 p-4 flex flex-col gap-1 text-xs bg-gray-800 overflow-auto">
        <pre>
          <code>{JSON.stringify(treeItems, null, 2)}</code>
        </pre>
      </div> */}
    </>
  );
};
