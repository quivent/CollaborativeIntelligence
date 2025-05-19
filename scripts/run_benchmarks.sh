#!/usr/bin/env bash
# run_benchmarks.sh - Benchmark script for memory optimization system
# Part of the Memory Optimization Integration Plan

set -euo pipefail

# Determine script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CI_ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Source time measurement utilities
source "$CI_ROOT_DIR/ci-tools/lib/common.sh"

# Set paths
MEMORY_CLI_PATH="${CI_ROOT_DIR}/target/release/memory_cli"
BENCHMARK_DIR="${SCRIPT_DIR}/benchmark_results"
LEGACY_SCRIPT="${CI_ROOT_DIR}/ci-tools/lib/memory-loader.sh"
BRIDGE_SCRIPT="${CI_ROOT_DIR}/ci-tools/lib/rust-memory-bridge.sh"

# Create benchmark directory
mkdir -p "$BENCHMARK_DIR"

# Create benchmark output file
BENCHMARK_FILE="${BENCHMARK_DIR}/memory_benchmark_$(date +%Y%m%d_%H%M%S).csv"
REPORT_FILE="${BENCHMARK_DIR}/memory_benchmark_$(date +%Y%m%d_%H%M%S).md"

# Write CSV header
echo "Operation,System,Run,Time (ms),File I/O Count,Memory Usage (KB)" > "$BENCHMARK_FILE"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Sample agents for testing
AGENTS=("Athena" "Developer" "Optimizer")

# Check if memory CLI is available, build if not
if [[ ! -f "$MEMORY_CLI_PATH" ]]; then
    echo -e "${YELLOW}Memory CLI not found. Building...${NC}"
    cd "$SCRIPT_DIR"
    ./build_memory_cli.sh
    
    if [[ ! -f "$MEMORY_CLI_PATH" ]]; then
        echo -e "${RED}ERROR: Failed to build memory CLI.${NC}"
        exit 1
    fi
fi

# Check if the legacy script is available
if [[ ! -f "$LEGACY_SCRIPT" ]]; then
    echo -e "${YELLOW}WARNING: Legacy memory script not found.${NC}"
    echo -e "${YELLOW}Benchmarking will only test the optimized system.${NC}"
    RUN_LEGACY=false
else
    RUN_LEGACY=true
fi

# Check if the bridge script is available
if [[ ! -f "$BRIDGE_SCRIPT" ]]; then
    echo -e "${YELLOW}WARNING: Bridge script not found.${NC}"
    echo -e "${YELLOW}Benchmarking will not include bridge tests.${NC}"
    RUN_BRIDGE=false
else
    RUN_BRIDGE=true
    source "$BRIDGE_SCRIPT"
fi

# Function to count I/O operations
count_io_operations() {
    local cmd="$1"
    local temp_strace="${BENCHMARK_DIR}/strace_$(date +%s).txt"
    
    # Use strace to count file operations
    strace -e trace=file -o "$temp_strace" $cmd >/dev/null 2>&1 || true
    
    # Count open, read, write, stat operations
    local count=$(grep -E 'open|read|write|stat' "$temp_strace" | wc -l)
    
    # Clean up
    rm -f "$temp_strace"
    
    echo "$count"
}

# Function to measure memory usage
measure_memory() {
    local cmd="$1"
    local temp_mem="${BENCHMARK_DIR}/mem_$(date +%s).txt"
    
    # Use time to measure memory usage
    /usr/bin/time -f "%M" -o "$temp_mem" $cmd >/dev/null 2>&1 || true
    
    # Read memory usage
    local mem=$(cat "$temp_mem")
    
    # Clean up
    rm -f "$temp_mem"
    
    echo "$mem"
}

# Function to measure execution time
measure_time() {
    local start_time=$(date +%s%N)
    eval "$1" >/dev/null 2>&1 || true
    local end_time=$(date +%s%N)
    
    # Calculate time in milliseconds
    echo $(( (end_time - start_time) / 1000000 ))
}

