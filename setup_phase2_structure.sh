#!/bin/bash

# Phase 2 Directory Structure Setup Script
# For Rust Learning Lab - Modules 01, 02, 03
# Base path: /home/admin-and/code/personal/rust-learning-lab/

set -e  # Exit on error

BASE_PATH="/home/admin-and/code/personal/rust-learning-lab"

echo "Setting up Phase 2 directory structure..."
echo "=========================================="

# ============================================================================
# MODULE 01: Core Fundamentals
# ============================================================================

echo ""
echo "Module 01: Core Fundamentals"
echo "-----------------------------"

# Create concept directories (most already exist, but ensuring they're created)
mkdir -p "$BASE_PATH/01-core-fundamentals/variables_and_mutability/examples"
mkdir -p "$BASE_PATH/01-core-fundamentals/data_types/examples"
mkdir -p "$BASE_PATH/01-core-fundamentals/functions/examples"
mkdir -p "$BASE_PATH/01-core-fundamentals/control_flow/examples"

# Create exercise directories
mkdir -p "$BASE_PATH/01-core-fundamentals/exercises/exercise_1_type_conversion"
mkdir -p "$BASE_PATH/01-core-fundamentals/exercises/exercise_2_string_manipulation"
mkdir -p "$BASE_PATH/01-core-fundamentals/exercises/exercise_3_fizzbuzz"
mkdir -p "$BASE_PATH/01-core-fundamentals/exercises/exercise_4_pattern_generation"

# Create .gitkeep files for examples directories
touch "$BASE_PATH/01-core-fundamentals/variables_and_mutability/examples/.gitkeep"
touch "$BASE_PATH/01-core-fundamentals/data_types/examples/.gitkeep"
touch "$BASE_PATH/01-core-fundamentals/functions/examples/.gitkeep"
touch "$BASE_PATH/01-core-fundamentals/control_flow/examples/.gitkeep"

# Create README.md for variables_and_mutability (if not exists)
if [ ! -f "$BASE_PATH/01-core-fundamentals/variables_and_mutability/README.md" ]; then
  echo "# Variables and Mutability" > "$BASE_PATH/01-core-fundamentals/variables_and_mutability/README.md"
fi

# Create key_takeaways.md for variables_and_mutability (if not exists)
if [ ! -f "$BASE_PATH/01-core-fundamentals/variables_and_mutability/key_takeaways.md" ]; then
  echo "# Key Takeaways: Variables and Mutability" > "$BASE_PATH/01-core-fundamentals/variables_and_mutability/key_takeaways.md"
fi

# Create README.md for data_types (if not exists)
if [ ! -f "$BASE_PATH/01-core-fundamentals/data_types/README.md" ]; then
  echo "# Data Types" > "$BASE_PATH/01-core-fundamentals/data_types/README.md"
fi

# Create key_takeaways.md for data_types (if not exists)
if [ ! -f "$BASE_PATH/01-core-fundamentals/data_types/key_takeaways.md" ]; then
  echo "# Key Takeaways: Data Types" > "$BASE_PATH/01-core-fundamentals/data_types/key_takeaways.md"
fi

# Create README.md for functions (if not exists)
if [ ! -f "$BASE_PATH/01-core-fundamentals/functions/README.md" ]; then
  echo "# Functions" > "$BASE_PATH/01-core-fundamentals/functions/README.md"
fi

# Create key_takeaways.md for functions (if not exists)
if [ ! -f "$BASE_PATH/01-core-fundamentals/functions/key_takeaways.md" ]; then
  echo "# Key Takeaways: Functions" > "$BASE_PATH/01-core-fundamentals/functions/key_takeaways.md"
fi

# Create README.md for control_flow (if not exists)
if [ ! -f "$BASE_PATH/01-core-fundamentals/control_flow/README.md" ]; then
  echo "# Control Flow" > "$BASE_PATH/01-core-fundamentals/control_flow/README.md"
fi

# Create key_takeaways.md for control_flow (if not exists)
if [ ! -f "$BASE_PATH/01-core-fundamentals/control_flow/key_takeaways.md" ]; then
  echo "# Key Takeaways: Control Flow" > "$BASE_PATH/01-core-fundamentals/control_flow/key_takeaways.md"
fi

echo "✓ Module 01 structure created"

# ============================================================================
# MODULE 02: Standard Library
# ============================================================================

