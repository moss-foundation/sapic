import { useCallback, useContext, useEffect, useMemo } from "react";

import { useUpdateProjectResource } from "@/adapters/tanstackQuery/project";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { EndpointViewContext } from "@/workbench/views/EndpointView/EndpointViewContext";
import { monitorForElements } from "@atlaskit/pragmatic-drag-and-drop/element/adapter";
import { AddPathParamParams, AddQueryParamParams } from "@repo/moss-project";

import { DraggableParamRowData } from "../types";
import {
  calculateDropType,
  getDraggableParamRowSourceData,
  getFirstNewParamRowFormLocationData,
  isLocationParamRow,
  isSourceParamRow,
} from "../utils/dragAndDrop";

export const useMonitorParamsRowForms = () => {
  const { resourceDescription, projectId, resource } = useContext(EndpointViewContext);
  const { mutate: updateProjectResource } = useUpdateProjectResource();

  const queryList = useMemo(() => {
    return sortObjectsByOrder(resourceDescription.queryParams);
  }, [resourceDescription.queryParams]);

  const pathList = useMemo(() => {
    return sortObjectsByOrder(resourceDescription.pathParams);
  }, [resourceDescription.pathParams]);

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
        description: queryParamToRemove.description,
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

      updateProjectResource({
        projectId,
        updatedResource: {
          ITEM: {
            id: resource.id,
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
    [queryList, pathList, updateProjectResource, projectId, resource.id]
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
        description: pathParamToRemove.description,
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

      updateProjectResource({
        projectId,
        updatedResource: {
          ITEM: {
            id: resource.id,
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
    [queryList, pathList, updateProjectResource, projectId, resource.id]
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

      updateProjectResource({
        projectId,
        updatedResource: {
          ITEM: {
            id: resource.id,
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
    [queryList, updateProjectResource, projectId, resource.id]
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

      updateProjectResource({
        projectId,
        updatedResource: {
          ITEM: {
            id: resource.id,
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
    [pathList, updateProjectResource, projectId, resource.id]
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
    });
  }, [
    handlePathToQueryNewParamRowForm,
    handleQueryToPathNewParamRowForm,
    handleWithinPathListNewParamRowForm,
    handleWithinQueryListNewParamRowForm,
  ]);
};
