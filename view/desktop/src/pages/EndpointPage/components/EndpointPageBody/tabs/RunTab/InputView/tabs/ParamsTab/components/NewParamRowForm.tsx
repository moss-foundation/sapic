import { ChangeEvent } from "react";

import { InputOutlined } from "@/components";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { CheckedState } from "@radix-ui/react-checkbox";

import { ParamProps } from "./types";

interface NewParamRowFormProps {
  onAdd: (Param: ParamProps) => void;
}

export const NewParamRowForm = ({ onAdd }: NewParamRowFormProps) => {
  const placeholderParam: ParamProps = {
    id: "__NewParamRowForm",
    checked: false,
    key: "",
    value: "",
    isRequired: false,
    type: "string",
  };

  const onCheckedChange = (checked: CheckedState) => {
    onAdd({
      ...placeholderParam,
      checked: checked === "indeterminate" ? true : Boolean(checked),
    });
  };

  const onKeyChange = (e: ChangeEvent<HTMLInputElement>) => {
    onAdd({
      ...placeholderParam,
      key: e.target.value,
    });
  };

  const onValueChange = (e: ChangeEvent<HTMLInputElement>) => {
    onAdd({
      ...placeholderParam,
      value: e.target.value,
    });
  };

  return (
    <div className="col-span-full grid grid-cols-subgrid items-center">
      <CheckboxWithLabel checked={placeholderParam.checked} onCheckedChange={onCheckedChange} className="col-span-1" />
      <InputOutlined
        value={placeholderParam.key}
        onChange={onKeyChange}
        placeholder="Key"
        contrast
        className="col-span-1"
      />
      <InputOutlined
        value={placeholderParam.value}
        onChange={onValueChange}
        placeholder="Value"
        contrast
        className="col-span-1"
      />
    </div>
  );
};
