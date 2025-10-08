import { useCallback, useContext, useEffect, useMemo } from "react";

import { useUpdateProjectEntry } from "@/hooks";
import { EndpointPageContext } from "@/pages/EndpointPage/EndpointPageContext";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { swapListByIndexWithEdge } from "@/utils/swapListByIndexWithEdge";
import { combine } from "@atlaskit/pragmatic-drag-and-drop/combine";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { AddPathParamParams, AddQueryParamParams } from "@repo/moss-project";

import { DraggableParamRowData, DropTargetNewParamRowFormData, DropTargetParamRowData } from "../types";
import {
  getDraggableParamRowSourceData,
  getFirstDraggableParamRowLocationData,
  getFirstNewParamRowFormLocationData,
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

  //param row
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
        desc: newPathParam.description,
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
        desc: newQueryParam.description,
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

  //new param row form
  const handleQueryToPathNewParamRowForm = useCallback(
    (sourceData: DraggableParamRowData) => {
      const sourceIndex = queryList.findIndex((param) => param.id === sourceData.data.param.id);
      const targetIndex = pathList.length;

      if (sourceIndex === -1 || targetIndex === -1) {
        console.error("Target not found or source index not found for query to path new param row form", {
          targetIndex,
          sourceIndex,
        });
        return;
      }

      const queryParamsToUpdate = queryList
        .filter((param) => param.order! > sourceData.data.param.order!)
        .map((param) => ({
          id: param.id,
          order: param.order! - 1,
        }));
      const queryParamToRemove = sourceData.data.param;

      const newPathParams = [
        ...pathList.slice(0, targetIndex),
        sourceData.data.param,
        ...pathList.slice(targetIndex),
      ].map((param, index) => ({ ...param, order: index + 1 }));

      const pathParamsToUpdate = newPathParams
        .filter((param) => {
          const newOrder = param.order;
          const oldOrder = pathList.find((p) => p.id === param.id)?.order;
          return newOrder !== oldOrder;
        })
        .map((param) => ({ id: param.id, order: param.order }));

      const pathParamToAdd: AddPathParamParams = {
        name: queryParamToRemove.name,
        value: queryParamToRemove.value,
        order: targetIndex + 1,
        desc: queryParamToRemove.description,
        options: {
          disabled: queryParamToRemove.disabled,
          propagate: queryParamToRemove.propagate,
        },
      };

      if (
        pathParamsToUpdate.length === 0 &&
        queryParamsToUpdate.length === 0 &&
        !pathParamToAdd &&
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
            queryParamsToRemove: [queryParamToRemove.id],
          },
        },
      });
    },
    [queryList, pathList, updateProjectEntry, projectId, entry.id]
  );

  const handlePathToQueryNewParamRowForm = useCallback(
    (sourceData: DraggableParamRowData) => {
      const sourceIndex = pathList.findIndex((param) => param.id === sourceData.data.param.id);
      const targetIndex = queryList.length;

      if (sourceIndex === -1 || targetIndex === -1) {
        console.error("Target  not found or source index not found for path to query new param row form", {
          targetIndex,
          sourceIndex,
        });
        return;
      }

      const pathParamsToUpdate = pathList
        .filter((param) => param.order! > sourceData.data.param.order!)
        .map((param) => ({
          id: param.id,
          order: param.order! - 1,
        }));
      const pathParamToRemove = sourceData.data.param;

      const newQueryParams = [
        ...queryList.slice(0, targetIndex),
        sourceData.data.param,
        ...queryList.slice(targetIndex),
      ].map((param, index) => ({ ...param, order: index + 1 }));

      const queryParamsToUpdate = newQueryParams
        .filter((param) => {
          const newOrder = param.order;
          const oldOrder = queryList.find((p) => p.id === param.id)?.order;
          return newOrder !== oldOrder;
        })
        .map((param) => ({ id: param.id, order: param.order }));

      const queryParamToAdd: AddQueryParamParams = {
        name: pathParamToRemove.name,
        value: pathParamToRemove.value,
        order: targetIndex + 1,
        desc: pathParamToRemove.description,
        options: {
          disabled: pathParamToRemove.disabled,
          propagate: pathParamToRemove.propagate,
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
            pathParamsToRemove: [pathParamToRemove.id],
            queryParamsToAdd: [queryParamToAdd],
            queryParamsToUpdate: queryParamsToUpdate,
            queryParamsToRemove: [],
          },
        },
      });
    },
    [queryList, pathList, updateProjectEntry, projectId, entry.id]
  );

  const handleWithinQueryListNewParamRowForm = useCallback(
    (sourceData: DraggableParamRowData) => {
      const updatedQueryList = [
        ...queryList.filter((param) => {
          return param.id !== sourceData.data.param.id;
        }),
        sourceData.data.param,
      ].map((param, index) => ({ ...param, order: index + 1 }));

      const queryParamsToUpdate = updatedQueryList
        .filter((param) => {
          const newOrder = param.order;
          const oldOrder = queryList.find((p) => p.id === param.id)?.order;
          return newOrder !== oldOrder;
        })
        .map((param) => ({ id: param.id, order: param.order }));

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

  const handleWithinPathListNewParamRowForm = useCallback(
    (sourceData: DraggableParamRowData) => {
      const updatedPathList = [
        ...pathList.filter((param) => {
          return param.id !== sourceData.data.param.id;
        }),
        sourceData.data.param,
      ].map((param, index) => ({ ...param, order: index + 1 }));

      const pathParamsToUpdate = updatedPathList
        .filter((param) => {
          const newOrder = param.order;
          const oldOrder = pathList.find((p) => p.id === param.id)?.order;
          return newOrder !== oldOrder;
        })
        .map((param) => ({ id: param.id, order: param.order }));

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

  useEffect(() => {
    return combine(
      monitorForElements({
        canMonitor({ source }) {
          return isSourceParamRow(source);
        },
        onDrop({ location, source }) {
          const sourceData = getDraggableParamRowSourceData(source);
          const dropTargetData = getFirstDraggableParamRowLocationData(location);

          if (!sourceData || !dropTargetData || !dropTargetData.edge) {
            console.warn("Invalid source or drop target data or edge for param row", {
              sourceData,
              dropTargetData,
            });
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
      }),
      monitorForElements({
        canMonitor({ source }) {
          return isSourceParamRow(source);
        },
        onDrop({ location, source }) {
          const sourceData = getDraggableParamRowSourceData(source);
          const dropTargetData = getFirstNewParamRowFormLocationData(location);

          if (!sourceData || !dropTargetData) {
            console.warn("Invalid source or drop target data for new param row form", {
              sourceData,
              dropTargetData,
            });
            return;
          }

          const dropType = calculateDropType(sourceData, dropTargetData);
          switch (dropType) {
            case "WithinQueryList":
              handleWithinQueryListNewParamRowForm(sourceData);
              break;
            case "WithinPathList":
              handleWithinPathListNewParamRowForm(sourceData);
              break;
            case "QueryToPath":
              handleQueryToPathNewParamRowForm(sourceData);
              break;
            case "PathToQuery":
              handlePathToQueryNewParamRowForm(sourceData);
              break;
            case "Invalid":
              break;
          }
        },
      })
    );
  }, [
    handlePathToQuery,
    handlePathToQueryNewParamRowForm,
    handleQueryToPath,
    handleQueryToPathNewParamRowForm,
    handleWithinPathList,
    handleWithinPathListNewParamRowForm,
    handleWithinQueryList,
    handleWithinQueryListNewParamRowForm,
  ]);
};

export const calculateDropType = (
  sourceData: DraggableParamRowData,
  dropTargetData: DropTargetParamRowData | DropTargetNewParamRowFormData
) => {
  if (sourceData.data.paramType === "query" && dropTargetData.data.paramType === "query") {
    return "WithinQueryList";
  }

  if (sourceData.data.paramType === "path" && dropTargetData.data.paramType === "path") {
    return "WithinPathList";
  }

  if (sourceData.data.paramType === "query" && dropTargetData.data.paramType === "path") {
    return "QueryToPath";
  }

  if (sourceData.data.paramType === "path" && dropTargetData.data.paramType === "query") {
    return "PathToQuery";
  }

  return "Invalid";
};
