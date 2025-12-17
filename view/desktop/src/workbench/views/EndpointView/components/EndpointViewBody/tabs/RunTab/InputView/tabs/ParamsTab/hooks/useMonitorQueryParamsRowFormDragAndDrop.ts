import { useCallback, useContext, useEffect, useMemo } from "react";

import { resourceDetailsCollection } from "@/app/resourceSummariesCollection";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { EndpointViewContext } from "@/workbench/views/EndpointView/EndpointViewContext";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { eq, useLiveQuery } from "@tanstack/react-db";

import { DraggableParamRowData } from "../types";
import {
  calculateDropType,
  getDraggableParamRowSourceData,
  getFirstNewParamRowFormLocationData,
  isLocationParamRow,
  isSourceParamRow,
} from "../utils/dragAndDrop";

export const useMonitorQueryParamsRowFormDragAndDrop = () => {
  const { resourceId } = useContext(EndpointViewContext);

  const { data: localResourceDetails } = useLiveQuery((q) =>
    q
      .from({ collection: resourceDetailsCollection })
      .where(({ collection }) => eq(collection.id, resourceId))
      .findOne()
  );

  const sortedQueryList = useMemo(() => {
    return sortObjectsByOrder(localResourceDetails?.queryParams ?? []);
  }, [localResourceDetails?.queryParams]);

  const handleDragAndDropWithinQueryListNewParamRowForm = useCallback(
    (sourceData: DraggableParamRowData) => {
      if (!localResourceDetails) return;

      const queryListWithNewOrders = [
        ...sortedQueryList.filter((param) => {
          return param.id !== sourceData.data.param.id;
        }),
        sourceData.data.param,
      ].map((param, index) => ({ ...param, order: index + 1 }));

      const haveOrdersChanged = queryListWithNewOrders.some((param) => {
        const oldOrder = sortedQueryList.find((p) => p.id === param.id)?.order;
        return param.order !== oldOrder;
      });

      if (!haveOrdersChanged) return;

      resourceDetailsCollection.update(resourceId, (draft) => {
        if (!draft) return;
        draft.queryParams = queryListWithNewOrders;

        const splitUrl = draft.url?.split("?");
        if (splitUrl) {
          const newUrlQueryParams = queryListWithNewOrders
            .filter((param) => !param.disabled)
            .map((param) => `${param.name}=${param.value}`)
            .join("&");

          draft.url = splitUrl[0] + "?" + newUrlQueryParams;
        }
      });
    },
    [localResourceDetails, sortedQueryList, resourceId]
  );

  useEffect(() => {
    return monitorForElements({
      canMonitor: ({ source }) => isSourceParamRow(source),
      onDrop({ location, source }) {
        const sourceData = getDraggableParamRowSourceData(source);
        const dropTargetData = getFirstNewParamRowFormLocationData(location);

        if (!sourceData || !dropTargetData) {
          if (!isLocationParamRow(location)) {
            console.warn("Invalid source or drop target data for new param row form", {
              sourceData,
              dropTargetData,
            });
          }

          return;
        }

        const dropType = calculateDropType(sourceData, dropTargetData);
        switch (dropType) {
          case "WithinQueryList":
            handleDragAndDropWithinQueryListNewParamRowForm(sourceData);
            break;
          case "Invalid":
            break;
        }
      },
    });
  }, [handleDragAndDropWithinQueryListNewParamRowForm]);
};
