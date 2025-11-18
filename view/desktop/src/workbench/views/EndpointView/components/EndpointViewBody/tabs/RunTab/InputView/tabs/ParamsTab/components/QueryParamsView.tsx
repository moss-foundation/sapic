import { useContext, useState } from "react";

import { Scrollbar } from "@/lib/ui";
import CheckboxWithLabel from "@/lib/ui/CheckboxWithLabel";
import { RoundedCounter } from "@/lib/ui/RoundedCounter";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { useUpdateProjectResource } from "@/workbench/adapters/tanstackQuery/project";
import { ActionButton } from "@/workbench/ui/components";
import { EndpointViewContext } from "@/workbench/views/EndpointView/EndpointViewContext";
import { CheckedState } from "@radix-ui/react-checkbox";
import { AddQueryParamParams, QueryParamInfo, UpdateQueryParamParams } from "@repo/moss-project";

import { ParamDragType } from "../constants";
import { NewParamRowForm } from "./NewParamRowForm";
import { ParamRow } from "./ParamRow";

export const QueryParamsView = () => {
  const { resourceDescription: entryDescription, resource, projectId } = useContext(EndpointViewContext);

  const { mutate: updateProjectResource } = useUpdateProjectResource();
  const [columnToFocusOnMount, setColumnToFocusOnMount] = useState<string | null>(null);

  const handleParamRowChange = (updatedParam: QueryParamInfo) => {
    const initialParam = entryDescription.queryParams.find((param) => param.id === updatedParam.id);

    if (!initialParam) return;

    const buildUpdateObject = (initial: QueryParamInfo, updated: QueryParamInfo) => {
      const updateObj: UpdateQueryParamParams = { id: updated.id };

      if (initial.name !== updated.name) updateObj.name = updated.name;

      if (initial.value !== updated.value)
        updateObj.value = {
          "UPDATE": updated.value,
        };

      if (initial.description !== updated.description && updated.description)
        updateObj.description = {
          "UPDATE": updated.description,
        };

      if (initial.order !== updated.order) updateObj.order = updated.order;

      const optionsChanged = initial.disabled !== updated.disabled || initial.propagate !== updated.propagate;

      if (optionsChanged) {
        updateObj.options = {
          disabled: updated.disabled,
          propagate: updated.propagate,
        };
      }

      return updateObj;
    };

    if (entryDescription.kind === "Item") {
      updateProjectResource({
        projectId,
        updatedResource: {
          ITEM: {
            id: resource.id,
            queryParamsToUpdate: [buildUpdateObject(initialParam, updatedParam)],
            headersToAdd: [],
            headersToUpdate: [],
            headersToRemove: [],
            pathParamsToAdd: [],
            pathParamsToUpdate: [],
            pathParamsToRemove: [],
            queryParamsToAdd: [],
            queryParamsToRemove: [],
          },
        },
      });
    }
  };

  const handleParamRowDelete = (paramId: string) => {
    const deletedParam = entryDescription.queryParams.find((param) => param.id === paramId);

    if (!deletedParam) return;

    const queryParamsToUpdate = entryDescription.queryParams
      .filter((param) => param.order! > deletedParam.order!)
      .map((param) => ({
        id: param.id,
        order: param.order! - 1,
      }));

    if (entryDescription.kind === "Item") {
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
            queryParamsToRemove: [paramId],
          },
        },
      });
    }
  };

  const handleAddNewRow = (queryParam: QueryParamInfo) => {
    if (queryParam.name) {
      setColumnToFocusOnMount("key");
    } else if (queryParam.value) {
      setColumnToFocusOnMount("value");
    } else {
      setColumnToFocusOnMount(null);
    }

    const newQueryParam: AddQueryParamParams = {
      name: queryParam.name,
      value: queryParam.value,
      order: entryDescription.queryParams.length + 1,
      options: {
        disabled: false,
        propagate: false,
      },
    };

    if (entryDescription.kind === "Item") {
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
            queryParamsToUpdate: [],
            queryParamsToRemove: [],
            queryParamsToAdd: [newQueryParam],
          },
        },
      });
    }
  };

  const handleAllParamsCheckedChange = (checked: CheckedState) => {
    if (checked === "indeterminate") return;

    updateProjectResource({
      projectId,
      updatedResource: {
        ITEM: {
          id: resource.id,
          queryParamsToUpdate: entryDescription.queryParams
            .filter((param) => param.disabled === checked)
            .map((param) => ({
              id: param.id,
              options: { disabled: !checked, propagate: param.propagate },
            })),
          headersToAdd: [],
          headersToUpdate: [],
          headersToRemove: [],
          pathParamsToAdd: [],
          pathParamsToUpdate: [],
          pathParamsToRemove: [],
          queryParamsToAdd: [],
          queryParamsToRemove: [],
        },
      },
    });
  };

  const allParamsChecked = entryDescription.queryParams.every((param) => !param.disabled);
  const someParamsChecked = entryDescription.queryParams.some((param) => !param.disabled);
  const howManyParamsChecked = entryDescription.queryParams.filter((param) => !param.disabled).length;

  const headerCheckedState = allParamsChecked ? true : someParamsChecked ? "indeterminate" : false;

  const sortedQueryParams = sortObjectsByOrder(entryDescription.queryParams);

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
          <RoundedCounter count={howManyParamsChecked} color="gray" />
        </div>

        <div className="flex items-center gap-1">
          <ActionButton icon="MoreHorizontal" />
        </div>
      </div>

      <Scrollbar className="min-h-0 flex-1">
        <div className="grid grid-cols-[min-content_minmax(128px,1fr)_minmax(128px,1fr)_min-content_min-content_min-content] gap-2 p-3">
          {sortedQueryParams.map((param, index) => {
            const isLastRow = index === entryDescription.queryParams.length - 1;
            return (
              <ParamRow
                key={param.id}
                param={param}
                onChange={handleParamRowChange}
                onDelete={() => handleParamRowDelete(param.id)}
                keyToFocusOnMount={isLastRow ? columnToFocusOnMount : null}
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
