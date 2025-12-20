# 📚 Worpen Documentation Index

Complete guide to building APIs with YAML/JSON and Worpen Dynamic Routes Engine.

---

## 🎯 Quick Navigation

### 🚀 Getting Started (Start Here!)

1. **[Introduction](01-introduction.md)** - What is Worpen? Core concepts and philosophy
2. **[Basic Structure](02-basic-structure.md)** - Anatomy of a route: name, path, parameters, logic
3. **[Variables Basics](03-variables-basics.md)** - Using `Set` and `Get` operations, `{{variable}}` syntax

### 🔧 Core Features

4. **[Control Flow](04-control-flow.md)** - `If/Else` conditionals, comparison operators
5. **[Loops](05-loops.md)** - `ForEach`, `While`, `Until` with break/continue
6. **[Functions](06-functions.md)** - `CallFunction` for reusable logic blocks
7. **[Math Operations](07-math-operations.md)** - `sum`, `divide`, `multiply`, `round`, `abs`, `max`, `min`
8. **[String Operations](08-string-operations.md)** - `upper`, `lower`, `trim`, `replace`, `concat`
9. **[Date Operations](09-date-operations.md)** - Date parsing, formatting, calculations

### 🎓 Advanced Topics

10. **[Error Handling](10-error-handling.md)** - Try/catch patterns, validation strategies
11. **[Parallel Execution](11-parallel-execution.md)** - Concurrent operations for performance
12. **[Complete Examples](12-complete-examples.md)** - Real-world use cases and patterns

### 🆕 YAML Syntax (NEW!)

13. **[YAML Syntax Guide](13-yaml-syntax.md)** ⭐ **890 lines** - Complete YAML reference
    - Tagged syntax (!Return, !Set, !If, etc.)
    - All operations with examples
    - Best practices for readability
    
14. **[Expression Syntax](14-expressions.md)** ⭐ **750 lines** - Filters, pipes, and helpers
    - 30+ pipe filters (`| upper`, `| sum`, `| map`, etc.)
    - 25+ helper functions (`now()`, `uuid()`, `hash()`, etc.)
    - Advanced patterns and combinations
    
15. **[Migration Guide](15-migration-guide.md)** ⭐ **650 lines** - JSON to YAML conversion
    - Step-by-step migration process
    - Side-by-side comparisons
    - Common patterns and gotchas
    
16. **[Best Practices](16-best-practices.md)** ⭐ **500 lines** - Patterns and anti-patterns
    - Performance optimization
    - Security considerations
    - Maintainability tips

---

## 📊 Documentation Stats

- **Total Pages**: 16 comprehensive guides
- **Total Lines**: 6,500+ lines of documentation
- **Examples**: 100+ working code samples
- **Coverage**: Every operation, every feature

---

## 🎯 Learning Paths

### Path 1: Quick Start (30 minutes)
1. Read [Introduction](01-introduction.md)
2. Read [Basic Structure](02-basic-structure.md)
3. Try examples from [YAML Syntax Guide](13-yaml-syntax.md)
4. Deploy your first route!

### Path 2: Complete Course (2-3 hours)
1. **Basics**: Pages 1-3 (Introduction, Structure, Variables)
2. **Core**: Pages 4-6 (Control Flow, Loops, Functions)
3. **Operations**: Pages 7-9 (Math, String, Date)
4. **Advanced**: Pages 10-12 (Error Handling, Parallel, Examples)
5. **YAML**: Pages 13-16 (Syntax, Expressions, Migration, Best Practices)

### Path 3: Reference (As Needed)
- Bookmark [Expression Syntax](14-expressions.md) for filter/helper reference
- Use [Complete Examples](12-complete-examples.md) for patterns
- Check [Best Practices](16-best-practices.md) before production deployment

---

## 🔍 Find What You Need

### Common Tasks

| I want to... | Check this guide |
|:-------------|:-----------------|
| Define my first route | [Basic Structure](02-basic-structure.md) |
| Use variables | [Variables Basics](03-variables-basics.md) |
| Add conditional logic | [Control Flow](04-control-flow.md) |
| Loop through data | [Loops](05-loops.md) |
| Do math calculations | [Math Operations](07-math-operations.md) |
| Manipulate strings | [String Operations](08-string-operations.md) |
| Query database | [Basic Structure](02-basic-structure.md) - QueryDb section |
| Use YAML instead of JSON | [YAML Syntax Guide](13-yaml-syntax.md) |
| Convert JSON to YAML | [Migration Guide](15-migration-guide.md) |
| Use filters/pipes | [Expression Syntax](14-expressions.md) |
| See real examples | [Complete Examples](12-complete-examples.md) |
| Optimize performance | [Best Practices](16-best-practices.md) |

### By Operation Type

| Operation | Documentation |
|:----------|:--------------|
| `Set`, `Get` | [Variables Basics](03-variables-basics.md) |
| `If`, `Else` | [Control Flow](04-control-flow.md) |
| `ForEach`, `While`, `Until` | [Loops](05-loops.md) |
| `CallFunction` | [Functions](06-functions.md) |
| `MathOp` | [Math Operations](07-math-operations.md) |
| `StringOp` | [String Operations](08-string-operations.md) |
| `DateOp` | [Date Operations](09-date-operations.md) |
| `QueryDb` | [Basic Structure](02-basic-structure.md) |
| `Return` | [Basic Structure](02-basic-structure.md) |
| `Parallel` | [Parallel Execution](11-parallel-execution.md) |

---

## 💡 Tips for Success

### 1. Start Simple
Begin with basic routes and gradually add complexity. Every complex route is just simple operations combined.

### 2. Use YAML
YAML is more readable and requires 50-70% less code than JSON. See [YAML Syntax Guide](13-yaml-syntax.md).

### 3. Leverage Expressions
Master the expression syntax with filters and helpers. One line of `{{prices | sum | divide(count)}}` replaces multiple operations. See [Expression Syntax](14-expressions.md).

### 4. Test Incrementally
Add operations one at a time and test after each addition. Use the test suite patterns from [Complete Examples](12-complete-examples.md).

### 5. Follow Best Practices
Read [Best Practices](16-best-practices.md) before deploying to production. Security and performance matter!

---

## 🆘 Getting Help

### Documentation Issues
- **Unclear?** [Open an issue](https://github.com/matinsanei/worpen/issues) with "docs:" prefix
- **Missing info?** Suggest additions via pull request
- **Found errors?** We appreciate corrections!

### Technical Support
- **Bug reports**: [GitHub Issues](https://github.com/matinsanei/worpen/issues)
- **Feature requests**: [GitHub Discussions](https://github.com/matinsanei/worpen/discussions)
- **Questions**: Check [Complete Examples](12-complete-examples.md) first

---

## 📝 Contributing to Docs

Help us improve the documentation:

1. **Fix typos**: Small PRs welcome!
2. **Add examples**: Share your working routes
3. **Improve clarity**: Suggest better explanations
4. **Translate**: Help make docs multilingual

---

## 🎓 Other Resources

- 🏠 **[Main README](../README.md)** - Project overview and quick start
- 🗺️ **[Roadmap](../ROADMAP.md)** - Development timeline and future plans
- 🔧 **[Backend README](../backend/README.md)** - Technical architecture
- 📊 **[Phase Summaries](../backend/)** - Development logs and decisions
- 🛠️ **[CLI Tools](../backend/tools/)** - JSON to YAML converter

---

<div align="center">

**Happy Route Building! 🚀**

*Questions? Check [Complete Examples](12-complete-examples.md) or [open an issue](https://github.com/matinsanei/worpen/issues)*

[← Back to Main README](../README.md)

</div>
