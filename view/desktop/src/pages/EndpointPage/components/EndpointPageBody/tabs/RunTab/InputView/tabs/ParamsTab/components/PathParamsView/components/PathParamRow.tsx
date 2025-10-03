import { ChangeEvent, memo, useCallback, useEffect, useRef } from "react";

import { ActionButton, InputOutlined } from "@/components";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { CheckedState } from "@radix-ui/react-checkbox";

import { PathParam } from "../types";

interface PathParamRowProps {
  param: PathParam;
  onChange: (updatedParam: PathParam) => void;
  keyToFocusOnMount?: string | null;
}

export const PathParamRow = memo(({ param, onChange, keyToFocusOnMount }: PathParamRowProps) => {
  const keyRef = useRef<HTMLInputElement>(null);
  const valueRef = useRef<HTMLInputElement>(null);

  const onCheckedChange = useCallback(
    (checked: CheckedState) => onChange({ ...param, checked: checked === "indeterminate" ? true : Boolean(checked) }),
    [onChange, param]
  );

  const onKeyChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => onChange({ ...param, key: e.target.value }),
    [onChange, param]
  );

  const onValueChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => onChange({ ...param, value: e.target.value }),
    [onChange, param]
  );

  useEffect(() => {
    if (keyToFocusOnMount === "key") {
      keyRef.current?.focus();
    }
    if (keyToFocusOnMount === "value") {
      valueRef.current?.focus();
    }
  }, [keyToFocusOnMount]);

  return (
    <div className="col-span-full grid grid-cols-subgrid items-center">
      <CheckboxWithLabel checked={param.checked} onCheckedChange={onCheckedChange} />
      <InputOutlined ref={keyRef} value={param.key} onChange={onKeyChange} contrast />
      <InputOutlined ref={valueRef} value={param.value} onChange={onValueChange} contrast />
      <div className="flex items-center gap-1">
        <ActionButton icon="ConfigMap" />
        <ActionButton icon="AddToVcs" />
        <ActionButton icon="RemoveCircle" />
      </div>
    </div>
  );
});
