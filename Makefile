# ======================================================
# Sapic Makefile - Build and Development Configuration
# ======================================================

# ---- Environment Settings ----
export LOG_LEVEL = trace

# ---- Asset Directories ----
export THEMES_DIR = ${CURDIR}/assets/themes
export LOCALES_DIR = ${CURDIR}/assets/locales
export APP_LOG_DIR = ${CURDIR}/logs/app
export SESSION_LOG_DIR = ${CURDIR}/logs/session

# ---- Default Goal ----
.DEFAULT_GOAL := run-desktop

# ---- OS Detection ----
ifeq ($(OS),Windows_NT)
    DETECTED_OS := Windows
    HOME_DIR := ${USERPROFILE}
    ifeq ($(shell where py 2>NUL),)
        PYTHON := python
    else
        PYTHON := py
    endif
    PIP := pip
export DEV_APP_DIR = ${USERPROFILE}\.sapic
else
    DETECTED_OS := $(shell uname)
    HOME_DIR := ${HOME}
    PYTHON := python3
    PIP := pip3

export DEV_APP_DIR = ${HOME}/.sapic
endif

# ---- Environment Settings ----
export LOG_LEVEL = trace
export DEV_APP_DIR = ${HOME_DIR}/.sapic

# ---- Asset Directories ----
export THEMES_DIR = ${CURDIR}/assets/themes
export LOCALES_DIR = ${CURDIR}/assets/locales
export ICONS_DIR = ${CURDIR}/assets/icons
export ICONS_OUTPUT_DIR = ${CURDIR}/view/desktop/src/assets/icons
export APP_LOG_DIR = ${CURDIR}/logs/app
export SESSION_LOG_DIR = ${CURDIR}/logs/session

# ---- Default Goal ----
.DEFAULT_GOAL := run-desktop

# ---- Directory Paths ----
# Tool directories
GEN_BINDINGS_DIR := tools/gen-bindings

# Application directories
DESKTOP_DIR := view/desktop
XTASK_DIR := tools/xtask
MISC_DIR := misc

# ---- Crate Directories ----
COLLECTION_MODELS_DIR := crates/moss-collection
THEME_MODELS_DIR := crates/moss-theme
STATE_MODELS_DIR := crates/moss-state
NLS_MODELS_DIR := crates/moss-nls
LOGGING_MODELS_DIR := crates/moss-logging
ENVIRONMENT_MODELS_DIR := crates/moss-environment
WORKSPACE_MODELS_DIR := crates/moss-workspace
COMMON_MODELS_DIR := crates/moss-common
WORKBENCH_MODELS_DIR := crates/moss-workbench
ACTIVITY_INDICATOR_MODELS_DIR := crates/moss-activity-indicator

# ---- Command Executables ----
PNPM := pnpm
CARGO := cargo
RUSTUP := rustup
NPX := npx

# ======================================================
# Run Commands
# ======================================================

## Run the desktop application in development mode
.PHONY: run-desktop
run-desktop:
	@cd $(DESKTOP_DIR) && $(PNPM) tauri dev


# ======================================================
# Setup Commands
# ======================================================

## Install dependencies and setup development environment
.PHONY: ready
ready: gen-icons
	$(PNPM) i
	@cd $(MISC_DIR) && $(PIP) install --break-system-packages -r requirements.txt

## Icon generator tool
.PHONY: gen-icons
gen-icons:
	@cd $(MISC_DIR) && $(PYTHON) svg_component_generator.py plan --source ${ICONS_DIR}
	@cd $(MISC_DIR) && $(PYTHON) svg_component_generator.py gen --source ${ICONS_DIR} \
								 --light-css ../assets/themes/light.css \
								 --dark-css ../assets/themes/dark.css \
								 --output-dir ${ICONS_OUTPUT_DIR}

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
	@$(CARGO) test export_bindings_ --manifest-path $($(2))/Cargo.toml
	@cd $(GEN_BINDINGS_DIR) && $(PNPM) run importsResolver ../../$($(2))

	@echo "Generating $(1) zod schemas..."
	@cd $(GEN_BINDINGS_DIR) && $(PNPM) run zodGenerator ../../$($(2))

	@echo "Updating exports in index.ts..."
	@cd $($(2)) && $(PYTHON) ${CURDIR}/$(MISC_DIR)/ts_exports_injector.py

	@echo "Formatting generated files"
	@cd $(GEN_BINDINGS_DIR) && $(PNPM) run importsConsolidator ../../$($(2))
	@cd $($(2)) && $(PNPM) format
	@echo "$(1) bindings generated successfully"
endef

# Apply the gen_bindings function to each crate
$(eval $(call gen_bindings,collection,COLLECTION_MODELS_DIR))
$(eval $(call gen_bindings,theme,THEME_MODELS_DIR))
$(eval $(call gen_bindings,state,STATE_MODELS_DIR))
$(eval $(call gen_bindings,nls,NLS_MODELS_DIR))
$(eval $(call gen_bindings,logging,LOGGING_MODELS_DIR))
$(eval $(call gen_bindings,environment,ENVIRONMENT_MODELS_DIR))
$(eval $(call gen_bindings,workspace,WORKSPACE_MODELS_DIR))
$(eval $(call gen_bindings,common,COMMON_MODELS_DIR))
$(eval $(call gen_bindings,workbench,WORKBENCH_MODELS_DIR))
$(eval $(call gen_bindings,activity-indicator,ACTIVITY_INDICATOR_MODELS_DIR))

## Generate all TypeScript bindings
.PHONY: gen-bindings
gen-bindings: \
	gen-collection-bindings \
	gen-theme-bindings \
	gen-state-bindings \
	gen-nls-bindings \
	gen-logging-bindings \
	gen-environment-bindings \
	gen-workspace-bindings \
	gen-common-bindings \
	gen-workbench-bindings \
	gen-activity-indicator-bindings

# ======================================================
# Utility Commands
# ======================================================

## Export CSS variables to JSON
.PHONY: export-css-variables
export-css-variables:
	@cd $(MISC_DIR) && $(PYTHON) css_variables_exporter.py

## Count Lines of Code
.PHONY: loc
loc:
	@cloc --exclude-dir=target,node_modules --include-ext=rs,ts,tsx .

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
.PHONY: workspace-audit
workspace-audit:
	@echo "Checking Non-workspace Dependencies..."
	@cd $(XTASK_DIR) && $(CARGO) run rwa

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
tidy: gen-license workspace-audit check-unused-deps
	$(MAKE) clean

# ======================================================
# Build Commands
# ======================================================

## Create a release build
.PHONY: build
build:
	# Enable compression feature for reducing binary size
	$(CARGO) build --bin desktop --features compression
