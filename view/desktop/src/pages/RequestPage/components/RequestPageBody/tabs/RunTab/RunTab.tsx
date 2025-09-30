import { useCallback, useState } from "react";

import { Resizable, ResizablePanel } from "@/lib/ui";
import { cn } from "@/utils/cn";
import { IDockviewPanelProps } from "@repo/moss-tabs";

import { useRequestPage } from "../../../../hooks/useRequestPage";
import { RequestPageProps } from "../../../../RequestPage";
import { areUrlsEquivalent, parseUrl } from "../../../../utils/urlParser";
import { RequestInputField } from "../../../RequestInputField";
import { InputView } from "./InputView/InputView";
import { OutputView } from "./OutputView/OutputView";

export const RunTab = ({ ...props }: IDockviewPanelProps<RequestPageProps>) => {
  const { requestData, httpMethod, setHttpMethod, updateRequestData } = useRequestPage();
  const [isResizableVertical, setIsResizableVertical] = useState(false);

  const handleSendRequest = (method: string, url: string) => {
    console.log("Sending request:", { method, url });
    // TODO: Implement actual request sending logic
    // Use getRequestUrlWithPathValues() for backend requests with actual path values
    setIsResizableVertical(!isResizableVertical);
  };

  const handleUrlChange = useCallback(
    (url: string) => {
      // Prevent unnecessary updates if URLs are functionally equivalent
      if (areUrlsEquivalent(url, requestData.url.raw)) {
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
      updateRequestData(updatedData);
    },
    [requestData.url.raw, updateRequestData]
  );

  return (
    <div className="flex grow flex-col gap-3">
      <RequestInputField
        initialMethod={httpMethod}
        initialUrl={requestData.url.raw}
        onSend={handleSendRequest}
        onUrlChange={handleUrlChange}
        onMethodChange={(method) => {
          if (method !== httpMethod) {
            setHttpMethod(method);
          }
        }}
      />

      <Resizable separator={false} key={isResizableVertical ? "vertical" : "horizontal"} vertical={isResizableVertical}>
        <ResizablePanel
          className={cn("flex flex-col", {
            "pb-1": isResizableVertical,
            "pr-1": !isResizableVertical,
          })}
        >
          <InputView {...props} />
        </ResizablePanel>
        <ResizablePanel
          className={cn("flex flex-col", {
            "pt-1": isResizableVertical,
            "pl-1": !isResizableVertical,
          })}
        >
          <OutputView {...props} />
        </ResizablePanel>
      </Resizable>
    </div>
  );
};
