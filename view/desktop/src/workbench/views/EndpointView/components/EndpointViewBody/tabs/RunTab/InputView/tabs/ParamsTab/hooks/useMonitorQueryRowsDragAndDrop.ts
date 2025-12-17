import { useCallback, useContext, useEffect, useMemo } from "react";

import { resourceDetailsCollection } from "@/db/resourceDetailsCollection";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { swapListByIndexWithEdge } from "@/workbench/utils/swapListByIndexWithEdge";
import { EndpointViewContext } from "@/workbench/views/EndpointView/EndpointViewContext";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { eq, useLiveQuery } from "@tanstack/react-db";

import { DraggableParamRowData, DropTargetParamRowData } from "../types";
import {
  calculateDropType,
  getDraggableParamRowSourceData,
  getFirstDraggableParamRowLocationData,
  isLocationNewParamRowForm,
  isSourceParamRow,
} from "../utils/dragAndDrop";

export const useMonitorQueryRowsDragAndDrop = () => {
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

  const handleDragAndDropWithinQueryList = useCallback(
    (sourceData: DraggableParamRowData, dropTargetData: DropTargetParamRowData) => {
      if (!localResourceDetails) return;

      const sourceIndex = sortedQueryList.findIndex((param) => param.id === sourceData.data.param.id);
      const targetIndex = sortedQueryList.findIndex((param) => param.id === dropTargetData.data.param.id);
      const edge = dropTargetData.edge;

      if (sourceIndex === -1 || targetIndex === -1 || !edge) {
        console.error("Source, target or edge not found", { sourceIndex, targetIndex, edge });
        return;
      }

      const newQueryList = swapListByIndexWithEdge(sourceIndex, targetIndex, sortedQueryList, edge);
      const newQueryListWithNewOrders = newQueryList.map((param, index) => ({ ...param, order: index + 1 }));
      const haveOrdersChanged = newQueryListWithNewOrders.some((param) => {
        const oldOrder = sortedQueryList.find((p) => p.id === param.id)?.order;
        return param.order !== oldOrder;
      });

      if (!haveOrdersChanged) return;

      resourceDetailsCollection.update(resourceId, (draft) => {
        if (!draft) return;
        draft.queryParams = newQueryListWithNewOrders;

        const splitUrl = draft.url?.split("?");
        if (splitUrl) {
          const newUrlQueryParams = newQueryListWithNewOrders
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
        const dropTargetData = getFirstDraggableParamRowLocationData(location);

        if (!sourceData || !dropTargetData || !dropTargetData.edge) {
          if (!isLocationNewParamRowForm(location)) {
            console.warn("Invalid source or drop target data or edge for param row", {
              sourceData,
              dropTargetData,
            });
          }

          return;
        }

        const dropType = calculateDropType(sourceData, dropTargetData);
        switch (dropType) {
          case "WithinQueryList":
            handleDragAndDropWithinQueryList(sourceData, dropTargetData);
            break;
          case "Invalid":
            break;
        }
      },
    });
  }, [handleDragAndDropWithinQueryList]);
};
