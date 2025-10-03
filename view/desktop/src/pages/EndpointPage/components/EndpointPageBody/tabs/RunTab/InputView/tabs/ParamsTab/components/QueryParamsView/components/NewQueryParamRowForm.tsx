import { ChangeEvent } from "react";

import { InputOutlined } from "@/components";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { CheckedState } from "@radix-ui/react-checkbox";

import { QueryParam } from "../types";

interface NewQueryParamRowFormProps {
  onAdd: (queryParam: QueryParam) => void;
}

export const NewQueryParamRowForm = ({ onAdd }: NewQueryParamRowFormProps) => {
  const placeholderQueryParam = {
    id: "__NewQueryParamRowForm",
    checked: false,
    key: "",
    value: "",
    isRequired: false,
    type: "string",
  };

  const onCheckedChange = (checked: CheckedState) => {
    onAdd({
      ...placeholderQueryParam,
      checked: checked === "indeterminate" ? true : Boolean(checked),
    });
  };

  const onKeyChange = (e: ChangeEvent<HTMLInputElement>) => {
    onAdd({
      ...placeholderQueryParam,
      key: e.target.value,
    });
  };

  const onValueChange = (e: ChangeEvent<HTMLInputElement>) => {
    onAdd({
      ...placeholderQueryParam,
      value: e.target.value,
    });
  };

  return (
    <div className="col-span-full grid grid-cols-subgrid items-center">
      <CheckboxWithLabel checked={placeholderQueryParam.checked} onCheckedChange={onCheckedChange} />
      <InputOutlined value={placeholderQueryParam.key} onChange={onKeyChange} placeholder="Key" contrast />
      <InputOutlined value={placeholderQueryParam.value} onChange={onValueChange} placeholder="Value" contrast />
    </div>
  );
};
