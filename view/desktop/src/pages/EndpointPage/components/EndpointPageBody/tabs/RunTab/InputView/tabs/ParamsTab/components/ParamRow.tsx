import { ChangeEvent, memo, useCallback, useEffect, useRef, useState } from "react";

import { ActionButton, InputOutlined } from "@/components";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { Icon } from "@/lib/ui";
import { CheckedState } from "@radix-ui/react-checkbox";
import { QueryParamInfo } from "@repo/moss-project";

interface ParamRowProps {
  param: QueryParamInfo;
  onChange: (updatedParam: QueryParamInfo) => void;
  onDelete: () => void;
  keyToFocusOnMount?: string | null;
}

export const ParamRow = memo(({ param: initialParam, onChange, keyToFocusOnMount, onDelete }: ParamRowProps) => {
  const keyRef = useRef<HTMLInputElement>(null);
  const valueRef = useRef<HTMLInputElement>(null);
  const debounceTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  const [param, setParam] = useState(initialParam);

  useEffect(() => {
    setParam(initialParam);
  }, [initialParam]);

  useEffect(() => {
    if (keyToFocusOnMount === "key") {
      keyRef.current?.focus();
    }
    if (keyToFocusOnMount === "value") {
      valueRef.current?.focus();
    }
  }, []);

  const debouncedOnChange = useCallback(
    (updatedParam: QueryParamInfo) => {
      if (debounceTimeoutRef.current) {
        clearTimeout(debounceTimeoutRef.current);
      }

      debounceTimeoutRef.current = setTimeout(() => {
        onChange(updatedParam);
      }, 500);
    },
    [onChange]
  );

  useEffect(() => {
    return () => {
      if (debounceTimeoutRef.current) {
        clearTimeout(debounceTimeoutRef.current);
      }
    };
  }, []);

  const onCheckedChange = useCallback(
    (checked: CheckedState) => {
      const updatedParam = { ...param, disabled: checked === "indeterminate" ? false : Boolean(!checked) };
      setParam(updatedParam);
      onChange(updatedParam);
    },
    [onChange, param]
  );

  const onKeyChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => {
      const updatedParam = { ...param, name: e.target.value };
      setParam(updatedParam);
      debouncedOnChange(updatedParam);
    },
    [debouncedOnChange, param]
  );

  const onValueChange = useCallback(
    (e: ChangeEvent<HTMLInputElement>) => {
      const updatedParam = { ...param, value: e.target.value };
      setParam(updatedParam);
      debouncedOnChange(updatedParam);
    },
    [debouncedOnChange, param]
  );

  return (
    <div key={param.id} className="col-span-full grid grid-cols-subgrid items-center">
      <CheckboxWithLabel checked={!param.disabled} onCheckedChange={onCheckedChange} />

      <InputOutlined ref={keyRef} value={param.name} onChange={onKeyChange} contrast />

      {/* @ts-expect-error  We are not being able to handle anything except string for now */}
      <InputOutlined ref={valueRef} value={param.value} onChange={onValueChange} contrast />

      <Icon icon="RequiredAsterisk" />
      <TypeBadgePlaceholder type="string" />

      <div className="flex items-center gap-1">
        <ActionButton icon="ConfigMap" />
        <ActionButton icon="AddToVcs" />
        <ActionButton icon="RemoveCircle" onClick={onDelete} />
        <div className="underline">{param.order}</div>
      </div>
    </div>
  );
});

const TypeBadgePlaceholder = ({ type }: { type: string }) => {
  return (
    <div className="background-(--moss-green-9) flex items-center justify-center rounded-full px-1.5 text-[10px] leading-[15px] text-(--moss-green-1)">
      {type}
    </div>
  );
};
