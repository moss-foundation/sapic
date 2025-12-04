# ======================================================
# Sapic Makefile - Build and Development Configuration
# ======================================================

# ---- Environment Settings ----
export LOG_LEVEL = trace

# ---- Default Goal ----
.DEFAULT_GOAL := run-desktop

# ---- OS Detection ----
ifeq ($(OS),Windows_NT)
    DETECTED_OS := Windows
    HOME_DIR := ${USERPROFILE}
export DEV_USER_DIR = ${USERPROFILE}\.sapic

else
    DETECTED_OS := $(shell uname)
    HOME_DIR := ${HOME}

export DEV_USER_DIR = ${HOME}/.sapic
endif

# ---- Directory Settings ----
export DEV_RESOURCE_DIR = ${CURDIR}

# ---- Asset Directories ----
export ICONS_DIR = ${CURDIR}/assets/icons
export ICONS_OUTPUT_DIR = ${CURDIR}/view/desktop/src/assets/icons

export TYPEDOC_DIR = ${CURDIR}/autodocs

# ---- Directory Paths ----
# Tool directories
GEN_BINDINGS_DIR := tools/gen-bindings
CARGO_NEW_TS := tools/cargo-new-ts
THEME_INSTALL := tools/theme-install

# Application directories
DESKTOP_DIR := view/desktop
XTASK_DIR := tools/xtask
MISC_DIR := misc
SCRIPTS_DIR := scripts

# ---- Crate Directories ----
BASE_DIR := crates/base
IPC_MODELS_DIR := crates/ipc
BINDINGUTILS_DIR := crates/moss-bindingutils
ACTIVITY_BROADCASTER_MODELS_DIR := crates/moss-activity-broadcaster
WINDOW_MODELS_DIR := crates/window
PROJECT_MODELS_DIR := crates/moss-project
ENVIRONMENT_MODELS_DIR := crates/moss-environment
WORKSPACE_MODELS_DIR := crates/moss-workspace
GIT_MODELS_DIR := crates/moss-git
SHARED_STORAGE_MODELS_DIR := plugins/shared_storage
SETTINGS_STORAGE_MODELS_DIR := plugins/settings-storage
URL_PARSER_MODELS_DIR := plugins/url-parser

# ---- Command Executables ----
PNPM := pnpm
CARGO := cargo
RUSTUP := rustup
NPX := npx
UV := uv

# ======================================================
# Run Commands
# ======================================================

## Run the desktop application in development mode
.PHONY: run-desktop
run-desktop:
	@cd $(DESKTOP_DIR) && $(PNPM) tauri dev --features devtools


# ======================================================
# Setup Commands
# ======================================================

.PHONY: pnpm-i
pnpm-i:
	$(PNPM) i

## Install dependencies and setup development environment
.PHONY: ready
ready: pnpm-i gen-icons gen-theme-tokens gen-typedoc
	@cd $(CARGO_NEW_TS) && $(CARGO) install --path .

## Generate TypeDoc documentation
.PHONY: gen-typedoc
gen-typedoc:
	$(PNPM) typedoc

## Icon generator tool
.PHONY: gen-icons
gen-icons:
	@cd $(SCRIPTS_DIR) && $(UV) run svg_component_generator.py plan --source ${ICONS_DIR}
	@cd $(SCRIPTS_DIR) && $(UV) run svg_component_generator.py gen --source ${ICONS_DIR} \
								 --light-json ../extensions/theme-defaults/themes/light-default.json \
								 --dark-json ../extensions/theme-defaults/themes/dark-default.json \
								 --output-dir ${ICONS_OUTPUT_DIR}

## Generate theme tokens
.PHONY: gen-theme-tokens
gen-theme-tokens:
	@cd extensions/theme-defaults && $(PNPM) build
	$(MAKE) export-css-variables

# ======================================================
# TypeScript Bindings Generation
# ======================================================

# The gen_bindings function generates TypeScript models and Zod Schema from Rust structures.
# Zod Schema provides information about function calling arguments for the AI agent
# The export_bindings_ prefix is used to run only those tests that trigger
# the generation of models. 

define gen_bindings
.PHONY: gen-$(1)-bindings
gen-$(1)-bindings:
	@echo "Removing old $(1) models"
	@cd $($(2)) && rm -rf bindings

	@echo "Generating $(1) models..."
	@$(CARGO) test --lib export_bindings_ --manifest-path $($(2))/Cargo.toml
	@cd $(GEN_BINDINGS_DIR) && $(PNPM) run constantsSorter ../../$($(2))
	@cd $(GEN_BINDINGS_DIR) && $(PNPM) run importsResolver ../../$($(2))

	@echo "Generating $(1) zod schemas..."
	@cd $(GEN_BINDINGS_DIR) && $(PNPM) run zodGenerator ../../$($(2))

	@echo "Updating exports in index.ts..."
	@cd $(GEN_BINDINGS_DIR) && $(PNPM) run tsExportsInjector ../../$($(2))

	@echo "Formatting generated files"
	@cd $(GEN_BINDINGS_DIR) && $(PNPM) run importsConsolidator ../../$($(2))
	@cd $($(2)) && $(PNPM) format
	@echo "$(1) bindings generated successfully"
endef

