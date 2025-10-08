import { ChangeEvent, useCallback, useRef, useState } from "react";

import { InputOutlined } from "@/components";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { CheckedState } from "@radix-ui/react-checkbox";
import { QueryParamInfo } from "@repo/moss-project";

interface NewParamRowFormProps {
  onAdd: (Param: QueryParamInfo) => void;
}

export const NewParamRowForm = ({ onAdd }: NewParamRowFormProps) => {
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

  return (
    <div className="col-span-full grid grid-cols-subgrid items-center">
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
