# Makefile for Collaborative Intelligence

.PHONY: install install-user install-system uninstall help install-memory

# Installation directories
USER_BIN := $(HOME)/.local/bin
SYSTEM_BIN := /usr/local/bin
INSTALL_DIR ?= $(USER_BIN)

help:
	@echo "Collaborative Intelligence command-line tool"
	@echo ""
	@echo "Targets:"
	@echo "  make install        - Install memory architecture for all agents"
	@echo "  make install-memory - Install memory architecture for all agents (explicit)"
	@echo "  make install-cli    - Install CI command-line tool to default location ($(INSTALL_DIR))"
	@echo "  make install-user   - Install CI tool to user bin ($(USER_BIN))"
	@echo "  make install-system - Install CI tool to system bin ($(SYSTEM_BIN), requires sudo)"
	@echo "  make uninstall      - Remove the CI command-line tool"
	@echo ""
	@echo "Options:"
	@echo "  INSTALL_DIR=PATH    - Custom installation directory (for CLI installation)"

install:
	@$(MAKE) install-memory

install-memory:
	@echo "Installing Collaborative Intelligence Memory Architecture..."
	@echo ""
	@echo "Setting up memory system for agents..."
	@for agent in $(CURDIR)/AGENTS/*/; do \
		if [ -d "$$agent" ]; then \
			agent_name=$$(basename $$agent); \
			echo "Processing agent: $$agent_name"; \
			if [ ! -f "$$agent/MEMORY.md" ]; then \
				echo "  Creating MEMORY.md for $$agent_name"; \
				cp $(CURDIR)/AGENTS/Manager/templates/MEMORY.md.template $$agent/MEMORY.md 2>/dev/null || \
				echo "  Warning: Template not found, skipping MEMORY.md creation"; \
			fi; \
			if [ ! -f "$$agent/ContinuousLearning.md" ]; then \
				echo "  Creating ContinuousLearning.md for $$agent_name"; \
				cp $(CURDIR)/AGENTS/Manager/templates/ContinuousLearning.md.template $$agent/ContinuousLearning.md 2>/dev/null || \
				echo "  Warning: Template not found, skipping ContinuousLearning.md creation"; \
			fi; \
			if [ ! -d "$$agent/Sessions" ]; then \
				echo "  Creating Sessions directory for $$agent_name"; \
				mkdir -p $$agent/Sessions; \
				echo "# $$agent_name Sessions" > $$agent/Sessions/README.md; \
			fi; \
		fi; \
	done
	@echo ""
	@echo "Memory architecture installation complete!"
	@echo "All agents now have:"
	@echo "  - MEMORY.md (if not already present)"
	@echo "  - ContinuousLearning.md (if not already present)"
	@echo "  - Sessions/ directory structure"
	@echo ""
	@echo "To install the CLI tool, use: make install-cli"

install-cli:
	@echo "Installing Collaborative Intelligence CLI tool..."
	@mkdir -p $(INSTALL_DIR)
	@chmod +x $(CURDIR)/ci-tools/CI
	@echo '#!/bin/bash' > $(INSTALL_DIR)/CI
	@echo '# Collaborative Intelligence tool wrapper' >> $(INSTALL_DIR)/CI
	@echo 'export CI_REPO_PATH="$(CURDIR)"' >> $(INSTALL_DIR)/CI
	@echo 'exec "$(CURDIR)/ci-tools/CI" "$$@"' >> $(INSTALL_DIR)/CI
	@chmod +x $(INSTALL_DIR)/CI
	@echo "Installation complete. CI command installed to $(INSTALL_DIR)/CI"
	@if ! echo $$PATH | grep -q "$(INSTALL_DIR)"; then \
		echo "Warning: $(INSTALL_DIR) is not in your PATH"; \
		echo "Add it to your shell configuration:"; \
		echo "  export PATH=\"\$$PATH:$(INSTALL_DIR)\""; \
		echo ""; \
		echo "Or use 'make install-system' to install to $(SYSTEM_BIN)"; \
	fi
	@echo ""
	@echo "Usage: CI init my-project"
	@echo "See 'CI --help' for more information"

# Install to user bin directory
install-user:
	@$(MAKE) INSTALL_DIR=$(USER_BIN) install-cli

# Install to system bin directory (requires sudo)
install-system:
	@echo "Installing to system directory ($(SYSTEM_BIN))"
	@echo "This requires sudo access"
	@sudo mkdir -p $(SYSTEM_BIN)
	@sudo chmod +x $(CURDIR)/ci-tools/CI
	@sudo bash -c 'echo "#!/bin/bash" > $(SYSTEM_BIN)/CI'
	@sudo bash -c 'echo "# Collaborative Intelligence tool wrapper" >> $(SYSTEM_BIN)/CI'
	@sudo bash -c 'echo "export CI_REPO_PATH=\"$(CURDIR)\"" >> $(SYSTEM_BIN)/CI'
	@sudo bash -c 'echo "exec \"$(CURDIR)/ci-tools/CI\" \"\$$@\"" >> $(SYSTEM_BIN)/CI'
	@sudo chmod +x $(SYSTEM_BIN)/CI
	@echo "Installation complete. CI command installed to $(SYSTEM_BIN)/CI"
	@echo ""
	@echo "Usage: CI init my-project"
	@echo "See 'CI --help' for more information"

# Create a symlink in the current directory
install-local:
	@echo "Creating local symlink for CI command..."
	@chmod +x $(CURDIR)/ci-tools/CI
	@echo '#!/bin/bash' > $(CURDIR)/CI
	@echo '# Collaborative Intelligence tool wrapper' >> $(CURDIR)/CI
	@echo 'export CI_REPO_PATH="$(CURDIR)"' >> $(CURDIR)/CI 
	@echo 'exec "$(CURDIR)/ci-tools/CI" "$$@"' >> $(CURDIR)/CI
	@chmod +x $(CURDIR)/CI
	@echo "Created local CI command: $(CURDIR)/CI"
	@echo ""
	@echo "Usage: ./CI init my-project"
	@echo "See './CI --help' for more information"

uninstall:
	@echo "Uninstalling Collaborative Intelligence CLI tool..."
	@if [ -f "$(USER_BIN)/CI" ]; then \
		rm -f "$(USER_BIN)/CI"; \
		echo "Removed from $(USER_BIN)/CI"; \
	fi
	@if [ -f "$(SYSTEM_BIN)/CI" ]; then \
		sudo rm -f "$(SYSTEM_BIN)/CI"; \
		echo "Removed from $(SYSTEM_BIN)/CI"; \
	fi
	@if [ -f "$(CURDIR)/CI" ]; then \
		rm -f "$(CURDIR)/CI"; \
		echo "Removed local command $(CURDIR)/CI"; \
	fi
	@echo "Uninstallation complete"