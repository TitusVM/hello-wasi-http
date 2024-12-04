#!/bin/bash

# Exit immediately if any command fails
set -e

# Step 0: Generate signing keys
echo "Generating signing keys..."
wasmsign2 keygen --public-key public.key --secret-key secret.key

# Create the binaries folder if it doesn't exist
mkdir -p binaries

TARGET="wasm32-wasip2"

# Function to build, sign, and compose components
build_and_sign_components() {
  local suffix=$1  # Suffix for signed binaries and composed WASM
  
  # Build and sign components
  for component in hello_wasi_http rpn; do
    echo "Processing component: $component ($suffix)"
    
    # Change to the component directory
    cd $component
    
    # Compile the component
    echo "Building component..."
    cargo auditable build --target=$TARGET
    
    # Sign the component
    echo "Signing component..."
    SIGNED_WASM="../binaries/signed_${component}_${suffix}.wasm"
    wasmsign2 sign -k ../secret.key -i target/$TARGET/debug/$component.wasm -o $SIGNED_WASM
    
    # Return to the root directory
    cd ..
  done

  # Compose the components
  echo "Composing components ($suffix)..."
  COMPOSED_WASM="binaries/composed_${suffix}.wasm"
  wac plug binaries/signed_hello_wasi_http_${suffix}.wasm --plug binaries/signed_rpn_${suffix}.wasm -o $COMPOSED_WASM
  
  # Sign the composed component
  SIGNED_COMPOSED_WASM="signed_composed_${suffix}.wasm"
  echo "Signing composed component ($suffix)..."
  wasmsign2 sign -k secret.key -i $COMPOSED_WASM -o $SIGNED_COMPOSED_WASM
}

# Process original Cargo.toml
echo "Building and signing components with original Cargo.toml..."
build_and_sign_components "safe"


echo "All operations completed successfully!"

