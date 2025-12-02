import { useCallback, useState } from "react";

import { Resizable, ResizablePanel } from "@/lib/ui";
import { cn } from "@/utils/cn";
import { UrlEditor } from "@/workbench/ui/components/UrlEditor/UrlEditor";
import { useEndpointView } from "@/workbench/views/EndpointView/hooks/useEndpointView";

import { areUrlsEquivalent, parseUrl } from "../../../../utils/urlParser";
import { EndpointInputField } from "../../../EndpointInputField";
import { InputView } from "./InputView/InputView";
import { useMonitorParamsRowForms } from "./InputView/tabs/ParamsTab/hooks/useMonitorParamRowForms";
import { useMonitorParamsRows } from "./InputView/tabs/ParamsTab/hooks/useMonitorParamsRows";
import { OutputView } from "./OutputView/OutputView";

export const RunTab = () => {
  const { endpointData, httpMethod, setHttpMethod, updateEndpointData } = useEndpointView();

  const [isResizableVertical, setIsResizableVertical] = useState(false);

  const handleSendEndpoint = (method: string, url: string) => {
    console.log("Sending endpoint:", { method, url });
    // TODO: Implement actual request sending logic
    // Use getRequestUrlWithPathValues() for backend endpoints with actual path values
    setIsResizableVertical(!isResizableVertical);
  };

  const handleUrlChange = useCallback(
    (url: string) => {
      // Prevent unnecessary updates if URLs are functionally equivalent
      if (areUrlsEquivalent(url, endpointData.url.raw)) {
        return;
      }

      const parsed = parseUrl(url);

      const updatedData = {
        url: {
          raw: url,
          originalPathTemplate: parsed.url.originalPathTemplate,
          port: parsed.url.port,
          host: parsed.url.host,
          path_params: parsed.url.path_params,
          query_params: parsed.url.query_params,
        },
      };
      updateEndpointData(updatedData);
    },
    [endpointData.url.raw, updateEndpointData]
  );

  useMonitorParamsRows();
  useMonitorParamsRowForms();

  return (
    <div className="flex grow flex-col gap-2.5">
      <EndpointInputField
        initialMethod={httpMethod}
        initialUrl={endpointData.url.raw}
        onSend={handleSendEndpoint}
        onUrlChange={handleUrlChange}
        onMethodChange={(method) => {
          if (method !== httpMethod) {
            setHttpMethod(method);
          }
        }}
      />
      <UrlEditor value={endpointData.url.raw} onChange={(url) => console.log("url", url)} />

      <Resizable separator={false} key={isResizableVertical ? "vertical" : "horizontal"} vertical={isResizableVertical}>
        <ResizablePanel
          className={cn("flex flex-col", {
            "pb-1": isResizableVertical,
            "pr-1": !isResizableVertical,
          })}
          minSize={isResizableVertical ? 103 : 310}
        >
          <InputView />
        </ResizablePanel>
        <ResizablePanel
          className={cn("flex flex-col", {
            "pt-1": isResizableVertical,
            "pl-1": !isResizableVertical,
          })}
          minSize={isResizableVertical ? 103 : 310}
        >
          <OutputView />
        </ResizablePanel>
      </Resizable>
    </div>
  );
};
