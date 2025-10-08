import { ChangeEvent, useCallback, useContext, useRef, useState } from "react";

import { DropIndicator, InputOutlined } from "@/components";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { EndpointPageContext } from "@/pages/EndpointPage/EndpointPageContext";
import { CheckedState } from "@radix-ui/react-checkbox";
import { QueryParamInfo } from "@repo/moss-project";

import { ParamDragType } from "../constants";
import { useDropTargetNewParamRowForm } from "../hooks/useDropTargetNewParamRowForm";

interface NewParamRowFormProps {
  onAdd: (Param: QueryParamInfo) => void;
  paramType: ParamDragType;
}

export const NewParamRowForm = ({ onAdd, paramType }: NewParamRowFormProps) => {
  const { entry } = useContext(EndpointPageContext);

  const newParamRowFormRef = useRef<HTMLDivElement>(null);

  const [placeholderParam, setPlaceholderParam] = useState<QueryParamInfo>({
    id: "__NewParamRowForm",
    disabled: true,
    name: "",
    value: "",
    propagate: false,
  });

  const debounceTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  const debouncedOnChange = useCallback(
    (updatedParam: QueryParamInfo) => {
      if (debounceTimeoutRef.current) {
        clearTimeout(debounceTimeoutRef.current);
      }

      debounceTimeoutRef.current = setTimeout(() => {
        onAdd(updatedParam);
      }, 500);
    },
    [onAdd]
  );

  const onCheckedChange = (checked: CheckedState) => {
    onAdd({
      ...placeholderParam,
      disabled: checked === "indeterminate" ? false : Boolean(!checked),
    });
  };

  const onKeyChange = (e: ChangeEvent<HTMLInputElement>) => {
    const updatedParam = { ...placeholderParam, name: e.target.value };
    setPlaceholderParam(updatedParam);
    debouncedOnChange(updatedParam);
  };

  const onValueChange = (e: ChangeEvent<HTMLInputElement>) => {
    const updatedParam = { ...placeholderParam, value: e.target.value };
    setPlaceholderParam(updatedParam);
    debouncedOnChange(updatedParam);
  };

  const { closestEdge } = useDropTargetNewParamRowForm({
    newParamRowFormRef,
    entryId: entry.id,
    paramType,
  });

  return (
    <div ref={newParamRowFormRef} className="relative col-span-full grid grid-cols-subgrid items-center">
      {closestEdge && <DropIndicator edge={closestEdge} gap={8} className="-ml-1.5" />}

      <CheckboxWithLabel
        checked={!placeholderParam.disabled}
        onCheckedChange={onCheckedChange}
        className="col-span-1"
      />
      <InputOutlined
        value={placeholderParam.name}
        onChange={onKeyChange}
        placeholder="Key"
        contrast
        className="col-span-1"
      />

      <InputOutlined
        //@ts-expect-error We are not being able to handle anything except string for now
        value={placeholderParam.value}
        onChange={onValueChange}
        placeholder="Value"
        contrast
        className="col-span-1"
      />
    </div>
  );
};