echo ""
echo "Module 02: Standard Library"
echo "---------------------------"

# Create concept directories
mkdir -p "$BASE_PATH/02-standard-library/collections/examples"
mkdir -p "$BASE_PATH/02-standard-library/string_operations/examples"
mkdir -p "$BASE_PATH/02-standard-library/iterator_patterns/examples"
mkdir -p "$BASE_PATH/02-standard-library/common_traits/examples"
mkdir -p "$BASE_PATH/02-standard-library/error_handling_basics/examples"

# Create exercise directories
mkdir -p "$BASE_PATH/02-standard-library/exercises/exercise_1_collection_manipulation"
mkdir -p "$BASE_PATH/02-standard-library/exercises/exercise_2_iterator_chain"
mkdir -p "$BASE_PATH/02-standard-library/exercises/exercise_3_parse_validate"
mkdir -p "$BASE_PATH/02-standard-library/exercises/exercise_4_find_duplicates"

# Create .gitkeep files for examples directories
touch "$BASE_PATH/02-standard-library/collections/examples/.gitkeep"
touch "$BASE_PATH/02-standard-library/string_operations/examples/.gitkeep"
touch "$BASE_PATH/02-standard-library/iterator_patterns/examples/.gitkeep"
touch "$BASE_PATH/02-standard-library/common_traits/examples/.gitkeep"
touch "$BASE_PATH/02-standard-library/error_handling_basics/examples/.gitkeep"

# Create README.md for collections (if not exists)
if [ ! -f "$BASE_PATH/02-standard-library/collections/README.md" ]; then
  echo "# Collections" > "$BASE_PATH/02-standard-library/collections/README.md"
fi

# Create key_takeaways.md for collections (if not exists)
if [ ! -f "$BASE_PATH/02-standard-library/collections/key_takeaways.md" ]; then
  echo "# Key Takeaways: Collections" > "$BASE_PATH/02-standard-library/collections/key_takeaways.md"
fi

# Create README.md for string_operations (if not exists)
if [ ! -f "$BASE_PATH/02-standard-library/string_operations/README.md" ]; then
  echo "# String Operations" > "$BASE_PATH/02-standard-library/string_operations/README.md"
fi

# Create key_takeaways.md for string_operations (if not exists)
if [ ! -f "$BASE_PATH/02-standard-library/string_operations/key_takeaways.md" ]; then
  echo "# Key Takeaways: String Operations" > "$BASE_PATH/02-standard-library/string_operations/key_takeaways.md"
fi

# Create README.md for iterator_patterns (if not exists)
if [ ! -f "$BASE_PATH/02-standard-library/iterator_patterns/README.md" ]; then
  echo "# Iterator Patterns" > "$BASE_PATH/02-standard-library/iterator_patterns/README.md"
fi

# Create key_takeaways.md for iterator_patterns (if not exists)
if [ ! -f "$BASE_PATH/02-standard-library/iterator_patterns/key_takeaways.md" ]; then
  echo "# Key Takeaways: Iterator Patterns" > "$BASE_PATH/02-standard-library/iterator_patterns/key_takeaways.md"
fi

# Create README.md for common_traits (if not exists)
if [ ! -f "$BASE_PATH/02-standard-library/common_traits/README.md" ]; then
  echo "# Common Traits" > "$BASE_PATH/02-standard-library/common_traits/README.md"
fi

# Create key_takeaways.md for common_traits (if not exists)
if [ ! -f "$BASE_PATH/02-standard-library/common_traits/key_takeaways.md" ]; then
  echo "# Key Takeaways: Common Traits" > "$BASE_PATH/02-standard-library/common_traits/key_takeaways.md"
fi

# Create README.md for error_handling_basics (if not exists)
if [ ! -f "$BASE_PATH/02-standard-library/error_handling_basics/README.md" ]; then
  echo "# Error Handling Basics" > "$BASE_PATH/02-standard-library/error_handling_basics/README.md"
fi

# Create key_takeaways.md for error_handling_basics (if not exists)
if [ ! -f "$BASE_PATH/02-standard-library/error_handling_basics/key_takeaways.md" ]; then
  echo "# Key Takeaways: Error Handling Basics" > "$BASE_PATH/02-standard-library/error_handling_basics/key_takeaways.md"
fi

echo "✓ Module 02 structure created"

# ============================================================================
# MODULE 03: Tooling and Quality
# ============================================================================

