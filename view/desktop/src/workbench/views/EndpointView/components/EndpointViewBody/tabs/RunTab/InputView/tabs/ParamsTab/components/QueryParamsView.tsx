import { useContext, useState } from "react";

import { resourceDetailsCollection } from "@/db/resourceDetailsCollection";
import { Scrollbar } from "@/lib/ui";
import CheckboxWithLabel from "@/lib/ui/CheckboxWithLabel";
import { RoundedCounter } from "@/lib/ui/RoundedCounter";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { ActionButton } from "@/workbench/ui/components";
import { EndpointViewContext } from "@/workbench/views/EndpointView/EndpointViewContext";
import { CheckedState } from "@radix-ui/react-checkbox";
import { QueryParamInfo } from "@repo/moss-project";
import { eq, useLiveQuery } from "@tanstack/react-db";

import { extractParsedValueString } from "../../../../utils";
import { ParamDragType } from "../constants";
import { useMonitorQueryParamsRowFormDragAndDrop } from "../hooks/useMonitorQueryParamsRowFormDragAndDrop";
import { useMonitorQueryRowsDragAndDrop } from "../hooks/useMonitorQueryRowsDragAndDrop";
import { NewParamRowForm } from "./NewParamRowForm";
import { QueryParamRow } from "./QueryParamRow";

export const QueryParamsView = () => {
  const { resourceId } = useContext(EndpointViewContext);

  const [columnToFocusOnMount, setColumnToFocusOnMount] = useState<string | null>(null);

  const { data: localResourceDetails } = useLiveQuery((q) =>
    q
      .from({ collection: resourceDetailsCollection })
      .where(({ collection }) => eq(collection.id, resourceId))
      .findOne()
  );

  useMonitorQueryRowsDragAndDrop();
  useMonitorQueryParamsRowFormDragAndDrop();

  const handleParamRowChange = (updatedParam: QueryParamInfo, originalParam: QueryParamInfo) => {
    resourceDetailsCollection.update(resourceId, (draft) => {
      if (!draft) return;

      const newQueryParams = draft.queryParams.map((param) =>
        param.id === updatedParam.id
          ? {
              ...param,
              ...updatedParam,
            }
          : param
      );

      draft.queryParams = newQueryParams;

      if (updatedParam.disabled !== originalParam.disabled) {
        const splitUrl = draft.url?.split("?");
        if (splitUrl) {
          const newUrlQueryParams = newQueryParams
            .filter((param) => !param.disabled)
            .map((param) => `${param.name}=${param.value}`)
            .join("&");

          draft.url = splitUrl[0] + "?" + newUrlQueryParams;
        }
        return;
      }

      if (updatedParam.name !== originalParam.name || updatedParam.value !== originalParam.value) {
        const splitUrl = draft.url?.split("?");
        if (splitUrl) {
          const newUrlQueryParams = newQueryParams
            .filter((param) => !param.disabled)
            .map((param) => {
              return `${param.name}=${param.value ? extractParsedValueString([{ string: param.value.toString() }]) : ""}`;
            })
            .join("&");

          draft.url = splitUrl[0] + "?" + newUrlQueryParams;
        }
        return;
      }
    });
  };

  const handleParamRowDelete = (deletedParam: QueryParamInfo) => {
    resourceDetailsCollection.update(resourceId, (draft) => {
      if (!draft?.queryParams) return;

      const newQueryParams = draft.queryParams
        .filter((param) => param.id !== deletedParam.id)
        .map((param, index) => ({
          ...param,
          order: index + 1,
        }));

      draft.queryParams = newQueryParams;

      const splitUrl = draft.url?.split("?");
      if (splitUrl) {
        const newUrlQueryParams = newQueryParams.map((param) => `${param.name}=${param.value}`).join("&");
        draft.url = splitUrl[0] + "?" + newUrlQueryParams;
      }
    });
  };

  const handleAddNewRow = (queryParam: QueryParamInfo) => {
    if (queryParam.name) {
      setColumnToFocusOnMount("key");
    } else if (queryParam.value) {
      setColumnToFocusOnMount("value");
    } else {
      setColumnToFocusOnMount(null);
    }

    resourceDetailsCollection.update(resourceId, (draft) => {
      if (!draft) return;

      const newQueryParams = [
        ...draft.queryParams,
        {
          ...queryParam,
          id: Math.random().toString(36).substring(2, 15),
          disabled: false,
          propagate: false,
          order: draft.queryParams.length + 1,
        },
      ];

      draft.queryParams = newQueryParams;

      const splitUrl = draft.url?.split("?");
      if (splitUrl) {
        const baseUrl = splitUrl[0];
        const newQueryString = newQueryParams
          .filter((param) => !param.disabled)
          .map((param) => `${param.name}=${param.value}`)
          .join("&");

        draft.url = baseUrl + "?" + newQueryString;
      }
    });
  };

  const handleAllParamsCheckedChange = (checked: CheckedState) => {
    resourceDetailsCollection.update(resourceId, (draft) => {
      if (!draft) return;

      draft.queryParams = draft.queryParams.map((param) => ({
        ...param,
        disabled: checked === "indeterminate" ? false : Boolean(!checked),
      }));
    });
  };

  const allParamsChecked = localResourceDetails?.queryParams?.every((param) => !param.disabled);
  const someParamsChecked = localResourceDetails?.queryParams?.some((param) => !param.disabled);
  const howManyParamsChecked = localResourceDetails?.queryParams?.filter((param) => !param.disabled).length;

  const headerCheckedState = allParamsChecked ? true : someParamsChecked ? "indeterminate" : false;

  const sortedQueryParams = sortObjectsByOrder(localResourceDetails?.queryParams ?? []);

  return (
    <div className="flex h-full flex-col">
      <div className="border-(--moss-border) flex w-full justify-between border-b px-3 py-[5px]">
        <div className="flex items-center gap-1 overflow-hidden">
          <CheckboxWithLabel
            checked={headerCheckedState}
            onCheckedChange={handleAllParamsCheckedChange}
            label="Query Params"
            className="gap-3 truncate"
          />
          <RoundedCounter count={howManyParamsChecked ?? 0} color="gray" />
        </div>

        <div className="flex items-center gap-1">
          <ActionButton icon="MoreHorizontal" />
        </div>
      </div>

      <Scrollbar className="min-h-0 flex-1">
        <div className="grid grid-cols-[min-content_minmax(128px,1fr)_minmax(128px,1fr)_min-content_min-content_min-content] gap-2 p-3">
          {sortedQueryParams.map((param, index) => {
            const isLastRow = index === sortedQueryParams.length - 1;
            return (
              <QueryParamRow
                key={param.id}
                param={param}
                onChange={handleParamRowChange}
                onDelete={() => handleParamRowDelete(param)}
                keyToFocusOnMount={isLastRow ? columnToFocusOnMount : null}
                setColumnToFocusOnMount={setColumnToFocusOnMount}
                paramType="query"
              />
            );
          })}

          <NewParamRowForm onAdd={handleAddNewRow} paramType={ParamDragType.QUERY} key={sortedQueryParams.length} />
        </div>
      </Scrollbar>
    </div>
  );
};
