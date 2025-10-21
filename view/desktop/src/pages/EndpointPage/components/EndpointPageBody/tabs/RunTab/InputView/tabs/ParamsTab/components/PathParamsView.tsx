import { useContext, useState } from "react";

import { ActionButton } from "@/components";
import { useUpdateProjectEntry } from "@/hooks";
import { Scrollbar } from "@/lib/ui";
import CheckboxWithLabel from "@/lib/ui/CheckboxWithLabel";
import { RoundedCounter } from "@/lib/ui/RoundedCounter";
import { EndpointPageContext } from "@/pages/EndpointPage/EndpointPageContext";
import { sortObjectsByOrder } from "@/utils/sortObjectsByOrder";
import { CheckedState } from "@radix-ui/react-checkbox";
import { AddQueryParamParams, QueryParamInfo, UpdateQueryParamParams } from "@repo/moss-project";

import { ParamDragType } from "../constants";
import { NewParamRowForm } from "./NewParamRowForm";
import { ParamRow } from "./ParamRow";

export const PathParamsView = () => {
  const { entryDescription, entry: node, projectId } = useContext(EndpointPageContext);

  const { mutate: updateProjectEntry } = useUpdateProjectEntry();
  const [columnToFocusOnMount, setColumnToFocusOnMount] = useState<string | null>(null);

  const handleParamRowChange = (updatedParam: QueryParamInfo) => {
    const initialParam = entryDescription.pathParams.find((param) => param.id === updatedParam.id);

    if (!initialParam) return;

    const buildUpdateObject = (initial: QueryParamInfo, updated: QueryParamInfo) => {
      const updateObj: UpdateQueryParamParams = { id: updated.id };

      if (initial.name !== updated.name) updateObj.name = updated.name;

      if (initial.value !== updated.value)
        updateObj.value = {
          "UPDATE": updated.value,
        };
      if (initial.order !== updated.order) updateObj.order = updated.order;
      if (initial.description !== updated.description && updated.description)
        updateObj.description = {
          "UPDATE": updated.description,
        };

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
      updateProjectEntry({
        projectId,
        updatedEntry: {
          ITEM: {
            id: node.id,
            headersToAdd: [],
            headersToUpdate: [],
            headersToRemove: [],
            pathParamsToAdd: [],
            pathParamsToUpdate: [buildUpdateObject(initialParam, updatedParam)],
            pathParamsToRemove: [],
            queryParamsToAdd: [],
            queryParamsToUpdate: [],
            queryParamsToRemove: [],
          },
        },
      });
    }
  };

  const handleParamRowDelete = (paramId: string) => {
    const deletedParam = entryDescription.pathParams.find((param) => param.id === paramId);

    if (!deletedParam) return;

    const pathParamsToUpdate = entryDescription.pathParams
      .filter((param) => param.order! > deletedParam.order!)
      .map((param) => ({
        id: param.id,
        order: param.order! - 1,
      }));

    if (entryDescription.kind === "Item") {
      updateProjectEntry({
        projectId,
        updatedEntry: {
          ITEM: {
            id: node.id,
            headersToAdd: [],
            headersToUpdate: [],
            headersToRemove: [],
            pathParamsToAdd: [],
            pathParamsToUpdate: pathParamsToUpdate,
            pathParamsToRemove: [paramId],
            queryParamsToAdd: [],
            queryParamsToUpdate: [],
            queryParamsToRemove: [],
          },
        },
      });
    }
  };

  const handleAddNewRow = (pathParam: QueryParamInfo) => {
    if (pathParam.name) {
      setColumnToFocusOnMount("key");
    } else if (pathParam.value) {
      setColumnToFocusOnMount("value");
    } else {
      setColumnToFocusOnMount(null);
    }

    const newPathParam: AddQueryParamParams = {
      name: pathParam.name,
      value: pathParam.value,
      order: entryDescription.pathParams.length + 1,
      options: {
        disabled: false,
        propagate: false,
      },
    };

    if (entryDescription.kind === "Item") {
      updateProjectEntry({
        projectId,
        updatedEntry: {
          ITEM: {
            id: node.id,
            headersToAdd: [],
            headersToUpdate: [],
            headersToRemove: [],
            pathParamsToAdd: [newPathParam],
            pathParamsToUpdate: [],
            pathParamsToRemove: [],
            queryParamsToUpdate: [],
            queryParamsToRemove: [],
            queryParamsToAdd: [],
          },
        },
      });
    }
  };

  const handleAllParamsCheckedChange = (checked: CheckedState) => {
    if (checked === "indeterminate") return;

    updateProjectEntry({
      projectId,
      updatedEntry: {
        ITEM: {
          id: node.id,
          queryParamsToUpdate: [],
          headersToAdd: [],
          headersToUpdate: [],
          headersToRemove: [],
          pathParamsToAdd: [],
          pathParamsToUpdate: entryDescription.pathParams
            .filter((param) => param.disabled === checked)
            .map((param) => ({
              id: param.id,
              options: { disabled: !checked, propagate: param.propagate },
            })),
          pathParamsToRemove: [],
          queryParamsToAdd: [],
          queryParamsToRemove: [],
        },
      },
    });
  };

  const allParamsChecked = entryDescription.pathParams.every((param) => !param.disabled);
  const someParamsChecked = entryDescription.pathParams.some((param) => !param.disabled);
  const howManyParamsChecked = entryDescription.pathParams.filter((param) => !param.disabled).length;

  const headerCheckedState = allParamsChecked ? true : someParamsChecked ? "indeterminate" : false;

  const sortedPathParams = sortObjectsByOrder(entryDescription.pathParams);

  return (
    <div className="flex h-full flex-col">
      <div className="flex w-full shrink-0 justify-between border-b border-(--moss-border) px-3 py-[5px]">
        <div className="flex items-center gap-1 overflow-hidden">
          <CheckboxWithLabel
            checked={headerCheckedState}
            onCheckedChange={handleAllParamsCheckedChange}
            label="Path Params"
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
          {sortedPathParams.map((param, index) => {
            const isLastRow = index === entryDescription.pathParams.length - 1;
            return (
              <ParamRow
                key={param.id}
                param={param}
                onChange={handleParamRowChange}
                onDelete={() => handleParamRowDelete(param.id)}
                keyToFocusOnMount={isLastRow ? columnToFocusOnMount : null}
                paramType="path"
              />
            );
          })}
          <NewParamRowForm onAdd={handleAddNewRow} paramType={ParamDragType.PATH} key={sortedPathParams.length} />
        </div>
      </Scrollbar>
    </div>
  );
};