# Function to run a single benchmark
run_benchmark() {
    local operation="$1"
    local system="$2"
    local cmd="$3"
    local run_number="$4"
    
    echo -e "${BLUE}Benchmarking: $operation - $system (Run $run_number)${NC}"
    
    # Measure time
    local time_ms=$(measure_time "$cmd")
    
    # Count I/O operations
    local io_count=0
    # Skip I/O counting if operation is expensive
    if [[ "$operation" != "load_all_agents" ]]; then
        io_count=$(count_io_operations "$cmd")
    fi
    
    # Measure memory usage
    local memory=$(measure_memory "$cmd")
    
    # Log to CSV
    echo "$operation,$system,$run_number,$time_ms,$io_count,$memory" >> "$BENCHMARK_FILE"
    
    echo -e "${GREEN}  Time: ${time_ms}ms, I/O: ${io_count}, Memory: ${memory}KB${NC}"
}

# Function to create benchmark report
create_benchmark_report() {
    echo -e "${BLUE}Creating benchmark report...${NC}"
    
    # Create report header
    cat > "$REPORT_FILE" << EOF
# Memory Optimization Benchmark Report

Generated on: $(date)

## Summary

This report compares the performance of the memory optimization system with the legacy implementation.

## Benchmark Results

EOF

    # Process CSV data with awk to create summary tables
    awk -F, 'BEGIN {
        print "| Operation | System | Avg Time (ms) | Avg I/O Count | Avg Memory (KB) |";
        print "|-----------|--------|--------------|---------------|----------------|";
    }
    NR > 1 {
        operations[$1][$2]["time"] += $4;
        operations[$1][$2]["io"] += $5;
        operations[$1][$2]["memory"] += $6;
        operations[$1][$2]["count"]++;
    }
    END {
        for (op in operations) {
            for (sys in operations[op]) {
                count = operations[op][sys]["count"];
                avg_time = operations[op][sys]["time"] / count;
                avg_io = operations[op][sys]["io"] / count;
                avg_memory = operations[op][sys]["memory"] / count;
                printf "| %s | %s | %.2f | %.2f | %.2f |\n", 
                    op, sys, avg_time, avg_io, avg_memory;
            }
        }
    }' "$BENCHMARK_FILE" >> "$REPORT_FILE"
    
    # Add improvement calculations
    cat >> "$REPORT_FILE" << EOF

## Improvement Analysis

EOF

    # Process CSV data with awk to calculate improvements
    awk -F, 'BEGIN {
        print "| Operation | Time Improvement | I/O Reduction | Memory Efficiency |";
        print "|-----------|------------------|---------------|-------------------|";
    }
    NR > 1 {
        operations[$1][$2]["time"] += $4;
        operations[$1][$2]["io"] += $5;
        operations[$1][$2]["memory"] += $6;
        operations[$1][$2]["count"]++;
    }
    END {
        for (op in operations) {
            if ("Legacy" in operations[op] && "Optimized" in operations[op]) {
                legacy_time = operations[op]["Legacy"]["time"] / operations[op]["Legacy"]["count"];
                optimized_time = operations[op]["Optimized"]["time"] / operations[op]["Optimized"]["count"];
                time_improvement = (legacy_time - optimized_time) / legacy_time * 100;
                
                legacy_io = operations[op]["Legacy"]["io"] / operations[op]["Legacy"]["count"];
                optimized_io = operations[op]["Optimized"]["io"] / operations[op]["Optimized"]["count"];
                io_reduction = (legacy_io - optimized_io) / legacy_io * 100;
                
                legacy_memory = operations[op]["Legacy"]["memory"] / operations[op]["Legacy"]["count"];
                optimized_memory = operations[op]["Optimized"]["memory"] / operations[op]["Optimized"]["count"];
                memory_efficiency = (legacy_memory - optimized_memory) / legacy_memory * 100;
                
                printf "| %s | %.2f%% | %.2f%% | %.2f%% |\n", 
                    op, time_improvement, io_reduction, memory_efficiency;
            }
        }
    }' "$BENCHMARK_FILE" >> "$REPORT_FILE"
    
    echo -e "${GREEN}Benchmark report created: $REPORT_FILE${NC}"
}

# Run benchmarks

echo -e "${BLUE}Starting benchmark suite...${NC}"

# Number of runs for each test
NUM_RUNS=5

