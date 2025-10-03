import { ChangeEvent, memo, useCallback, useEffect, useRef } from "react";

import { ActionButton, InputOutlined } from "@/components";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { Icon } from "@/lib/ui";
import { CheckedState } from "@radix-ui/react-checkbox";

import { QueryParam } from "../types";

interface QueryParamRowProps {
  param: QueryParam;
  onChange: (updatedParam: QueryParam) => void;
  keyToFocusOnMount?: string | null;
}

export const QueryParamRow = memo(({ param, onChange, keyToFocusOnMount }: QueryParamRowProps) => {
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
    <div key={param.id} className="col-span-full grid grid-cols-subgrid items-center">
      <CheckboxWithLabel checked={param.checked} onCheckedChange={onCheckedChange} />

      <InputOutlined ref={keyRef} value={param.key} onChange={onKeyChange} contrast />
      <InputOutlined ref={valueRef} value={param.value} onChange={onValueChange} contrast />

      <Icon icon="RequiredAsterisk" />
      <TypeBadgePlaceholder type={param.type} />

      <div className="flex items-center gap-1">
        <ActionButton icon="ConfigMap" />
        <ActionButton icon="AddToVcs" />
        <ActionButton icon="RemoveCircle" />
      </div>
    </div>
  );
});

const TypeBadgePlaceholder = ({ type }: { type: string }) => {
  return (
    <div className="background-(--moss-green-9) flex h-min items-center justify-center rounded-full px-1.5 text-[10px] leading-4 text-(--moss-green-1)">
      {type}
    </div>
  );
};
