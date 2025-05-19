# Knowledge Distribution Workflow Scheduling

This guide explains how to set up automated scheduling for the Knowledge Distribution Workflow to ensure regular processing and distribution of knowledge throughout the Collaborative Intelligence system.

## Prerequisites

- The Knowledge Distribution Architecture is fully implemented
- All scripts are properly installed in `/scripts/knowledge/`
- You have administrative access to set up scheduled tasks

## Scheduling Options

### Using Cron (macOS/Linux)

1. Open your crontab for editing:
   ```bash
   crontab -e
   ```

2. Add a line to schedule the workflow:
   ```
   # Run Knowledge Distribution Workflow every day at 2 AM
   0 2 * * * /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/scripts/knowledge/scheduled_workflow.sh
   ```

   Alternative schedules:
   ```
   # Run every Monday at 3 AM
   0 3 * * 1 /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/scripts/knowledge/scheduled_workflow.sh
   
   # Run every 6 hours
   0 */6 * * * /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/scripts/knowledge/scheduled_workflow.sh
   ```

3. Save and exit the editor

### Using Launchd (macOS)

1. Create a launchd plist file at `~/Library/LaunchAgents/com.collaborativeintelligence.knowledge-workflow.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.collaborativeintelligence.knowledge-workflow</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/scripts/knowledge/scheduled_workflow.sh</string>
    </array>
    <key>StartCalendarInterval</key>
    <dict>
        <key>Hour</key>
        <integer>2</integer>
        <key>Minute</key>
        <integer>0</integer>
    </dict>
    <key>StandardOutPath</key>
    <string>/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/KnowledgeRegistry/logs/launchd_stdout.log</string>
    <key>StandardErrorPath</key>
    <string>/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/KnowledgeRegistry/logs/launchd_stderr.log</string>
</dict>
</plist>
```

2. Load the plist file:
```bash
launchctl load ~/Library/LaunchAgents/com.collaborativeintelligence.knowledge-workflow.plist
```

### Using Task Scheduler (Windows)

1. Open Task Scheduler
2. Click "Create Basic Task"
3. Enter a name and description for the task
4. Set the trigger (daily at 2 AM)
5. For the action, select "Start a program"
6. Browse to select the scheduled_workflow.sh script
7. Complete the wizard to save the task

## Monitoring

Logs for each workflow run are stored in:
```
/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/KnowledgeRegistry/logs/
```

The latest log is always linked to:
```
/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/KnowledgeRegistry/logs/latest_workflow.log
```

## Manual Execution

To run the workflow manually:

```bash
cd /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence
./scripts/knowledge/scheduled_workflow.sh
```

Or to run with custom parameters:

```bash
cd /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence
./scripts/knowledge/knowledge_workflow.py --threshold 0.8
```

## Customization

The workflow behavior can be customized by editing the parameters in the `scheduled_workflow.sh` script:

- `THRESHOLD`: Minimum relevance score for content identification (default: 0.7)
- `LOG_DIR`: Location for workflow logs

## Troubleshooting

If the scheduled workflow fails:

1. Check the log files in the `logs` directory
2. Verify that all scripts have execute permissions
3. Ensure the paths in the scheduling configuration are correct
4. Try running the workflow manually to identify any issues

---

Last Updated: 2025-05-18