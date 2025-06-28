// world plugin:js-demo/js-demo
export type Value = import("./interfaces/plugin-base-types.js").Value;
export type * as PluginBaseHostFunctions from "./interfaces/plugin-base-host-functions.js"; // import plugin:base/host-functions
export type * as PluginBaseTypes from "./interfaces/plugin-base-types.js"; // import plugin:base/types
export function execute(input: Value): Value;
