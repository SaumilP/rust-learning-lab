#!/bin/bash
# Phase 2 Setup - Condensed Command List
# Base: /home/admin-and/code/personal/rust-learning-lab/

BASE="/home/admin-and/code/personal/rust-learning-lab"

# ============================================================================
# MODULE 01: Core Fundamentals
# ============================================================================

# Directories
mkdir -p "$BASE/01-core-fundamentals/variables_and_mutability/examples"
mkdir -p "$BASE/01-core-fundamentals/data_types/examples"
mkdir -p "$BASE/01-core-fundamentals/functions/examples"
mkdir -p "$BASE/01-core-fundamentals/control_flow/examples"
mkdir -p "$BASE/01-core-fundamentals/exercises/exercise_1_type_conversion"
mkdir -p "$BASE/01-core-fundamentals/exercises/exercise_2_string_manipulation"
mkdir -p "$BASE/01-core-fundamentals/exercises/exercise_3_fizzbuzz"
mkdir -p "$BASE/01-core-fundamentals/exercises/exercise_4_pattern_generation"

# .gitkeep files
touch "$BASE/01-core-fundamentals/variables_and_mutability/examples/.gitkeep"
touch "$BASE/01-core-fundamentals/data_types/examples/.gitkeep"
touch "$BASE/01-core-fundamentals/functions/examples/.gitkeep"
touch "$BASE/01-core-fundamentals/control_flow/examples/.gitkeep"

# README.md files (only if not exists)
[ ! -f "$BASE/01-core-fundamentals/variables_and_mutability/README.md" ] && echo "# Variables and Mutability" > "$BASE/01-core-fundamentals/variables_and_mutability/README.md"
[ ! -f "$BASE/01-core-fundamentals/data_types/README.md" ] && echo "# Data Types" > "$BASE/01-core-fundamentals/data_types/README.md"
[ ! -f "$BASE/01-core-fundamentals/functions/README.md" ] && echo "# Functions" > "$BASE/01-core-fundamentals/functions/README.md"
[ ! -f "$BASE/01-core-fundamentals/control_flow/README.md" ] && echo "# Control Flow" > "$BASE/01-core-fundamentals/control_flow/README.md"

# key_takeaways.md files (only if not exists)
[ ! -f "$BASE/01-core-fundamentals/variables_and_mutability/key_takeaways.md" ] && echo "# Key Takeaways: Variables and Mutability" > "$BASE/01-core-fundamentals/variables_and_mutability/key_takeaways.md"
[ ! -f "$BASE/01-core-fundamentals/data_types/key_takeaways.md" ] && echo "# Key Takeaways: Data Types" > "$BASE/01-core-fundamentals/data_types/key_takeaways.md"
[ ! -f "$BASE/01-core-fundamentals/functions/key_takeaways.md" ] && echo "# Key Takeaways: Functions" > "$BASE/01-core-fundamentals/functions/key_takeaways.md"
[ ! -f "$BASE/01-core-fundamentals/control_flow/key_takeaways.md" ] && echo "# Key Takeaways: Control Flow" > "$BASE/01-core-fundamentals/control_flow/key_takeaways.md"

# ============================================================================
# MODULE 02: Standard Library
# ============================================================================

# Directories
mkdir -p "$BASE/02-standard-library/collections/examples"
mkdir -p "$BASE/02-standard-library/string_operations/examples"
mkdir -p "$BASE/02-standard-library/iterator_patterns/examples"
mkdir -p "$BASE/02-standard-library/common_traits/examples"
mkdir -p "$BASE/02-standard-library/error_handling_basics/examples"
mkdir -p "$BASE/02-standard-library/exercises/exercise_1_collection_manipulation"
mkdir -p "$BASE/02-standard-library/exercises/exercise_2_iterator_chain"
mkdir -p "$BASE/02-standard-library/exercises/exercise_3_parse_validate"
mkdir -p "$BASE/02-standard-library/exercises/exercise_4_find_duplicates"

# .gitkeep files
touch "$BASE/02-standard-library/collections/examples/.gitkeep"
touch "$BASE/02-standard-library/string_operations/examples/.gitkeep"
touch "$BASE/02-standard-library/iterator_patterns/examples/.gitkeep"
touch "$BASE/02-standard-library/common_traits/examples/.gitkeep"
touch "$BASE/02-standard-library/error_handling_basics/examples/.gitkeep"

