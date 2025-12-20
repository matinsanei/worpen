# Worpen Convert - JSON to YAML Converter CLI

A powerful command-line tool for converting Worpen dynamic routes between JSON and YAML formats.

## Features

✅ **Single File Conversion** - Convert one JSON file to YAML  
✅ **Batch Processing** - Convert entire directories recursively  
✅ **Validation** - Validate YAML route structure and syntax  
✅ **Diff Comparison** - Compare JSON and YAML files semantically  
✅ **Dry Run Mode** - Preview changes before writing files  
✅ **Colored Output** - Beautiful terminal output with emojis  

## Installation

```bash
cd backend/tools
cargo build --release
```

The binary will be located at `target/release/worpen-convert` (or `worpen-convert.exe` on Windows).

## Usage

### Convert Single File

Convert a JSON route to YAML:

```bash
worpen-convert convert --input routes/user.json --output routes/user.yaml
```

Auto-generate output filename (replaces .json with .yaml):

```bash
worpen-convert convert --input routes/user.json
```

Preview conversion without writing (dry run):

```bash
worpen-convert convert --input routes/user.json --dry-run
```

### Batch Conversion

Convert all JSON files in a directory:

```bash
worpen-convert batch --input-dir routes/ --output-dir routes/yaml/
```

Recursive mode (includes subdirectories):

```bash
worpen-convert batch --input-dir routes/ --recursive
```

Default output directory (creates `yaml/` subdirectory):

```bash
worpen-convert batch --input-dir routes/
```

Dry run to preview batch conversion:

```bash
worpen-convert batch --input-dir routes/ --dry-run
```

### Validate YAML Route

Check YAML syntax and route structure:

```bash
worpen-convert validate --file routes/user.yaml
```

Verbose mode (shows all validation checks):

```bash
worpen-convert validate --file routes/user.yaml --verbose
```

**Validation checks:**
- ✓ YAML syntax validity
- ✓ Required fields (path, method, logic, response)
- ✓ Path format (starts with `/`)
- ✓ HTTP method (GET, POST, PUT, DELETE, PATCH)
- ✓ Logic structure (array format)

### Compare Files

Show semantic differences between JSON and YAML:

```bash
worpen-convert diff --json routes/user.json --yaml routes/user.yaml
```

**Output:**
- ✅ Semantically identical - files represent the same data
- 📊 File size comparison
- ⚠️ Differences highlighted if content differs

## Examples

### Example 1: Convert with Preview

```bash
$ worpen-convert convert -i user.json --dry-run

🔄 Converting JSON to YAML...
Input:  user.json
Output: user.yaml

📄 Preview (dry run - not saved):
────────────────────────────────────────────────────────────────────────────────
path: /users/:id
method: GET
logic:
  - variable: user_id
    value: "{{ request.params.id }}"
  - sql:
      query: SELECT * FROM users WHERE id = :user_id
      params:
        user_id: "{{ user_id }}"
    assign: user
response:
  status: 200
  body: "{{ user }}"
────────────────────────────────────────────────────────────────────────────────
```

### Example 2: Batch Convert with Summary

```bash
$ worpen-convert batch -i routes/ -o routes/yaml/ --recursive

🔄 Batch Converting JSON to YAML...
Input directory: routes/
Output directory: routes/yaml/

Found 5 JSON file(s)

Converting user-get.json... ✅
Converting user-create.json... ✅
Converting order-process.json... ✅
Converting auth-login.json... ✅
Converting game-physics.json... ✅

════════════════════════════════════════════════════════
Summary:
  Converted: 5
  Failed:    0
════════════════════════════════════════════════════════
```

### Example 3: Validate Route

