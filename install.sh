#!/bin/bash

# --- Metropolis Installation Script ---
# "The city is breathing."

# Colors for the terminal
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${CYAN}"
echo "  __  __   ______  _______  _____    ____   _____    ____   _       _____   _____ "
echo " |  \/  | |  ____||__   __||  __ \  / __ \ |  __ \  / __ \ | |     |_   _| / ____|"
echo " | \  / | | |__      | |   | |__) || |  | || |__) || |  | || |       | |  | (___  "
echo " | |\/| | |  __|     | |   |  _  / | |  | ||  ___/ | |  | || |       | |   \___ \ "
echo " | |  | | | |____    | |   | | \ \ | |__| || |     | |__| || |____  _| |_  ____) |"
echo " |_|  |_| |______|   |_|   |_|  \_\ \____/ |_|      \____/ |______||_____||_____/ "
echo -e "${NC}"
echo -e "${MAGENTA}Initializing city infrastructure...${NC}"

# Check for Cargo (Rust)
if ! command -v cargo &> /dev/null
then
    echo -e "${RED}Error: Rust (cargo) is not installed.${NC}"
    echo -e "Metropolis requires Rust to build. Install it at: https://rustup.rs/"
    exit 1
fi

# Create a temporary directory
TEMP_DIR=$(mktemp -d)
echo -e "-> Downloading architecture to $TEMP_DIR..."

# Clone the repository
git clone --depth 1 https://github.com/5c0/metropolis.git "$TEMP_DIR" > /dev/null 2>&1

cd "$TEMP_DIR" || exit

# Build the project
echo -e "-> Constructing binary (Release Mode)..."
cargo build --release > /dev/null 2>&1

# Determine install location
# We try ~/.local/bin as it's cleaner for users, or /usr/local/bin if they want global
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

echo -e "-> Installing binary to $INSTALL_DIR..."
cp target/release/metropolis "$INSTALL_DIR/metropolis"

# Clean up
echo -e "-> Cleaning up construction site..."
rm -rf "$TEMP_DIR"

echo -e "\n${GREEN}Metropolis is live!${NC}"
echo -e "You can now run it by typing: ${CYAN}metropolis${NC}"
echo -e "(Make sure $INSTALL_DIR is in your PATH)\n"
