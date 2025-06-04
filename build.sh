#!/bin/bash

# Exit immediately if any command fails
set -e

# Define some ANSI color codes for styling
GREEN='\033[1;92m'
BLUE='\033[1;94m'
RED='\033[1;91m'
YELLOW='\033[1;93m'
NC='\033[0m' # No color

# Pretty banner
echo -e "${BLUE}==============================="
echo -e "🚀 Rust Library Build & Test 🔧"
echo -e "===============================${NC}"

# Step 1: Build the library
echo -e "${YELLOW}🔨 Building the Rust library...${NC}"
cargo build --lib
echo -e "${GREEN}✅ Build successful!${NC}"

# Step 2: Run the library tests
echo -e "${YELLOW}🧪 Running library tests...${NC}"
cargo test --lib
echo -e "${GREEN}✅ All tests passed!${NC}"

# Done
echo -e "${BLUE}==============================="
echo -e "🎉 Done! Everything looks great."
echo -e "===============================${NC}"
