```
+-------------------------+        +-------------------------+
|                         |        |                         |
|    User Request         |        |    Claude API           |
|    (file operations)    |        |                         |
|                         |        |                         |
+-----------+-------------+        +-------------+-----------+
            |                                    ^
            |                                    |
            v                                    |
+-------------------------+        +-------------+-----------+
|                         |        |                         |
|    Claude Code CLI      +------->+    Cache Layer          |
|                         |        |                         |
|                         |<-------+                         |
+-------------------------+        +-------------+-----------+
                                              |
                                   +----------+-----------+
                                   |                      |
                             +-----v------+        +------v-----+
                             |            |        |            |
                             | In-Memory  |        | Persistent |
                             | Cache      |        | Cache      |
                             |            |        |            |
                             +------------+        +------------+

```

The Claude Code CLI intercepts file operations and checks:

1. **Read operation (View tool):**
   - If file exists in cache and is valid, return cached content
   - If not in cache or invalidated, fetch from filesystem and update cache

2. **Write operation (Edit/Replace tools):**
   - Write to filesystem
   - Update cache with new content

3. **Cache validation:**
   - Check file modification time against cache timestamp
   - If file was modified externally, invalidate cache entry
   - Periodically clean up expired cache entries

This architecture reduces API calls by eliminating redundant file reads, while ensuring cache consistency.