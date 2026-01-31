import { ChangeEvent, useContext, useRef, useState } from "react";

import { useGetLocalResourceDetails } from "@/db/resourceDetails/hooks/useGetLocalResourceDetails";
import CheckboxWithLabel from "@/lib/ui/CheckboxWithLabel";
import Input from "@/lib/ui/Input";
import { DropIndicator } from "@/workbench/ui/components";
import { EndpointViewContext } from "@/workbench/views/EndpointView/EndpointViewContext";
import { CheckedState } from "@radix-ui/react-checkbox";
import { QueryParamInfo } from "@repo/moss-project";

import { ParamDragType } from "../constants";
import { useDropTargetNewParamRowForm } from "../hooks/useDropTargetNewParamRowForm";

interface NewParamRowFormProps {
  onAdd: (Param: QueryParamInfo) => void;
  paramType: ParamDragType;
}

export const NewParamRowForm = ({ onAdd, paramType }: NewParamRowFormProps) => {
  const { resourceId } = useContext(EndpointViewContext);

  const localResourceDetails = useGetLocalResourceDetails(resourceId);

  const newParamRowFormRef = useRef<HTMLDivElement>(null);

  const [placeholderParam, setPlaceholderParam] = useState<QueryParamInfo>({
    id: "__NewParamRowForm",
    disabled: false,
    name: "",
    value: "",
    description: "",
    propagate: false,
  });

  const onCheckedChange = (checked: CheckedState) => {
    onAdd({
      ...placeholderParam,
      disabled: checked === "indeterminate" ? false : Boolean(!checked),
    });
  };

  const onKeyChange = (e: ChangeEvent<HTMLInputElement>) => {
    const updatedParam = { ...placeholderParam, name: e.target.value };
    setPlaceholderParam(updatedParam);
    onAdd(updatedParam);
  };

  const onValueChange = (e: ChangeEvent<HTMLInputElement>) => {
    const updatedParam = { ...placeholderParam, value: e.target.value };
    setPlaceholderParam(updatedParam);
    onAdd(updatedParam);
  };

  const { closestEdge } = useDropTargetNewParamRowForm({
    newParamRowFormRef,
    resourceId: localResourceDetails?.id ?? "Unknown Resource ID",
    paramType,
  });

  return (
    <div ref={newParamRowFormRef} className="relative col-span-full grid grid-cols-subgrid items-center">
      {closestEdge && <DropIndicator edge={closestEdge} gap={8} className="-ml-1.5" />}

      <CheckboxWithLabel checked={false} onCheckedChange={onCheckedChange} className="col-span-1" />
      <Input
        intent="outlined"
        value={placeholderParam.name}
        onChange={onKeyChange}
        placeholder="Key"
        contrast
        className="col-span-1"
      />
      <Input
        intent="outlined"
        // @ts-expect-error We are not being able to handle anything except string for now
        value={placeholderParam.value}
        onChange={onValueChange}
        placeholder="Value"
        contrast
        className="col-span-1"
      />
    </div>
  );
};
