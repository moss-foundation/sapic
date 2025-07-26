import { ParameterData } from "@/components/Table";

// Sample data for Query Params
export const queryParamsData: ParameterData[] = [
  {
    order: 1,
    id: "1",
    key: "pageToken",
    value: "{{mu_func()}}",
    type: "string",
    description: "An opaque token used to fetch the next page of results.",
    global_value: "",
    local_value: 0,
    properties: { disabled: false },
  },
  {
    order: 2,
    id: "2",
    key: "limit",
    value: "{{defaultLimit}}",
    type: "number",
    description: "Maximum number of results to return in this query.",
    global_value: "",
    local_value: 0,
    properties: { disabled: false },
  },
  {
    order: 3,
    id: "3",
    key: "visibleOnly",
    value: "true",
    type: "bool",
    description: "If true, returns only visible columns for the table. This...",
    global_value: "",
    local_value: 0,
    properties: { disabled: true },
  },
];

// Sample data for Path Params
export const pathParamsData: ParameterData[] = [
  {
    order: 1,
    id: "4",
    key: "docId",
    value: "{{vault::myVariable}}",
    type: "string",
    description: "An opaque token used to fetch the next page of results.",
    global_value: "",
    local_value: 0,
    properties: { disabled: false },
  },
  {
    order: 2,
    id: "5",
    key: "tableIdOrName",
    value: "{{defaultLimit}}",
    type: "number",
    description: "Maximum number of results to return in this query.",
    global_value: "",
    local_value: 0,
    properties: { disabled: false },
  },
];
