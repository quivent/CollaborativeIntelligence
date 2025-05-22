# Progress Bar Header Templates

## Template Collection for Specification Documents

### Template 1: Emoji Circle Progress
**Source**: `docs/system-design/documentation-linting-report.md`

```markdown
**Progress**: 🟢🟢🟢🟢🟢🟢🟢🟢🟢🟢🟢🟢🟢🟢🟢🟢🟢🟢🟢🟢 100%

### Phase Progress
| Phase | Status | Progress | Count |
|-------|--------|----------|-------|
| **Phase 1**: Documentation Analysis | ✅ Complete | 🟢🟢🟢🟢🟢🟢🟢🟢🟢🟢 100% | 42/42 |
| **Phase 2**: Implementation | ✅ Complete | 🟢🟢🟢🟢🟢🟢 100% | 6/6 |
```

### Template 2: Emoji Circle with Status Indicators
**Source**: Current migration spec format

```markdown
## Progress Overview

**Overall Progress**: ⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪ 0% (0/38)

### Phase Progress
| Phase | Status | Progress | Count |
|-------|--------|----------|-------|
| **Phase 1**: Agent and System Documentation | 🔴 Not Started | ⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪ 0% | 0/13 |
| **Phase 2**: Protocol and Configuration Documentation | 🔴 Not Started | ⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪⚪ 0% | 0/12 |

### Status Legend
- 🟢 **Completed** - All files migrated and validated
- 🟡 **In Progress** - Migration started but not complete
- 🔴 **Not Started** - Phase not yet begun
```

### Template 3: Rectangular Block Progress
**Source**: `interfaces/CIA/VISUAL_DESIGN_SYSTEM.md`

```markdown
## Progress Overview

**Overall Progress**: [□□□□□□□□□□□□□□□□□□□□] 0% (0/38)

### Phase Progress
| Phase | Status | Progress | Count |
|-------|--------|----------|-------|
| **Phase 1**: Agent and System Documentation | 🔴 Not Started | [□□□□□□□□□□□□□] 0% | 0/13 |
| **Phase 2**: Protocol and Configuration Documentation | 🔴 Not Started | [□□□□□□□□□□□□] 0% | 0/12 |

### Status Legend
- **Progress Bars**: [■] = Completed, [□] = Remaining
```

### Template 4: Advanced Hash Progress (CLI Style)
**Source**: `interfaces/CIA/VISUAL_DESIGN_SYSTEM.md`

```markdown
**Overall Progress**: [####################] 100% Complete
**Phase 1 Progress**: [##########----------] 50% (5/10)
**Phase 2 Progress**: [####----------------] 20% (2/10)

### Alternative CLI Format
[■■■■■■■■■■□□□□□□□□□□] 50%
[████████████████████] 100% Complete
```

### Template 5: Multi-Character Block Progress
**Source**: Various files - Enhanced format

```markdown
## Progress Tracking

**Overall**: ████████████░░░░░░░░░░░░ 60% (24/40)

### Phase Breakdown
- **Phase 1**: ████████████████████ 100% (20/20) ✅ Complete
- **Phase 2**: ████████░░░░░░░░░░░░ 40% (4/10) 🔄 In Progress  
- **Phase 3**: ░░░░░░░░░░░░░░░░░░░░ 0% (0/10) ⏸️ Pending

### Legend
- █ Completed blocks
- ░ Remaining blocks
- ✅ Phase complete
- 🔄 Phase in progress
- ⏸️ Phase pending
```

### Template 6: Gradient Progress Style
**Source**: Enhanced design

```markdown
## Implementation Progress

**Total**: ▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒░░ 65% (13/20)

| Component | Progress | Status |
|-----------|----------|---------|
| Core System | ▓▓▓▓▓▓▓▓▓▓ 100% | ✅ Complete |
| Interface | ▓▓▓▓▓▓▒▒▒▒ 60% | 🔄 Active |
| Testing | ▓▓▒▒▒▒░░░░ 30% | 📋 Planned |

### Character Legend
- ▓ Solid completion
- ▒ Partial completion  
- ░ Not started
```