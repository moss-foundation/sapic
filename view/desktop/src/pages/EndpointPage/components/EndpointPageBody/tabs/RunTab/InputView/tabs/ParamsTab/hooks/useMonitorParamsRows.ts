import { useCallback, useContext, useEffect, useMemo } from "react";

import { useUpdateProjectEntry } from "@/hooks";
import { EndpointPageContext } from "@/pages/EndpointPage/EndpointPageContext";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { swapListByIndexWithEdge } from "@/utils/swapListByIndexWithEdge";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { AddPathParamParams, AddQueryParamParams } from "@repo/moss-project";

import { DraggableParamRowData, DropTargetParamRowData } from "../types";
import {
  calculateDropType,
  getDraggableParamRowSourceData,
  getFirstDraggableParamRowLocationData,
  isLocationNewParamRowForm,
  isSourceParamRow,
} from "../utils/dragAndDrop";

export const useMonitorParamsRows = () => {
  const { entryDescription, projectId, entry } = useContext(EndpointPageContext);
  const { mutate: updateProjectEntry } = useUpdateProjectEntry();

  const queryList = useMemo(() => {
    return sortObjectsByOrder(entryDescription.queryParams);
  }, [entryDescription.queryParams]);

  const pathList = useMemo(() => {
    return sortObjectsByOrder(entryDescription.pathParams);
  }, [entryDescription.pathParams]);

  const handleWithinQueryList = useCallback(
    (sourceData: DraggableParamRowData, dropTargetData: DropTargetParamRowData) => {
      const sourceIndex = queryList.findIndex((param) => param.id === sourceData.data.param.id);
      const targetIndex = queryList.findIndex((param) => param.id === dropTargetData.data.param.id);
      const edge = dropTargetData.edge;

      if (sourceIndex === -1 || targetIndex === -1 || !edge) {
        console.error("Source, target or edge not found", { sourceIndex, targetIndex, edge });
        return;
      }

      const newQueryList = swapListByIndexWithEdge(sourceIndex, targetIndex, queryList, edge);
      const newQueryListWithNewOrders = newQueryList.map((param, index) => ({ ...param, order: index + 1 }));
      const queryParamsToUpdate = newQueryListWithNewOrders
        .filter((param) => {
          const newOrder = param.order;
          const oldOrder = queryList.find((p) => p.id === param.id)?.order;
          return newOrder !== oldOrder;
        })
        .map((param) => ({
          id: param.id,
          order: param.order,
        }));

      if (queryParamsToUpdate.length === 0) return;

      updateProjectEntry({
        projectId,
        updatedEntry: {
          ITEM: {
            id: entry.id,
            headersToAdd: [],
            headersToUpdate: [],
            headersToRemove: [],
            pathParamsToAdd: [],
            pathParamsToUpdate: [],
            pathParamsToRemove: [],
            queryParamsToAdd: [],
            queryParamsToUpdate: queryParamsToUpdate,
            queryParamsToRemove: [],
          },
        },
      });
    },
    [queryList, updateProjectEntry, projectId, entry.id]
  );

  const handleWithinPathList = useCallback(
    (sourceData: DraggableParamRowData, dropTargetData: DropTargetParamRowData) => {
      const sourceIndex = pathList.findIndex((param) => param.id === sourceData.data.param.id);
      const targetIndex = pathList.findIndex((param) => param.id === dropTargetData.data.param.id);
      const edge = dropTargetData.edge;

      if (sourceIndex === -1 || targetIndex === -1 || !edge) {
        console.error("Source, target or edge not found", { sourceIndex, targetIndex, edge });
        return;
      }

      const newPathList = swapListByIndexWithEdge(sourceIndex, targetIndex, pathList, edge);
      const newPathListWithNewOrders = newPathList.map((param, index) => ({ ...param, order: index + 1 }));
      const pathParamsToUpdate = newPathListWithNewOrders
        .filter((param) => {
          const newOrder = param.order;
          const oldOrder = pathList.find((p) => p.id === param.id)?.order;
          return newOrder !== oldOrder;
        })
        .map((param) => ({
          id: param.id,
          order: param.order,
        }));

      if (pathParamsToUpdate.length === 0) return;

      updateProjectEntry({
        projectId,
        updatedEntry: {
          ITEM: {
            id: entry.id,
            headersToAdd: [],
            headersToUpdate: [],
            headersToRemove: [],
            pathParamsToAdd: [],
            pathParamsToUpdate: pathParamsToUpdate,
            pathParamsToRemove: [],
            queryParamsToAdd: [],
            queryParamsToUpdate: [],
            queryParamsToRemove: [],
          },
        },
      });
    },
    [pathList, updateProjectEntry, projectId, entry.id]
  );

  const handleQueryToPath = useCallback(
    (sourceData: DraggableParamRowData, dropTargetData: DropTargetParamRowData) => {
      const sourceIndex = queryList.findIndex((param) => param.id === sourceData.data.param.id);
      const targetIndex = pathList.findIndex((param) => param.id === dropTargetData.data.param.id);
      const edge = dropTargetData.edge;

      if (sourceIndex === -1 || targetIndex === -1 || !edge) {
        console.error("Source, target or edge not found", { sourceIndex, targetIndex, edge });
        return;
      }

      const queryParamsToUpdate = queryList
        .filter((param) => param.order! > sourceData.data.param.order!)
        .map((param) => ({
          id: param.id,
          order: param.order! - 1,
        }));

      const queryParamToRemove = sourceData.data.param.id;

      const targetIndexWithEdge = targetIndex + (edge === "top" ? 0 : 1);
      const newPathList = [
        ...pathList.slice(0, targetIndexWithEdge),
        sourceData.data.param,
        ...pathList.slice(targetIndexWithEdge),
      ];
      const newPathListWithNewOrders = newPathList.map((param, index) => ({ ...param, order: index + 1 }));
      const pathParamsToUpdate = newPathListWithNewOrders
        .filter((param) => {
          const newOrder = param.order;
          const oldOrder = pathList.find((p) => p.id === param.id)?.order;

          const isDifferentOrder = newOrder !== oldOrder;

          const isNewItem = param.id === sourceData.data.param.id;

          if (isNewItem) return false;

          return isDifferentOrder;
        })
        .map((param) => ({
          id: param.id,
          order: param.order,
        }));

      const newPathParam = newPathListWithNewOrders.find((param) => param.id === sourceData.data.param.id)!;
      const pathParamToAdd: AddPathParamParams = {
        name: newPathParam.name,
        value: newPathParam.value,
        order: newPathParam.order,
        description: newPathParam.description,
        options: {
          disabled: newPathParam.disabled,
          propagate: newPathParam.propagate,
        },
      };

      if (
        pathParamsToUpdate.length === 0 &&
        queryParamsToUpdate.length === 0 &&
        !pathParamToAdd.name &&
        !queryParamToRemove
      ) {
        return;
      }

      updateProjectEntry({
        projectId,
        updatedEntry: {
          ITEM: {
            id: entry.id,
            headersToAdd: [],
            headersToUpdate: [],
            headersToRemove: [],
            pathParamsToAdd: [pathParamToAdd],
            pathParamsToUpdate: pathParamsToUpdate,
            pathParamsToRemove: [],
            queryParamsToAdd: [],
            queryParamsToUpdate: queryParamsToUpdate,
            queryParamsToRemove: [queryParamToRemove],
          },
        },
      });
    },
    [queryList, pathList, updateProjectEntry, projectId, entry.id]
  );

  const handlePathToQuery = useCallback(
    (sourceData: DraggableParamRowData, dropTargetData: DropTargetParamRowData) => {
      const sourceIndex = pathList.findIndex((param) => param.id === sourceData.data.param.id);
      const targetIndex = queryList.findIndex((param) => param.id === dropTargetData.data.param.id);
      const edge = dropTargetData.edge;

      if (sourceIndex === -1 || targetIndex === -1 || !edge) {
        console.error("Source, target or edge not found", { sourceIndex, targetIndex, edge });
        return;
      }

      const pathParamsToUpdate = pathList
        .filter((param) => param.order! > sourceData.data.param.order!)
        .map((param) => ({
          id: param.id,
          order: param.order! - 1,
        }));

      const pathParamToRemove = sourceData.data.param.id;

      const targetIndexWithEdge = targetIndex + (edge === "top" ? 0 : 1);
      const newQueryList = [
        ...queryList.slice(0, targetIndexWithEdge),
        sourceData.data.param,
        ...queryList.slice(targetIndexWithEdge),
      ];
      const newQueryListWithNewOrders = newQueryList.map((param, index) => ({ ...param, order: index + 1 }));
      const queryParamsToUpdate = newQueryListWithNewOrders
        .filter((param) => {
          const newOrder = param.order;
          const oldOrder = queryList.find((p) => p.id === param.id)?.order;

          const isDifferentOrder = newOrder !== oldOrder;

          const isNewItem = param.id === sourceData.data.param.id;

          if (isNewItem) return false;

          return isDifferentOrder;
        })
        .map((param) => ({
          id: param.id,
          order: param.order,
        }));

      const newQueryParam = newQueryListWithNewOrders.find((param) => param.id === sourceData.data.param.id)!;
      const queryParamToAdd: AddQueryParamParams = {
        name: newQueryParam.name,
        value: newQueryParam.value,
        order: newQueryParam.order,
        description: newQueryParam.description,
        options: {
          disabled: newQueryParam.disabled,
          propagate: newQueryParam.propagate,
        },
      };

      if (
        pathParamsToUpdate.length === 0 &&
        queryParamsToUpdate.length === 0 &&
        !pathParamToRemove &&
        !queryParamToAdd
      ) {
        return;
      }

      updateProjectEntry({
        projectId,
        updatedEntry: {
          ITEM: {
            id: entry.id,
            headersToAdd: [],
            headersToUpdate: [],
            headersToRemove: [],
            pathParamsToAdd: [],
            pathParamsToUpdate: pathParamsToUpdate,
            pathParamsToRemove: [pathParamToRemove],
            queryParamsToAdd: [queryParamToAdd],
            queryParamsToUpdate: queryParamsToUpdate,
            queryParamsToRemove: [],
          },
        },
      });
    },
    [queryList, pathList, updateProjectEntry, projectId, entry.id]
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
            handleWithinQueryList(sourceData, dropTargetData);
            break;
          case "WithinPathList":
            handleWithinPathList(sourceData, dropTargetData);
            break;
          case "QueryToPath":
            handleQueryToPath(sourceData, dropTargetData);
            break;
          case "PathToQuery":
            handlePathToQuery(sourceData, dropTargetData);
            break;
          case "Invalid":
            break;
        }
      },
    });
  }, [handlePathToQuery, handleQueryToPath, handleWithinPathList, handleWithinQueryList]);
};
