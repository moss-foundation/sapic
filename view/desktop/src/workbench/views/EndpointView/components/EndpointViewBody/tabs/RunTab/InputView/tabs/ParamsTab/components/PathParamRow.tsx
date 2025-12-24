import { ChangeEvent, memo, useRef } from "react";

import Input from "@/lib/ui/Input";
import { cn } from "@/utils";
import { QueryParamInfo } from "@repo/moss-project";

interface PathParamRowProps {
  param: QueryParamInfo;
  onChange: (updatedParam: QueryParamInfo, originalValue: QueryParamInfo) => void;
}

export const PathParamRow = memo(({ param, onChange }: PathParamRowProps) => {
  const keyRef = useRef<HTMLInputElement>(null);
  const valueRef = useRef<HTMLInputElement>(null);

  const onValueChange = (e: ChangeEvent<HTMLInputElement>) => {
    const updatedParam = { ...param, value: e.target.value };
    onChange(updatedParam, param);
  };
  return (
    <div key={param.id} className={cn("relative col-span-full grid grid-cols-subgrid items-center")}>
      <Input intent="outlined" ref={keyRef} value={param.name} contrast disabled />

      {/* @ts-expect-error  We are not being able to handle anything except string for now */}
      <Input intent="outlined" ref={valueRef} value={param.value} onChange={onValueChange} contrast />
    </div>
  );
});
