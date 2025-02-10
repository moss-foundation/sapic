.DEFAULT_GOAL := run-desktop

export LOG_LEVEL = trace


# --- App Directories ---
DESKTOP_DIR := view/desktop

# --- Executables ---
PNPM := pnpm
CARGO := cargo
RUSTUP := rustup

# --- Commands ---

## Run Desktop Application
.PHONY: run-desktop
run-desktop:
	@cd $(DESKTOP_DIR) && $(PNPM) tauri dev