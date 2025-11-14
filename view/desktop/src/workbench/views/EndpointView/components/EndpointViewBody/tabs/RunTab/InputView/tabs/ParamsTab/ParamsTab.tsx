import { Resizable, ResizablePanel } from "@/lib/ui";

import { PathParamsView } from "./components/PathParamsView";
import { QueryParamsView } from "./components/QueryParamsView";

export const ParamsTabContent = () => {
  return (
    <Resizable vertical>
      <ResizablePanel minSize={32}>
        <QueryParamsView />
      </ResizablePanel>
      <ResizablePanel minSize={32}>
        <PathParamsView />
      </ResizablePanel>
    </Resizable>
  );
};
