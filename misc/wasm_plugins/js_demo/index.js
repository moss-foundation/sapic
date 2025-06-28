import { greet } from "plugin:base/host-functions";

export const execute = (value) => {
  greet(value);
  return {
    tag: "str",
    val: "Success",
  };
};
