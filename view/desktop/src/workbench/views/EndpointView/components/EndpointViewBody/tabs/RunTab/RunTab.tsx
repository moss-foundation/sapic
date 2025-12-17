import { useCallback, useContext, useState } from "react";

import { resourceDetailsCollection } from "@/app/resourceSummariesCollection";
import { Resizable, ResizablePanel } from "@/lib/ui";
import { sortObjectsByOrder } from "@/utils";
import { cn } from "@/utils/cn";
import { useTokenizer } from "@/workbench/adapters/tanstackQuery/tokenizer/useTokenizer";
import { EndpointViewContext } from "@/workbench/views/EndpointView/EndpointViewContext";
import { ResourceProtocol } from "@repo/moss-project";
import { eq, useLiveQuery } from "@tanstack/react-db";

import { EndpointInputField } from "../../../EndpointInputField";
import { InputView } from "./InputView/InputView";
import { OutputView } from "./OutputView/OutputView";
import { createPathParam, createQueryParam, extractParsedValueString } from "./utils";

export const RunTab = () => {
  const { resourceId } = useContext(EndpointViewContext);
  const { mutateAsync: parseUrl } = useTokenizer();
  const { data: localResourceDetails } = useLiveQuery((q) =>
    q
      .from({ collection: resourceDetailsCollection })
      .where(({ collection }) => eq(collection.id, resourceId))
      .findOne()
  );

  const [isResizableVertical, setIsResizableVertical] = useState(false);

  const handleSendEndpoint = (method: string, url: string) => {
    console.log("Sending endpoint:", { method, url });
    // TODO: Implement actual request sending logic
    // Use getRequestUrlWithPathValues() for backend endpoints with actual path values
    setIsResizableVertical(!isResizableVertical);
  };

  const handleUrlChange = useCallback(
    (url: string) => {
      // 1. UPDATE URL IMMEDIATELY
      // We must update the URL synchronously to keep the React state in sync with the CodeMirror instance.
      // If we wait for the async `parseUrl` to finish, the parent component will pass an outdated `url` prop
      // back to the UrlEditor while the user is still typing. This mismatch causes the UrlEditor to
      // fully replace the document content, which resets the cursor position and the user's typing.
      resourceDetailsCollection.update(resourceId, (draft) => {
        if (!draft) return;
        draft.url = url;
      });

      // 2. PARSE AND UPDATE PARAMS ASYNC (Does not block typing)
      // We handle the heavy parsing logic in a detached promise chain
      parseUrl(url)
        .then((parsedUrl) => {
          try {
            resourceDetailsCollection.update(resourceId, (draft) => {
              const pathVarsSet = new Set<string>();

              parsedUrl.pathPart.forEach((part) => {
                if ("pathVariable" in part) {
                  pathVarsSet.add(part.pathVariable);
                }
              });

              // Path params logic
              const pathParamNames = Array.from(pathVarsSet.values());
              const newPathParams = pathParamNames.map((pathParamName, index) => {
                const existingPathParam = draft.pathParams?.find((param) => param.name === pathParamName);
                return createPathParam(pathParamName, index, existingPathParam);
              });
              draft.pathParams = newPathParams;

              // Query params logic
              const currentDisabledQueryParams = draft.queryParams?.filter((param) => param.disabled) ?? [];

              const newQueryParams = parsedUrl.queryPart.map((queryParam, index) => {
                const urlQueryParamName = extractParsedValueString(queryParam.key);
                const urlQueryParamValue = queryParam.value ? extractParsedValueString(queryParam.value) : "";

                const existingActiveQueryParam = draft.queryParams
                  ?.filter((param) => !param.disabled)
                  .map((param, index) => {
                    return {
                      ...param,
                      order: index + 1,
                    };
                  })
                  .find((param) => param.name === urlQueryParamName && param.order === index + 1);

                return createQueryParam(urlQueryParamName, urlQueryParamValue, index, existingActiveQueryParam);
              });

              const allQueryParams = sortObjectsByOrder([...currentDisabledQueryParams, ...newQueryParams]).map(
                (param, index) => ({
                  ...param,
                  order: index + 1,
                })
              );

              draft.queryParams = allQueryParams;
            });
          } catch (error) {
            console.error(error);
          }
        })
        .catch((error) => {
          //Technically it is normal for the parser to fail while the user is typing incomplete syntax.
          console.error(error);
        });
    },
    [parseUrl, resourceId]
  );

  const handleProtocolChange = (protocol: ResourceProtocol) => {
    resourceDetailsCollection.update(resourceId, (draft) => {
      if (!draft) return;
      draft.protocol = protocol;
    });
  };

  if (!localResourceDetails) return null;

  return (
    <div className="flex grow flex-col gap-2.5">
      <EndpointInputField
        initialProtocol={localResourceDetails.protocol}
        initialUrl={localResourceDetails?.url}
        onSend={handleSendEndpoint}
        onUrlChange={handleUrlChange}
        onProtocolChange={handleProtocolChange}
      />

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