# Benchmark 1: Create agent cache
for run in $(seq 1 $NUM_RUNS); do
    # Optimized system
    run_benchmark "create_cache" "Optimized" \
        "$MEMORY_CLI_PATH cache --agent TestAgent --directory /test/dir --initializer Benchmark --intent 'Benchmark test'" \
        $run
    
    # Legacy system
    if [[ "$RUN_LEGACY" == "true" ]]; then
        run_benchmark "create_cache" "Legacy" \
            "bash -c 'source $LEGACY_SCRIPT && create_agent_cache TestAgent /test/dir Benchmark \"Benchmark test\"'" \
            $run
    fi
    
    # Bridge
    if [[ "$RUN_BRIDGE" == "true" ]]; then
        run_benchmark "create_cache" "Bridge" \
            "bash -c 'source $BRIDGE_SCRIPT && create_agent_cache TestAgent /test/dir Benchmark \"Benchmark test\"'" \
            $run
    fi
done

# Benchmark 2: Load agent memory
for agent in "${AGENTS[@]}"; do
    for run in $(seq 1 $NUM_RUNS); do
        # Optimized system
        run_benchmark "load_memory_$agent" "Optimized" \
            "$MEMORY_CLI_PATH load --agent $agent" \
            $run
        
        # Legacy system
        if [[ "$RUN_LEGACY" == "true" ]]; then
            run_benchmark "load_memory_$agent" "Legacy" \
                "bash -c 'source $LEGACY_SCRIPT && load_agent_memory $agent'" \
                $run
        fi
        
        # Bridge
        if [[ "$RUN_BRIDGE" == "true" ]]; then
            run_benchmark "load_memory_$agent" "Bridge" \
                "bash -c 'source $BRIDGE_SCRIPT && load_agent_memory $agent'" \
                $run
        fi
    done
done

# Benchmark 3: Load all agents sequentially
for run in $(seq 1 $NUM_RUNS); do
    # Create command to load all agents
    optimized_cmd=""
    legacy_cmd=""
    bridge_cmd=""
    
    for agent in "${AGENTS[@]}"; do
        optimized_cmd+="$MEMORY_CLI_PATH load --agent $agent >/dev/null 2>&1; "
        
        if [[ "$RUN_LEGACY" == "true" ]]; then
            legacy_cmd+="load_agent_memory $agent >/dev/null 2>&1; "
        fi
        
        if [[ "$RUN_BRIDGE" == "true" ]]; then
            bridge_cmd+="load_agent_memory $agent >/dev/null 2>&1; "
        fi
    done
    
    # Optimized system
    run_benchmark "load_all_agents" "Optimized" \
        "$optimized_cmd" \
        $run
    
    # Legacy system
    if [[ "$RUN_LEGACY" == "true" ]]; then
        run_benchmark "load_all_agents" "Legacy" \
            "bash -c 'source $LEGACY_SCRIPT && $legacy_cmd'" \
            $run
    fi
    
    # Bridge
    if [[ "$RUN_BRIDGE" == "true" ]]; then
        run_benchmark "load_all_agents" "Bridge" \
            "bash -c 'source $BRIDGE_SCRIPT && $bridge_cmd'" \
            $run
    fi
done

# Benchmark 4: Validate agent memory
for agent in "${AGENTS[@]}"; do
    for run in $(seq 1 $NUM_RUNS); do
        # Optimized system
        run_benchmark "validate_memory_$agent" "Optimized" \
            "$MEMORY_CLI_PATH validate --agent $agent" \
            $run
        
        # Legacy system does not have validation capability
        
        # Bridge
        if [[ "$RUN_BRIDGE" == "true" ]]; then
            run_benchmark "validate_memory_$agent" "Bridge" \
                "bash -c 'source $BRIDGE_SCRIPT && validate_agent_memory $agent'" \
                $run
        fi
    done
done

# Benchmark 5: Check agent status
for run in $(seq 1 $NUM_RUNS); do
    # Optimized system
    run_benchmark "check_status" "Optimized" \
        "$MEMORY_CLI_PATH status" \
        $run
    
    # Legacy system does not have status capability
    
    # Bridge
    if [[ "$RUN_BRIDGE" == "true" ]]; then
        run_benchmark "check_status" "Bridge" \
            "bash -c 'source $BRIDGE_SCRIPT && get_agent_status'" \
            $run
    fi
done

# Create benchmark report
create_benchmark_report

echo -e "${GREEN}Benchmark completed. Results saved to: $BENCHMARK_FILE${NC}"
echo -e "${GREEN}Report generated at: $REPORT_FILE${NC}"