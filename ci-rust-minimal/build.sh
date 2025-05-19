#!/usr/bin/env bash
# Quick build script for minimal CI Rust CLI

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Building Minimal CI Rust CLI...${NC}"

# Check for cargo
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust and Cargo are required but not installed.${NC}"
    echo "Please install Rust from https://rustup.rs/ and try again."
    exit 1
fi

# Run cargo check to validate syntax
echo -e "${YELLOW}Checking code...${NC}"
cargo check

if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Code check failed.${NC}"
    exit 1
fi

# Run cargo build
echo -e "${YELLOW}Building binary...${NC}"
cargo build

if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Build failed.${NC}"
    exit 1
fi

echo -e "${GREEN}Build successful!${NC}"

echo -e "\n${GREEN}Minimal CI Rust CLI built successfully!${NC}"
echo -e "You can run the binary with: ./target/debug/ci"
echo -e "Or install it with: ./install.sh"

# Create installation script
cat > install.sh << 'EOL'
#!/usr/bin/env bash
# Install script for minimal CI Rust CLI

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_DIR="${HOME}/.local/bin"

echo -e "${YELLOW}Installing Minimal CI Rust CLI...${NC}"

# Build release version
echo -e "${YELLOW}Building release version...${NC}"
cd "$SCRIPT_DIR"
cargo build --release

if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Build failed.${NC}"
    exit 1
fi

# Create installation directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Copy binary to installation directory
echo -e "${YELLOW}Installing binary to $INSTALL_DIR/ci...${NC}"
cp "$SCRIPT_DIR/target/release/ci" "$INSTALL_DIR/"

# Make binary executable
chmod +x "$INSTALL_DIR/ci"

# Check if installation directory is in PATH
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo -e "${YELLOW}Adding $INSTALL_DIR to PATH in your shell configuration file...${NC}"
    
    # Determine shell configuration file
    if [ -n "$ZSH_VERSION" ] || [ "$SHELL" = "/bin/zsh" ] || [ "$SHELL" = "/usr/bin/zsh" ]; then
        SHELL_CONFIG="${HOME}/.zshrc"
    elif [ -n "$BASH_VERSION" ] || [ "$SHELL" = "/bin/bash" ] || [ "$SHELL" = "/usr/bin/bash" ]; then
        SHELL_CONFIG="${HOME}/.bashrc"
        if [ "$(uname)" = "Darwin" ]; then
            SHELL_CONFIG="${HOME}/.bash_profile"
        fi
    else
        SHELL_CONFIG="${HOME}/.profile"
    fi
    
    echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_CONFIG"
    echo -e "${YELLOW}Added $INSTALL_DIR to PATH in $SHELL_CONFIG${NC}"
    echo -e "${YELLOW}Please restart your shell or run 'source $SHELL_CONFIG' to update your PATH.${NC}"
fi

echo -e "\n${GREEN}CI Rust CLI installed successfully!${NC}"
echo -e "You can now run 'ci --help' to see available commands."
echo -e "\n${YELLOW}Example usage:${NC}"
echo -e "  ci load Athena"
echo -e "  ci init my-project"
echo -e "  ci status"

exit 0
EOL

chmod +x install.sh

exit 0