echo ""
echo "Module 03: Tooling and Quality"
echo "-------------------------------"

# Create concept directories
mkdir -p "$BASE_PATH/03-tooling-and-quality/testing/examples"
mkdir -p "$BASE_PATH/03-tooling-and-quality/documentation/examples"
mkdir -p "$BASE_PATH/03-tooling-and-quality/code_quality_tools/examples"
mkdir -p "$BASE_PATH/03-tooling-and-quality/debugging/examples"

# Create exercise directories
mkdir -p "$BASE_PATH/03-tooling-and-quality/exercises/exercise_1_failing_tests"
mkdir -p "$BASE_PATH/03-tooling-and-quality/exercises/exercise_2_documentation"
mkdir -p "$BASE_PATH/03-tooling-and-quality/exercises/exercise_3_clippy_warnings"
mkdir -p "$BASE_PATH/03-tooling-and-quality/exercises/exercise_4_debug_output"

# Create .gitkeep files for examples directories
touch "$BASE_PATH/03-tooling-and-quality/testing/examples/.gitkeep"
touch "$BASE_PATH/03-tooling-and-quality/documentation/examples/.gitkeep"
touch "$BASE_PATH/03-tooling-and-quality/code_quality_tools/examples/.gitkeep"
touch "$BASE_PATH/03-tooling-and-quality/debugging/examples/.gitkeep"

# Create README.md for testing (if not exists)
if [ ! -f "$BASE_PATH/03-tooling-and-quality/testing/README.md" ]; then
  echo "# Testing" > "$BASE_PATH/03-tooling-and-quality/testing/README.md"
fi

# Create key_takeaways.md for testing (if not exists)
if [ ! -f "$BASE_PATH/03-tooling-and-quality/testing/key_takeaways.md" ]; then
  echo "# Key Takeaways: Testing" > "$BASE_PATH/03-tooling-and-quality/testing/key_takeaways.md"
fi

# Create README.md for documentation (if not exists)
if [ ! -f "$BASE_PATH/03-tooling-and-quality/documentation/README.md" ]; then
  echo "# Documentation" > "$BASE_PATH/03-tooling-and-quality/documentation/README.md"
fi

# Create key_takeaways.md for documentation (if not exists)
if [ ! -f "$BASE_PATH/03-tooling-and-quality/documentation/key_takeaways.md" ]; then
  echo "# Key Takeaways: Documentation" > "$BASE_PATH/03-tooling-and-quality/documentation/key_takeaways.md"
fi

# Create README.md for code_quality_tools (if not exists)
if [ ! -f "$BASE_PATH/03-tooling-and-quality/code_quality_tools/README.md" ]; then
  echo "# Code Quality Tools" > "$BASE_PATH/03-tooling-and-quality/code_quality_tools/README.md"
fi

# Create key_takeaways.md for code_quality_tools (if not exists)
if [ ! -f "$BASE_PATH/03-tooling-and-quality/code_quality_tools/key_takeaways.md" ]; then
  echo "# Key Takeaways: Code Quality Tools" > "$BASE_PATH/03-tooling-and-quality/code_quality_tools/key_takeaways.md"
fi

# Create README.md for debugging (if not exists)
if [ ! -f "$BASE_PATH/03-tooling-and-quality/debugging/README.md" ]; then
  echo "# Debugging" > "$BASE_PATH/03-tooling-and-quality/debugging/README.md"
fi

# Create key_takeaways.md for debugging (if not exists)
if [ ! -f "$BASE_PATH/03-tooling-and-quality/debugging/key_takeaways.md" ]; then
  echo "# Key Takeaways: Debugging" > "$BASE_PATH/03-tooling-and-quality/debugging/key_takeaways.md"
fi

echo "✓ Module 03 structure created"

# ============================================================================
# Summary
# ============================================================================

echo ""
echo "=========================================="
echo "Phase 2 directory structure setup complete!"
echo "=========================================="
echo ""
echo "Summary:"
echo "--------"
echo "Module 01: 4 concept folders + 4 exercise folders"
echo "Module 02: 5 concept folders + 4 exercise folders"
echo "Module 03: 4 concept folders + 4 exercise folders"
echo ""
echo "Each concept folder contains:"
echo "  - examples/ directory with .gitkeep"
echo "  - README.md (placeholder)"
echo "  - key_takeaways.md (placeholder)"
echo ""
echo "All exercise directories have been created."
echo ""
