import { useEffect, useState } from "react";

import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

import { Resizable, ResizablePanel } from "./components";
import Tabs from "./components/Tabs";
import { HeadBar } from "./parts/HeadBar/HeadBar";
import TabbedPane from "./parts/TabbedPane/TabbedPane";
import { swapObjectsById } from "./utils";

import "@repo/moss-tabs/assets/styles.css";

import CollectionTreeView from "./components/CollectionTreeView";

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
    <div className="background-(--moss-page-background) grid h-full grid-rows-[minmax(0px,46px)_1fr_auto] text-(--moss-text)">
      <HeadBar />
      <Resizable>
        <ResizablePanel preferredSize={270} minSize={150} maxSize={400} snap>
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

            <Tabs.Panels className="">
              {DNDList.map((item) => (
                <Tabs.Panel {...item} key={item.id} className="">
                  {item.id === 1 ? <CollectionTreeView /> : <div>{`Panel ${item.id}`}</div>}
                </Tabs.Panel>
              ))}
            </Tabs.Panels>
          </Tabs>
        </ResizablePanel>
        <ResizablePanel>
          <TabbedPane theme="dockview-theme-light" />
        </ResizablePanel>
      </Resizable>
    </div>
  );
}

export default App;
