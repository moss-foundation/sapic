import { useState } from "react";

import { ActionButton } from "@/components";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { Scrollbar } from "@/lib/ui";
import { Counter } from "@/lib/ui/RoundedCounter";

import { NewParamRowForm } from "./NewParamRowForm";
import { ParamRow } from "./ParamRow";
import { ParamProps } from "./types";

export const QueryParamsView = () => {
  const [params, setParams] = useState<ParamProps[]>([
    {
      id: "1",
      checked: false,
      key: "id",
      value: "716d8407-dd06-43d5-8957-074af3dc09ae",
      isRequired: true,
      type: "string",
    },
    {
      id: "2",
      checked: false,
      key: "sort_by",
      value: "ASC",
      isRequired: true,
      type: "string",
    },
  ]);
  const [columnToFocusOnMount, setColumnToFocusOnMount] = useState<string | null>(null);

  const handleParamRowChange = (updatedParam: ParamProps) => {
    setParams(params.map((p) => (p.id === updatedParam.id ? updatedParam : p)));
  };

  const addNewRowAtTheEnd = (queryParam: ParamProps) => {
    if (queryParam.key) {
      setColumnToFocusOnMount("key");
    } else if (queryParam.value) {
      setColumnToFocusOnMount("value");
    } else {
      setColumnToFocusOnMount(null);
    }

    setParams((prev) => [...prev, { ...queryParam, id: Math.random().toString(36).substring(2, 15) }]);
  };

  const allParamsChecked = params.every((param) => param.checked);
  const someParamsChecked = params.some((param) => param.checked);

  const headerCheckedState = allParamsChecked ? true : someParamsChecked ? "indeterminate" : false;

  return (
    <div>
      <div className="flex w-full justify-between border-b border-(--moss-border-color) px-3 py-[5px]">
        <div className="flex items-center gap-1 overflow-hidden">
          <CheckboxWithLabel
            checked={headerCheckedState}
            onCheckedChange={() => {
              if (allParamsChecked) {
                setParams(params.map((p) => ({ ...p, checked: false })));
              } else {
                setParams(params.map((p) => ({ ...p, checked: true })));
              }
            }}
            label="Query Params"
            className="gap-3 truncate"
          />
          <Counter count={1} color="gray" />
        </div>

        <div className="flex items-center gap-1">
          <ActionButton icon="MoreHorizontal" />
        </div>
      </div>
      <Scrollbar>
        {/* Params */}
        <div className="grid grid-cols-[min-content_minmax(128px,1fr)_minmax(128px,1fr)_min-content_min-content_min-content] gap-2 p-3">
          {params.map((param, index) => {
            const isLastRow = index === params.length - 1;
            return (
              <ParamRow
                key={param.id}
                param={param}
                onChange={handleParamRowChange}
                keyToFocusOnMount={isLastRow ? columnToFocusOnMount : null}
              />
            );
          })}
          <NewParamRowForm onAdd={addNewRowAtTheEnd} />
        </div>
      </Scrollbar>
    </div>
  );
};