```bash
$ worpen-convert validate -f routes/user.yaml --verbose

🔍 Validating YAML route...
File: routes/user.yaml
✅ YAML syntax is valid
  ✓ Field 'path' present
  ✓ Field 'method' present
  ✓ Field 'logic' present
  ✓ Field 'response' present
  ✓ Path format valid
  ✓ HTTP method valid
  ✓ Logic is an array

════════════════════════════════════════════════════════
Validation Summary:
  Passed: 7
  Failed: 0
════════════════════════════════════════════════════════

✅ Route is valid!
```

### Example 4: Compare Files

```bash
$ worpen-convert diff --json routes/user.json --yaml routes/user.yaml

🔍 Comparing JSON and YAML files...
✅ Files are semantically identical!

📊 File Size Comparison:
  JSON: 512 bytes
  YAML: 387 bytes
  YAML is 24% smaller
```

## Command Reference

### `convert` - Single File Conversion

| Flag | Type | Description |
|------|------|-------------|
| `-i, --input` | Path | Input JSON file (required) |
| `-o, --output` | Path | Output YAML file (optional) |
| `-d, --dry-run` | Flag | Preview without writing |

### `batch` - Batch Conversion

| Flag | Type | Description |
|------|------|-------------|
| `-i, --input-dir` | Path | Input directory (required) |
| `-o, --output-dir` | Path | Output directory (optional) |
| `-r, --recursive` | Flag | Process subdirectories |
| `-d, --dry-run` | Flag | Preview without writing |

### `validate` - YAML Validation

| Flag | Type | Description |
|------|------|-------------|
| `-f, --file` | Path | YAML file to validate (required) |
| `-v, --verbose` | Flag | Show all validation checks |

### `diff` - File Comparison

| Flag | Type | Description |
|------|------|-------------|
| `-j, --json` | Path | JSON file path (required) |
| `-y, --yaml` | Path | YAML file path (required) |

## Integration with Worpen

### Workflow 1: Migrate Existing Routes

```bash
# 1. Backup JSON routes
cp -r routes/ routes.backup/

# 2. Convert all routes
worpen-convert batch -i routes/ -o routes/ --recursive

# 3. Validate all YAML routes
for file in routes/*.yaml; do
    worpen-convert validate -f "$file"
done

# 4. Test routes with Worpen server
cargo run -p api
```

### Workflow 2: Verify Conversion Quality

```bash
# 1. Convert with dry run
worpen-convert batch -i routes/ --dry-run

# 2. Actually convert
worpen-convert batch -i routes/

# 3. Compare each file
for json in routes/*.json; do
    yaml="${json%.json}.yaml"
    worpen-convert diff --json "$json" --yaml "$yaml"
done
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error (file not found, parse error, validation failed, etc.) |

## Performance

**Benchmarks:**
- Single file conversion: ~1-2ms
- Batch conversion (100 files): ~150-200ms
- Validation: ~1ms per file

**File size reduction:**
- Typical: 20-40% smaller (YAML vs JSON)
- Complex routes: 30-50% smaller
- Simple routes: 15-25% smaller

## Troubleshooting

### Error: "Invalid JSON"

**Cause:** JSON file has syntax errors  
**Solution:** Check JSON with `jq` or online validator

```bash
jq . routes/user.json
```

### Error: "Invalid YAML"

**Cause:** YAML file has indentation or syntax errors  
**Solution:** Check YAML with `yamllint`

```bash
yamllint routes/user.yaml
```

### Error: "Field 'path' missing"

**Cause:** Route structure incomplete  
**Solution:** Ensure route has all required fields: `path`, `method`, `logic`, `response`

## Tips

1. **Always use dry run first** - Preview changes before writing
2. **Validate after conversion** - Ensure route structure is correct
3. **Use diff to verify** - Confirm semantic equivalence
4. **Backup before batch operations** - Safety first!
5. **Recursive mode with caution** - Make sure you want all subdirectories

## License

MIT License - See main project LICENSE file.

## Support

For issues or questions:
- GitHub Issues: [worpen/issues](https://github.com/worpen/worpen/issues)
- Documentation: [worpen/docs](https://github.com/worpen/worpen/tree/main/documentation)
