import { EndpointViewContextProps } from "@/workbench/views/EndpointView/EndpointViewContext";

interface VariableReactTooltipProps {
  variableName: string;
  ctx: EndpointViewContextProps;
}

export const VariableReactTooltip = ({ variableName, ctx }: VariableReactTooltipProps) => {
  return (
    <div className="rounded-md bg-white p-2 shadow-md">
      <div className="mb-2 text-sm">
        <span className="text-gray-500">Resource Id:</span>
        <span className="font-bold text-sky-600">{ctx.resourceId}</span>
      </div>

      <div className="text-sm text-gray-500">
        <span className="text-gray-500">Variable Name:</span>
        <span className="font-bold text-gray-800">{variableName}</span>
      </div>
    </div>
  );
};
