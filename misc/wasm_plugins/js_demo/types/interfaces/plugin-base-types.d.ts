/** @module Interface plugin:base/types **/
export type Number = NumberSigned | NumberUnsigned | NumberFloat;
export interface NumberSigned {
  tag: "signed";
  val: bigint;
}
export interface NumberUnsigned {
  tag: "unsigned";
  val: bigint;
}
export interface NumberFloat {
  tag: "float";
  val: number;
}
export type SimpleValue = SimpleValueNull | SimpleValueBoolean | SimpleValueNum | SimpleValueStr;
export interface SimpleValueNull {
  tag: "null";
}
export interface SimpleValueBoolean {
  tag: "boolean";
  val: boolean;
}
export interface SimpleValueNum {
  tag: "num";
  val: Number;
}
export interface SimpleValueStr {
  tag: "str";
  val: string;
}
export type Value = ValueNull | ValueBoolean | ValueNum | ValueStr | ValueArr | ValueObj;
export interface ValueNull {
  tag: "null";
}
export interface ValueBoolean {
  tag: "boolean";
  val: boolean;
}
export interface ValueNum {
  tag: "num";
  val: Number;
}
export interface ValueStr {
  tag: "str";
  val: string;
}
export interface ValueArr {
  tag: "arr";
  val: Array<SimpleValue>;
}
export interface ValueObj {
  tag: "obj";
  val: Array<[string, SimpleValue]>;
}
