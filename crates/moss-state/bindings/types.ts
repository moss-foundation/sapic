// This file was generated by misc/importer.py. Do not edit this file manually.
//
// The necessary import statements have been automatically added by a Python script.
// This ensures that all required dependencies are correctly referenced and available
// within this module.
//
// If you need to add or modify imports, please update the imports.json and
// re-run `make gen-models` it to regenerate the file accordingly.

import type { ThemeDescriptor } from "@repo/moss-theme";
import type { LocaleDescriptor } from "@repo/moss-nls";

export type Defaults = { theme: ThemeDescriptor; locale: LocaleDescriptor };

export type Preferences = { theme?: ThemeDescriptor; locale?: LocaleDescriptor };
