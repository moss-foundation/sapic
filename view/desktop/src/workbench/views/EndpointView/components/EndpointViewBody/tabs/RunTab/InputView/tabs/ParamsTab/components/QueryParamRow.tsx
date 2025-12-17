import { ChangeEvent, memo, useContext, useEffect, useRef } from "react";

import { resourcesDescriptionsCollection } from "@/app/resourcesDescriptionsCollection";
import { useHoverDelay } from "@/hooks";
import { Icon } from "@/lib/ui";
import CheckboxWithLabel from "@/lib/ui/CheckboxWithLabel";
import Input from "@/lib/ui/Input";
import { cn } from "@/utils";
import { ActionButton, DropIndicator } from "@/workbench/ui/components";
import { DragHandleButton } from "@/workbench/ui/components/DragHandleButton";
import { EndpointViewContext } from "@/workbench/views/EndpointView/EndpointViewContext";
import { CheckedState } from "@radix-ui/react-checkbox";
import { QueryParamInfo } from "@repo/moss-project";
import { eq, useLiveQuery } from "@tanstack/react-db";

import { useDraggableParamRow } from "../hooks/useDraggableParamRow";
import { ParamDragType } from "../types";

interface ParamRowProps {
  param: QueryParamInfo;
  onChange: (updatedParam: QueryParamInfo, originalParam: QueryParamInfo) => void;
  onDelete: () => void;
  keyToFocusOnMount?: string | null;
  setColumnToFocusOnMount?: (column: string | null) => void;
  paramType: ParamDragType;
}

export const QueryParamRow = memo(
  ({ param, onChange, keyToFocusOnMount, onDelete, paramType, setColumnToFocusOnMount }: ParamRowProps) => {
    const { resourceId } = useContext(EndpointViewContext);

    const { data: localResourceDescription } = useLiveQuery((q) =>
      q
        .from({ collection: resourcesDescriptionsCollection })
        .where(({ collection }) => eq(collection.id, resourceId))
        .findOne()
    );

    const keyRef = useRef<HTMLInputElement>(null);
    const valueRef = useRef<HTMLInputElement>(null);

    const { isHovered, handleMouseEnter, handleMouseLeave, resetHover } = useHoverDelay();

    const { isDragging, dragHandleRef, paramRowRef, closestEdge } = useDraggableParamRow({
      param,
      resourceId: localResourceDescription?.id ?? "Unknown Resource ID",
      paramType,
    });

    useEffect(() => {
      if (keyToFocusOnMount === "key") keyRef.current?.focus();
      if (keyToFocusOnMount === "value") valueRef.current?.focus();
      setColumnToFocusOnMount?.(null);
      // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    const onCheckedChange = (checked: CheckedState) => {
      const updatedParam = { ...param, disabled: checked === "indeterminate" ? false : Boolean(!checked) };
      onChange(updatedParam, param);
      resetHover();
    };

    const onKeyChange = (e: ChangeEvent<HTMLInputElement>) => {
      const updatedParam = { ...param, name: e.target.value };
      onChange(updatedParam, param);
    };

    const onValueChange = (e: ChangeEvent<HTMLInputElement>) => {
      const updatedParam = { ...param, value: e.target.value };
      onChange(updatedParam, param);
    };

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
            className={cn(
              "rounded-xs absolute left-0 top-1/2 -translate-y-1/2 shadow-none transition-opacity duration-200",
              {
                "pointer-events-auto opacity-100": isHovered,
                "pointer-events-none opacity-0": !isHovered,
              }
            )}
          />
        </div>

        <Input intent="outlined" ref={keyRef} value={param.name} onChange={onKeyChange} contrast />

        {/* @ts-expect-error  We are not being able to handle anything except string for now */}
        <Input intent="outlined" ref={valueRef} value={param.value} onChange={onValueChange} contrast />

        <Icon icon="RequiredAsterisk" />

        <TypeBadgePlaceholder type="string" />

        <div className="flex items-center gap-1">
          <ActionButton icon="ConfigMap" />
          <ActionButton icon="AddToVcs" />
          <ActionButton icon="RemoveCircle" onClick={onDelete} />
        </div>
      </div>
    );
  }
);

const TypeBadgePlaceholder = ({ type }: { type: string }) => {
  return (
    <div className="background-(--moss-green-9) text-(--moss-green-1) flex items-center justify-center rounded-full px-1.5 text-[10px] leading-[15px]">
      {type}
    </div>
  );
};
