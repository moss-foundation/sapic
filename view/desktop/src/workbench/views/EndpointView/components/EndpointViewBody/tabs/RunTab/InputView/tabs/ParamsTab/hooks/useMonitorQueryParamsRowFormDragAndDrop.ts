import { useCallback, useContext, useEffect, useMemo } from "react";

import { useGetLocalResourceDetails } from "@/db/resourceDetails/hooks/useGetLocalResourceDetails";
import { resourceDetailsCollection } from "@/db/resourceDetails/resourceDetailsCollection";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { EndpointViewContext } from "@/workbench/views/EndpointView/EndpointViewContext";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";

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

  const localResourceDetails = useGetLocalResourceDetails(resourceId);

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
        draft.metadata.isDirty = true;
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