# README.md files (only if not exists)
[ ! -f "$BASE/02-standard-library/collections/README.md" ] && echo "# Collections" > "$BASE/02-standard-library/collections/README.md"
[ ! -f "$BASE/02-standard-library/string_operations/README.md" ] && echo "# String Operations" > "$BASE/02-standard-library/string_operations/README.md"
[ ! -f "$BASE/02-standard-library/iterator_patterns/README.md" ] && echo "# Iterator Patterns" > "$BASE/02-standard-library/iterator_patterns/README.md"
[ ! -f "$BASE/02-standard-library/common_traits/README.md" ] && echo "# Common Traits" > "$BASE/02-standard-library/common_traits/README.md"
[ ! -f "$BASE/02-standard-library/error_handling_basics/README.md" ] && echo "# Error Handling Basics" > "$BASE/02-standard-library/error_handling_basics/README.md"

# key_takeaways.md files (only if not exists)
[ ! -f "$BASE/02-standard-library/collections/key_takeaways.md" ] && echo "# Key Takeaways: Collections" > "$BASE/02-standard-library/collections/key_takeaways.md"
[ ! -f "$BASE/02-standard-library/string_operations/key_takeaways.md" ] && echo "# Key Takeaways: String Operations" > "$BASE/02-standard-library/string_operations/key_takeaways.md"
[ ! -f "$BASE/02-standard-library/iterator_patterns/key_takeaways.md" ] && echo "# Key Takeaways: Iterator Patterns" > "$BASE/02-standard-library/iterator_patterns/key_takeaways.md"
[ ! -f "$BASE/02-standard-library/common_traits/key_takeaways.md" ] && echo "# Key Takeaways: Common Traits" > "$BASE/02-standard-library/common_traits/key_takeaways.md"
[ ! -f "$BASE/02-standard-library/error_handling_basics/key_takeaways.md" ] && echo "# Key Takeaways: Error Handling Basics" > "$BASE/02-standard-library/error_handling_basics/key_takeaways.md"

# ============================================================================
# MODULE 03: Tooling and Quality
# ============================================================================

# Directories
mkdir -p "$BASE/03-tooling-and-quality/testing/examples"
mkdir -p "$BASE/03-tooling-and-quality/documentation/examples"
mkdir -p "$BASE/03-tooling-and-quality/code_quality_tools/examples"
mkdir -p "$BASE/03-tooling-and-quality/debugging/examples"
mkdir -p "$BASE/03-tooling-and-quality/exercises/exercise_1_failing_tests"
mkdir -p "$BASE/03-tooling-and-quality/exercises/exercise_2_documentation"
mkdir -p "$BASE/03-tooling-and-quality/exercises/exercise_3_clippy_warnings"
mkdir -p "$BASE/03-tooling-and-quality/exercises/exercise_4_debug_output"

# .gitkeep files
touch "$BASE/03-tooling-and-quality/testing/examples/.gitkeep"
touch "$BASE/03-tooling-and-quality/documentation/examples/.gitkeep"
touch "$BASE/03-tooling-and-quality/code_quality_tools/examples/.gitkeep"
touch "$BASE/03-tooling-and-quality/debugging/examples/.gitkeep"

# README.md files (only if not exists)
[ ! -f "$BASE/03-tooling-and-quality/testing/README.md" ] && echo "# Testing" > "$BASE/03-tooling-and-quality/testing/README.md"
[ ! -f "$BASE/03-tooling-and-quality/documentation/README.md" ] && echo "# Documentation" > "$BASE/03-tooling-and-quality/documentation/README.md"
[ ! -f "$BASE/03-tooling-and-quality/code_quality_tools/README.md" ] && echo "# Code Quality Tools" > "$BASE/03-tooling-and-quality/code_quality_tools/README.md"
[ ! -f "$BASE/03-tooling-and-quality/debugging/README.md" ] && echo "# Debugging" > "$BASE/03-tooling-and-quality/debugging/README.md"

# key_takeaways.md files (only if not exists)
[ ! -f "$BASE/03-tooling-and-quality/testing/key_takeaways.md" ] && echo "# Key Takeaways: Testing" > "$BASE/03-tooling-and-quality/testing/key_takeaways.md"
[ ! -f "$BASE/03-tooling-and-quality/documentation/key_takeaways.md" ] && echo "# Key Takeaways: Documentation" > "$BASE/03-tooling-and-quality/documentation/key_takeaways.md"
[ ! -f "$BASE/03-tooling-and-quality/code_quality_tools/key_takeaways.md" ] && echo "# Key Takeaways: Code Quality Tools" > "$BASE/03-tooling-and-quality/code_quality_tools/key_takeaways.md"
[ ! -f "$BASE/03-tooling-and-quality/debugging/key_takeaways.md" ] && echo "# Key Takeaways: Debugging" > "$BASE/03-tooling-and-quality/debugging/key_takeaways.md"

echo "Phase 2 structure setup complete!"
