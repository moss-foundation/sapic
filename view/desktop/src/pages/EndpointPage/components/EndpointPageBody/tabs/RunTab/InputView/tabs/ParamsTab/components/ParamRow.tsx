import { ChangeEvent, memo, useCallback, useContext, useEffect, useRef, useState } from "react";

import { ActionButton, DropIndicator, InputOutlined } from "@/components";
import CheckboxWithLabel from "@/components/CheckboxWithLabel";
import { DragHandleButton } from "@/components/DragHandleButton";
import { useHoverDelay } from "@/hooks";
import { Icon } from "@/lib/ui";
import { EndpointPageContext } from "@/pages/EndpointPage/EndpointPageContext";
import { cn } from "@/utils";
import { CheckedState } from "@radix-ui/react-checkbox";
import { QueryParamInfo } from "@repo/moss-project";

import { useDraggableParamRow } from "../hooks/useDraggableParamRow";
import { ParamDragType } from "../types";

interface ParamRowProps {
  param: QueryParamInfo;
  onChange: (updatedParam: QueryParamInfo) => void;
  onDelete: () => void;
  keyToFocusOnMount?: string | null;
  paramType: ParamDragType;
}

export const ParamRow = memo(
  ({ param: initialParam, onChange, keyToFocusOnMount, onDelete, paramType }: ParamRowProps) => {
    const { entry } = useContext(EndpointPageContext);

    const keyRef = useRef<HTMLInputElement>(null);
    const valueRef = useRef<HTMLInputElement>(null);
    const debounceTimeoutRef = useRef<NodeJS.Timeout | null>(null);

    const [param, setParam] = useState(initialParam);

    const { isHovered, handleMouseEnter, handleMouseLeave } = useHoverDelay();

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

    const { isDragging, dragHandleRef, paramRowRef, closestEdge } = useDraggableParamRow({
      param,
      entryId: entry.id,
      paramType,
    });

    return (
      <div
        key={param.id}
        className={cn("relative col-span-full grid grid-cols-subgrid items-center", {
          "opacity-50": isDragging,
        })}
        ref={paramRowRef}
      >
        {closestEdge && <DropIndicator edge={closestEdge} gap={8} className="-ml-1.5" />}

        <div
          className="group/paramRow relative flex items-center gap-1"
          onMouseEnter={handleMouseEnter}
          onMouseLeave={handleMouseLeave}
        >
          <CheckboxWithLabel checked={!param.disabled} onCheckedChange={onCheckedChange} />
          <DragHandleButton
            ref={dragHandleRef}
            className={cn("absolute top-1/2 left-0 -translate-y-1/2 rounded-xs transition-opacity duration-200", {
              "pointer-events-auto opacity-100": isHovered,
              "pointer-events-none opacity-0": !isHovered,
            })}
          />
        </div>

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
  }
);

const TypeBadgePlaceholder = ({ type }: { type: string }) => {
  return (
    <div className="background-(--moss-green-9) flex items-center justify-center rounded-full px-1.5 text-[10px] leading-[15px] text-(--moss-green-1)">
      {type}
    </div>
  );
};
