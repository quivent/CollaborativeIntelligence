# Agent Implementation Guide: Batch Task Processing

This document provides practical guidelines for all agents to implement batch task processing, reducing workflow interruptions while maintaining comprehensive task tracking.

## Quick Start Implementation

### Step 1: Create your complete task list at the beginning

```javascript
// Create complete task list with a single TodoWrite call
function initializeTasks() {
  TodoWrite([
    {id: "1", content: "First task", status: "pending", priority: "high"},
    {id: "2", content: "Second task", status: "pending", priority: "medium"},
    {id: "3", content: "Third task", status: "pending", priority: "medium"},
    {id: "4", content: "Fourth task", status: "pending", priority: "low"},
    // Add all tasks at once
  ]);
  
  // Return immediately to processing without waiting for user input
  return "Tasks initialized - proceeding with implementation";
}
```

### Step 2: Track progress virtually while working

```javascript
// Create a virtual tracking structure (no TodoWrite calls)
const virtualTaskList = [
  {id: "1", content: "First task", status: "in_progress", priority: "high"},
  {id: "2", content: "Second task", status: "pending", priority: "medium"},
  {id: "3", content: "Third task", status: "pending", priority: "medium"},
  {id: "4", content: "Fourth task", status: "pending", priority: "low"},
];

// Later, update your virtual tracking as you progress
virtualTaskList[0].status = "completed";
virtualTaskList[1].status = "in_progress";

// Do not call TodoWrite after each status change
```

### Step 3: Update CLI only at major milestones

```javascript
// After completing significant work, synchronize with CLI
function updateCliAtMilestone() {
  TodoWrite(virtualTaskList);
  
  // Continue working immediately without waiting for confirmation
  return "CLI updated at milestone - continuing implementation";
}
```

### Step 4: Final update at completion

```javascript
// When all tasks are complete, perform final synchronization
function finalizeTaskProcessing() {
  // Ensure all tasks are marked completed in virtual tracking
  virtualTaskList.forEach(task => task.status = "completed");
  
  // Synchronize with CLI
  TodoWrite(virtualTaskList);
  
  return "All tasks completed successfully";
}
```

## Best Practices

### 1. Minimize CLI Integration Points

- Reduce TodoWrite calls to 2-3 per complete workflow
- Structure your implementation to batch multiple tasks between updates
- Avoid individual task status updates via TodoWrite/TodoRead

### 2. Strategic Update Timing

Only update the CLI at these strategic points:
- Initial task list creation
- After completing 30-50% of tasks (major milestone)
- Upon full completion of all tasks
- When explicitly asked for progress by user

### 3. User Communication

Keep the user informed despite fewer CLI updates:
- Verbally communicate progress in your responses
- Use clear section headers to indicate task transitions
- Provide concise summaries of completed work
- Use numbered lists to show progress through sequences

### 4. Virtual Tracking Methods

Choose the tracking method that best fits your workflow:
- Memory variables for simple, sequential tasks
- Internal data structures for complex, interdependent tasks
- Temporary files for very large task sets
- Comments in code when modifying implementation files

### 5. Handling Interdependent Tasks

For tasks with dependencies:
- Pre-check all dependencies before starting any tasks
- Group tasks by dependency chains
- Process each chain continuously before updating CLI
- Have contingency plans for unexpected blockers

## Implementation Template

```javascript
// Complete implementation template

// 1. INITIALIZATION PHASE
// Create full task list with single CLI interaction
function initializeTaskWorkflow() {
  // Define all tasks
  const allTasks = [
    {id: "1", content: "Task description 1", priority: "high"},
    {id: "2", content: "Task description 2", priority: "high"},
    {id: "3", content: "Task description 3", priority: "medium"},
    {id: "4", content: "Task description 4", priority: "medium"},
    {id: "5", content: "Task description 5", priority: "low"},
  ];
  
  // Initialize in CLI with single call
  TodoWrite(allTasks.map(task => ({...task, status: "pending"})));
  
  // Create virtual tracking copy
  const virtualTasks = allTasks.map(task => ({...task, status: "pending"}));
  
  return {
    message: "Initialized task workflow with " + allTasks.length + " tasks",
    tasks: virtualTasks
  };
}

// 2. PROCESSING PHASE
// Process tasks without CLI updates
function processTasks(virtualTasks) {
  // Process first task
  virtualTasks[0].status = "in_progress";
  // Code to implement task 1...
  virtualTasks[0].status = "completed";
  
  // Process second task
  virtualTasks[1].status = "in_progress";
  // Code to implement task 2...
  virtualTasks[1].status = "completed";
  
  // MILESTONE: Update CLI at 40% completion
  TodoWrite(virtualTasks);
  
  // Continue processing remaining tasks
  for (let i = 2; i < virtualTasks.length; i++) {
    virtualTasks[i].status = "in_progress";
    // Code to implement this task...
    virtualTasks[i].status = "completed";
  }
  
  return virtualTasks;
}

// 3. COMPLETION PHASE
// Finalize and report completion
function completeTaskWorkflow(virtualTasks) {
  // Ensure all tasks marked completed
  virtualTasks = virtualTasks.map(task => ({...task, status: "completed"}));
  
  // Final CLI update
  TodoWrite(virtualTasks);
  
  return "All " + virtualTasks.length + " tasks completed successfully";
}

// Main execution flow
const workflow = initializeTaskWorkflow();
const processedTasks = processTasks(workflow.tasks);
const result = completeTaskWorkflow(processedTasks);
```

## Adapting to Different Scenarios

### For Short Task Lists (1-3 tasks)
- Consider using just 2 CLI updates: initial and final
- Continue to track virtually between updates

### For Long Task Lists (10+ tasks)
- Add more strategic milestones (every 5 tasks)
- Consider grouping by functional area or dependency
- Provide more verbose progress communication to user

### For Highly Complex Workflows
- Create subtask groupings with their own virtual tracking
- Use hierarchical task organization
- Consider creating task visualization in documentation

## Troubleshooting

### If You Need to Change Task Order
- Adjust only in your virtual tracking
- Do not update CLI until next scheduled milestone
- Document the reordering in your workflow comments

### If You Encounter Blockers
- Mark blocked tasks in virtual tracking
- Proceed with non-blocked tasks
- Update CLI only at scheduled milestones
- Communicate blocking issues in your responses

### If User Requests Progress Update
- Provide verbal summary based on virtual tracking
- Update CLI only if explicitly requested
- Continue processing without waiting for confirmation

## Summary

By implementing these batch processing patterns, you will:
1. Minimize workflow interruptions
2. Reduce required user intervention
3. Maintain comprehensive task tracking
4. Align with CLAUDE.md directives for continuous processing
5. Improve overall agent efficiency and autonomy

For complete technical implementation details, see [BATCH_PROCESSING_IMPLEMENTATION.md](./BATCH_PROCESSING_IMPLEMENTATION.md).