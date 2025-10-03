import { ChangeEvent } from "react";

import { InputOutlined } from "@/components";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { CheckedState } from "@radix-ui/react-checkbox";

import { PathParam } from "../types";

interface NewPathParamRowFormProps {
  onAdd: (pathParam: PathParam) => void;
}

export const NewPathParamRowForm = ({ onAdd }: NewPathParamRowFormProps) => {
  const placeholderPathParam = {
    id: "__NewPathParamRowForm",
    checked: false,
    key: "",
    value: "",
  };

  const onCheckedChange = (checked: CheckedState) => {
    onAdd({
      ...placeholderPathParam,
      checked: checked === "indeterminate" ? true : Boolean(checked),
    });
  };

  const onKeyChange = (e: ChangeEvent<HTMLInputElement>) => {
    onAdd({
      ...placeholderPathParam,
      key: e.target.value,
    });
  };

  const onValueChange = (e: ChangeEvent<HTMLInputElement>) => {
    onAdd({
      ...placeholderPathParam,
      value: e.target.value,
    });
  };

  return (
    <div className="col-span-full grid grid-cols-subgrid items-center">
      <CheckboxWithLabel checked={placeholderPathParam.checked} onCheckedChange={onCheckedChange} />
      <InputOutlined value={placeholderPathParam.key} onChange={onKeyChange} placeholder="Key" contrast />
      <InputOutlined value={placeholderPathParam.value} onChange={onValueChange} placeholder="Value" contrast />
    </div>
  );
};
