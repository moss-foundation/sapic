export WORKSPACE_ROOT_DIR = ${CURDIR}
export LOG_LEVEL = trace

export THEMES_DIR = ${CURDIR}/assets/themes
export LOCALES_DIR =  ${CURDIR}/assets/locales
export APP_LOG_DIR = ${CURDIR}/logs/app
export SESSION_LOG_DIR = ${CURDIR}/logs/session

.DEFAULT_GOAL := run-desktop

# Detect Operating System
ifeq ($(OS),Windows_NT)
    DETECTED_OS := Windows
    HOME_DIR := ${USERPROFILE}
else
    DETECTED_OS := $(shell uname)
    HOME_DIR := ${HOME}
endif


# --- App Directories ---
DESKTOP_DIR := view/desktop
ICONS_DIR := tools/icongen
# --- Tool Directories ---
XTASK_DIR := tools/xtask
TS_IMPORT_INJECTOR := misc/ts_imports_injector.py
TS_EXPORT_INJECTOR := misc/ts_exports_injector.py
# --- Executables ---
PNPM := pnpm
CARGO := cargo
RUSTUP := rustup

ifeq ($(OS),Windows_NT)
    PYTHON := python
else
    PYTHON := python3
endif

# --- Commands ---

.PHONY: ready
ready: gen-icons
	$(PNPM) i


## Generate Icons
.PHONY: gen-icons
gen-icons:
	@cd $(ICONS_DIR) && $(PNPM) build


## Run Desktop Application
.PHONY: run-desktop
run-desktop:
	@cd $(DESKTOP_DIR) && $(PNPM) tauri dev

# --- Models ---

# The gen_models function generates TS models from Rust structures.
# The export_bindings_ prefix is used to run only those tests that trigger
# the generation of models. 
define gen_models
.PHONY: gen-$(1)-model
gen-$(1)-models:
	@$(CARGO) test export_bindings_ --manifest-path $($(2))/Cargo.toml
	@cd $($(2)) && $(PYTHON) ${WORKSPACE_ROOT_DIR}/$(TS_IMPORT_INJECTOR) package.json
	@cd $($(2)) && $(PYTHON) ${WORKSPACE_ROOT_DIR}/$(TS_EXPORT_INJECTOR)
	@cd $($(2)) && $(PNPM) format
endef

COLLECTION_MODELS_DIR := crates/moss-collection
THEME_MODELS_DIR := crates/moss-theme
STATE_MODELS_DIR := crates/moss-state
NLS_MODELS_DIR := crates/moss-nls
LOGGING_MODELS_DIR := crates/moss-logging
ENVIRONMENT_MODELS_DIR := crates/moss-environment
WORKSPACE_MODELS_DIR := crates/moss-workspace

$(eval $(call gen_models,collection,COLLECTION_MODELS_DIR))
$(eval $(call gen_models,theme,THEME_MODELS_DIR))
$(eval $(call gen_models,state,STATE_MODELS_DIR))
$(eval $(call gen_models,nls,NLS_MODELS_DIR))
$(eval $(call gen_models,logging,LOGGING_MODELS_DIR))
$(eval $(call gen_models,environment,ENVIRONMENT_MODELS_DIR))
$(eval $(call gen_models,workspace,WORKSPACE_MODELS_DIR))

## Generate All Models
.PHONY: gen-models
gen-models: \
	gen-collection-models \
	gen-theme-models \
	gen-state-models \
	gen-nls-models \
	gen-logging-models \
	gen-environment-models \
	gen-workspace-models \

# Utility Commands

## Count Lines of Code
.PHONY: loc
loc:
	@cloc --exclude-dir=target,node_modules --include-ext=rs,ts .

## Clean up merged Git branches except master, main, and dev
.PHONY: cleanup-git
cleanup-git:
ifeq ($(DETECTED_OS),Windows)
	@echo TODO: make cleanup-git this work on Windows
# @for /F "tokens=*" %i in ('git branch --merged ^| findstr /V "master main dev"') do git branch -d %i
else
	@git branch --merged | grep -Ev "(^\*|master|main|dev)" | xargs git branch -d
endif

# Clean up unused pnpm packages in all directories and store
# pnpm does not support recursive prune
.PHONY: clean-pnpm
clean-pnpm:
	@echo Cleaning PNPM cache...
	@echo Cleaning Desktop Directory Cache...
	@cd $(DESKTOP_DIR) && $(PNPM) prune
	@echo Cleaning PNPM Store Cache...
	$(PNPM) store prune

# Clean cargo cache
.PHONY: clean-cargo
clean-cargo:
	$(CARGO) clean

# Clean up various artifacts across the project
.PHONY: clean
clean: cleanup-git clean-pnpm clean-cargo


# Generate license with xtask
.PHONY: gen-license
gen-license:
	@echo Generating Workspace Licenses...
	@cd $(XTASK_DIR) && $(CARGO) run license

# Audit workspace dependency
.PHONY: workspace-audit
workspace-audit:
	@echo Checking Non-workspace Dependencies...
	@cd $(XTASK_DIR) && $(CARGO) run rwa

# Check unused dependency
.PHONY: check-unused-deps
check-unused-deps:
	@echo Installing cargo-udeps...
	$(CARGO) --quiet install cargo-udeps --locked
	@echo Installing Nightly Toolchain...
	$(RUSTUP) --quiet toolchain install nightly
	@echo Checking Unused Dependencies...
	$(CARGO) +nightly udeps --quiet

# Runs a series of maintenance tasks to keep the project organized and up-to-date.
# TODO: output workspace-audit and check-unused-deps to file
.PHONY: tidy
tidy: gen-license workspace-audit check-unused-deps
	$(MAKE) clean

# Create a release build
.PHONY: build
build:
	# Enable compression feature for reducing binary size
	$(CARGO) build --bin desktop --features compression

