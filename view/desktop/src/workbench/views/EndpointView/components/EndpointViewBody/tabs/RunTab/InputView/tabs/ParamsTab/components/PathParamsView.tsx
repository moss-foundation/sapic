import { useContext, useMemo } from "react";

import { resourceDetailsCollection } from "@/db/resourceDetailsCollection";
import { Scrollbar } from "@/lib/ui";
import { RoundedCounter } from "@/lib/ui/RoundedCounter";
import { sortObjectsByOrder } from "@/utils";
import { ActionButton } from "@/workbench/ui/components";
import { EndpointViewContext } from "@/workbench/views/EndpointView/EndpointViewContext";
import { QueryParamInfo } from "@repo/moss-project";
import { eq, useLiveQuery } from "@tanstack/react-db";

import { PathParamRow } from "./PathParamRow";

export const PathParamsView = () => {
  const { resourceId } = useContext(EndpointViewContext);

  const { data: localResourceDetails } = useLiveQuery((q) =>
    q
      .from({ collection: resourceDetailsCollection })
      .where(({ collection }) => eq(collection.id, resourceId))
      .findOne()
  );

  const handleParamRowChange = (updatedParam: QueryParamInfo) => {
    resourceDetailsCollection.update(resourceId, (draft) => {
      if (!draft?.pathParams) return;

      draft.pathParams = draft.pathParams.map((param) =>
        param.name === updatedParam.name
          ? {
              ...param,
              ...updatedParam,
            }
          : param
      );
    });
  };

  const pathParamsCount = useMemo(() => {
    return localResourceDetails?.pathParams.filter((param) => !param.disabled).length ?? 0;
  }, [localResourceDetails?.pathParams]);

  const sortedPathParams = sortObjectsByOrder(localResourceDetails?.pathParams ?? []);

  return (
    <div className="flex h-full flex-col">
      <div className="border-(--moss-border) flex w-full shrink-0 justify-between border-b px-3 py-[5px]">
        <div className="flex items-center gap-1 overflow-hidden">
          <h3>Path Params</h3>
          <RoundedCounter count={pathParamsCount ?? 0} color="gray" />
        </div>

        <div className="flex items-center gap-1">
          <ActionButton icon="MoreHorizontal" />
        </div>
      </div>

      <Scrollbar className="min-h-0 flex-1">
        <div className="grid grid-cols-2 gap-2 p-3">
          {sortedPathParams.map((param) => {
            return <PathParamRow key={param.id} param={param} onChange={handleParamRowChange} />;
          })}
        </div>
      </Scrollbar>
    </div>
  );
};