# Apply the gen_bindings function to each crate
$(eval $(call gen_bindings,window,WINDOW_MODELS_DIR))
$(eval $(call gen_bindings,project,PROJECT_MODELS_DIR))
$(eval $(call gen_bindings,environment,ENVIRONMENT_MODELS_DIR))
$(eval $(call gen_bindings,workspace,WORKSPACE_MODELS_DIR))
$(eval $(call gen_bindings,bindingutils,BINDINGUTILS_DIR))
$(eval $(call gen_bindings,ipc,IPC_MODELS_DIR))
$(eval $(call gen_bindings,git,GIT_MODELS_DIR))
$(eval $(call gen_bindings,base,BASE_DIR))
$(eval $(call gen_bindings,activity-broadcaster,ACTIVITY_BROADCASTER_MODELS_DIR))

$(eval $(call gen_bindings,shared-storage,SHARED_STORAGE_MODELS_DIR))
$(eval $(call gen_bindings,settings-storage,SETTINGS_STORAGE_MODELS_DIR))
$(eval $(call gen_bindings,url-parser,URL_PARSER_MODELS_DIR))

gen-window-bindings:
gen-project-bindings:
gen-environment-bindings:
gen-workspace-bindings:
gen-bindingutils-bindings:
gen-ipc-bindings:
gen-git-bindings:
gen-shared-storage-bindings:
gen-settings-storage-bindings:
gen-url-parser-bindings:
gen-base-bindings:
gen-activity-broadcaster-bindings:

## Generate all TypeScript bindings
.PHONY: gen-bindings
gen-bindings: \
	gen-window-bindings \
	gen-project-bindings \
	gen-activity-broadcaster-bindings \
	gen-environment-bindings \
	gen-workspace-bindings \
	gen-bindingutils-bindings \
	gen-ipc-bindings \
	gen-git-bindings \
	gen-shared-storage-bindings \
	gen-settings-storage-bindings \
	gen-base-bindings \


# ======================================================
# Utility Commands
# ======================================================

## Export CSS variables to JSON
.PHONY: export-css-variables
export-css-variables:
	@cd $(SCRIPTS_DIR) && $(UV) run css_variables_exporter.py --source ../extensions/theme-defaults/themes/light-default.json \
														   --dest ../packages/config-eslint/moss-lint-plugin/css_variables.json
	@$(PNPM) prettier --plugin=prettier-plugin-tailwindcss --write packages/config-eslint/moss-lint-plugin/css_variables.json

## Open TypeDoc documentation in browser
.PHONY: open-docs
open-docs:
ifeq ($(DETECTED_OS),Windows)
	@cmd.exe /C start "" "$(TYPEDOC_DIR)\index.html"
else ifeq ($(DETECTED_OS),Darwin)
	@open "$(TYPEDOC_DIR)/index.html"
else
	@xdg-open "$(TYPEDOC_DIR)/index.html"
endif

## Count Lines of Code
.PHONY: loc
loc:
	@cloc --vcs git --include-ext=rs,ts,tsx,py .

# ======================================================
# Cleanup Commands
# ======================================================

## Clean up merged Git branches except master, main, and dev
.PHONY: cleanup-git
cleanup-git:
ifeq ($(DETECTED_OS),Windows)
	@echo "TODO: make cleanup-git work on Windows"
# @for /F "tokens=*" %i in ('git branch --merged ^| findstr /V "master main dev"') do git branch -d %i
else
	@git branch --merged | grep -Ev "(^\*|master|main|dev)" | xargs git branch -d
endif

## Clean up unused pnpm packages in all directories and store
.PHONY: clean-pnpm
clean-pnpm:
	@echo "Cleaning PNPM cache..."
	@echo "Cleaning Desktop Directory Cache..."
	@cd $(DESKTOP_DIR) && $(PNPM) prune
	@echo "Cleaning PNPM Store Cache..."
	$(PNPM) store prune

## Clean cargo cache
.PHONY: clean-cargo
clean-cargo:
	$(CARGO) clean

## Clean up various artifacts across the project
.PHONY: clean
clean: cleanup-git clean-pnpm clean-cargo

# ======================================================
# Maintenance Commands
# ======================================================

## Generate license information
.PHONY: gen-license
gen-license:
	@echo "Generating Workspace Licenses..."
	@cd $(XTASK_DIR) && $(CARGO) run license

## Audit workspace dependencies
.PHONY: rust-audit
rust-audit:
	@echo "Checking Non-workspace Dependencies..."
	@cd $(XTASK_DIR) && $(CARGO) run audit

## Check unused dependencies
.PHONY: check-unused-deps
check-unused-deps:
	@echo "Installing cargo-udeps..."
	$(CARGO) --quiet install cargo-udeps --locked
	@echo "Installing Nightly Toolchain..."
	$(RUSTUP) --quiet toolchain install nightly
	@echo "Checking Unused Dependencies..."
	$(CARGO) +nightly udeps --quiet

## Run a series of maintenance tasks to keep the project organized
.PHONY: tidy
tidy: gen-license rust-audit check-unused-deps
	$(MAKE) clean

# ======================================================
# Build Commands
# ======================================================

## Create a release build
.PHONY: build
build:
	@echo "Building with compression feature for reducing binary size..."
	@cd $(DESKTOP_DIR) && $(PNPM) run tauri build --features compression
