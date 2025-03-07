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

# --- Executables ---
PNPM := pnpm
CARGO := cargo
RUSTUP := rustup

# --- Commands ---

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
	@$(CARGO) build --manifest-path $($(2))/Cargo.toml
endef

COLLECTION_MODELS_DIR := crates/moss-collection
THEME_MODELS_DIR := crates/moss-theme
STATE_MODELS_DIR := crates/moss-state
NLS_MODELS_DIR := crates/moss-nls

$(eval $(call gen_models,collection,COLLECTION_MODELS_DIR))
$(eval $(call gen_models,theme,THEME_MODELS_DIR))
$(eval $(call gen_models,state,STATE_MODELS_DIR))
$(eval $(call gen_models,nls,NLS_MODELS_DIR))

## Generate All Models
.PHONY: gen-models
gen-models: \
	gen-collection-models \
	gen-theme-models \
	gen-state-models \
	gen-nls-models \

# Utility Commands

## Count Lines of Code
.PHONY: loc
loc:
	@cloc --exclude-dir=$(EXCLUDE_DIRS) --include-ext=$(SRC_EXT) .

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

