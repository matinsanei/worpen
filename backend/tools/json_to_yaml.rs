use clap::{Parser, Subcommand};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use colored::Colorize;

#[derive(Parser)]
#[command(name = "worpen-convert")]
#[command(about = "Convert dynamic routes between JSON and YAML formats", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert a single file from JSON to YAML
    Convert {
        /// Input JSON file path
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output YAML file path (optional, defaults to input with .yaml extension)
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Dry run - preview conversion without writing file
        #[arg(short, long)]
        dry_run: bool,
    },
    
    /// Batch convert all JSON files in a directory
    Batch {
        /// Input directory containing JSON files
        #[arg(short, long)]
        input_dir: PathBuf,
        
        /// Output directory for YAML files (optional, defaults to input_dir/yaml/)
        #[arg(short, long)]
        output_dir: Option<PathBuf>,
        
        /// Recursive - process subdirectories
        #[arg(short, long)]
        recursive: bool,
        
        /// Dry run - preview conversions without writing files
        #[arg(short, long)]
        dry_run: bool,
    },
    
    /// Validate a YAML route file
    Validate {
        /// YAML file path to validate
        #[arg(short, long)]
        file: PathBuf,
        
        /// Verbose output showing all checks
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Show difference between JSON and YAML versions
    Diff {
        /// JSON file path
        #[arg(short, long)]
        json: PathBuf,
        
        /// YAML file path
        #[arg(short, long)]
        yaml: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Convert { input, output, dry_run } => {
            handle_convert(input, output.as_ref(), *dry_run);
        }
        Commands::Batch { input_dir, output_dir, recursive, dry_run } => {
            handle_batch(input_dir, output_dir.as_ref(), *recursive, *dry_run);
        }
        Commands::Validate { file, verbose } => {
            handle_validate(file, *verbose);
        }
        Commands::Diff { json, yaml } => {
            handle_diff(json, yaml);
        }
    }
}

fn handle_convert(input: &Path, output: Option<&PathBuf>, dry_run: bool) {
    println!("{}", "🔄 Converting JSON to YAML...".cyan().bold());
    println!("Input:  {}", input.display());
    
    // Read JSON file
    let json_content = match fs::read_to_string(input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} Failed to read file: {}", "❌".red(), e);
            process::exit(1);
        }
    };
    
    // Parse JSON
    let json_value: JsonValue = match serde_json::from_str(&json_content) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("{} Invalid JSON: {}", "❌".red(), e);
            process::exit(1);
        }
    };
    
    // Convert to YAML
    let yaml_content = match serde_yaml::to_string(&json_value) {
        Ok(yaml) => yaml,
        Err(e) => {
            eprintln!("{} Conversion failed: {}", "❌".red(), e);
            process::exit(1);
        }
    };
    
    // Determine output path
    let output_path = match output {
        Some(path) => path.clone(),
        None => {
            let mut path = input.to_path_buf();
            path.set_extension("yaml");
            path
        }
    };
    
    println!("Output: {}", output_path.display());
    
    if dry_run {
        println!("\n{}", "📄 Preview (dry run - not saved):".yellow().bold());
        println!("{}", "─".repeat(80).yellow());
        println!("{}", yaml_content);
        println!("{}", "─".repeat(80).yellow());
    } else {
        // Write YAML file
        match fs::write(&output_path, yaml_content) {
            Ok(_) => {
                println!("{} Conversion successful!", "✅".green());
                
                // Show file size comparison
                if let (Ok(json_meta), Ok(yaml_meta)) = (
                    fs::metadata(input),
                    fs::metadata(&output_path)
                ) {
                    let json_size = json_meta.len();
                    let yaml_size = yaml_meta.len();
                    let reduction = ((json_size as f64 - yaml_size as f64) / json_size as f64 * 100.0) as i32;
                    
                    println!("\n{}", "📊 File Size Comparison:".cyan());
                    println!("  JSON: {} bytes", json_size);
                    println!("  YAML: {} bytes", yaml_size);
                    
                    if reduction > 0 {
                        println!("  Reduction: {}% smaller", reduction.to_string().green());
                    } else {
                        println!("  Change: {}% larger", (-reduction).to_string().yellow());
                    }
                }
            }
            Err(e) => {
                eprintln!("{} Failed to write file: {}", "❌".red(), e);
                process::exit(1);
            }
        }
    }
}

fn handle_batch(input_dir: &Path, output_dir: Option<&PathBuf>, recursive: bool, dry_run: bool) {
    println!("{}", "🔄 Batch Converting JSON to YAML...".cyan().bold());
    println!("Input directory: {}", input_dir.display());
    
    if !input_dir.is_dir() {
        eprintln!("{} Input path is not a directory", "❌".red());
        process::exit(1);
    }
    
    // Determine output directory
    let out_dir = match output_dir {
        Some(path) => path.clone(),
        None => {
            let mut path = input_dir.to_path_buf();
            path.push("yaml");
            path
        }
    };
    
    println!("Output directory: {}", out_dir.display());
    
    // Create output directory if not dry run
    if !dry_run {
        if let Err(e) = fs::create_dir_all(&out_dir) {
            eprintln!("{} Failed to create output directory: {}", "❌".red(), e);
            process::exit(1);
        }
    }
    
    // Find JSON files
    let json_files = find_json_files(input_dir, recursive);
    
    if json_files.is_empty() {
        println!("{} No JSON files found", "⚠️".yellow());
        return;
    }
    
    println!("\nFound {} JSON file(s)\n", json_files.len().to_string().green().bold());
    
    let mut converted = 0;
    let mut failed = 0;
    
    for json_file in json_files {
        let file_name = json_file.file_name().unwrap().to_string_lossy();
        print!("Converting {}... ", file_name.cyan());
        
        let output_file = out_dir.join(json_file.file_name().unwrap()).with_extension("yaml");
        
        match convert_file(&json_file, &output_file, dry_run) {
            Ok(_) => {
                println!("{}", "✅".green());
                converted += 1;
            }
            Err(e) => {
                println!("{} {}", "❌".red(), e);
                failed += 1;
            }
        }
    }
    
    println!("\n{}", "═".repeat(60).cyan());
    println!("{}", "Summary:".cyan().bold());
    println!("  Converted: {}", converted.to_string().green());
    println!("  Failed:    {}", failed.to_string().red());
    println!("{}", "═".repeat(60).cyan());
    
    if dry_run {
        println!("\n{}", "💡 This was a dry run. Use without --dry-run to save files.".yellow());
    }
}

fn find_json_files(dir: &Path, recursive: bool) -> Vec<PathBuf> {
    let mut files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                files.push(path);
            } else if recursive && path.is_dir() {
                files.extend(find_json_files(&path, recursive));
            }
        }
    }
    
    files
}

fn convert_file(input: &Path, output: &Path, dry_run: bool) -> Result<(), String> {
    // Read JSON
    let json_content = fs::read_to_string(input)
        .map_err(|e| format!("Read error: {}", e))?;
    
    // Parse JSON
    let json_value: JsonValue = serde_json::from_str(&json_content)
        .map_err(|e| format!("Parse error: {}", e))?;
    
    // Convert to YAML
    let yaml_content = serde_yaml::to_string(&json_value)
        .map_err(|e| format!("Conversion error: {}", e))?;
    
    // Write YAML (if not dry run)
    if !dry_run {
        fs::write(output, yaml_content)
            .map_err(|e| format!("Write error: {}", e))?;
    }
    
    Ok(())
}

fn handle_validate(file: &Path, verbose: bool) {
    println!("{}", "🔍 Validating YAML route...".cyan().bold());
    println!("File: {}", file.display());
    
    // Read YAML file
    let yaml_content = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} Failed to read file: {}", "❌".red(), e);
            process::exit(1);
        }
    };
    
    // Parse YAML
    let yaml_value: YamlValue = match serde_yaml::from_str(&yaml_content) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("{} Invalid YAML: {}", "❌".red(), e);
            process::exit(1);
        }
    };
    
    println!("{} YAML syntax is valid", "✅".green());
    
    // Validate route structure
    let mut checks_passed = 0;
    let mut checks_failed = 0;
    
    if let YamlValue::Mapping(map) = &yaml_value {
        // Check for required fields
        let required_fields = vec!["path", "method", "logic", "response"];
        
        for field in &required_fields {
            if map.contains_key(&YamlValue::String(field.to_string())) {
                if verbose {
                    println!("  {} Field '{}' present", "✓".green(), field);
                }
                checks_passed += 1;
            } else {
                println!("  {} Field '{}' missing", "✗".red(), field);
                checks_failed += 1;
            }
        }
        
        // Check path format
        if let Some(YamlValue::String(path)) = map.get(&YamlValue::String("path".to_string())) {
            if path.starts_with('/') {
                if verbose {
                    println!("  {} Path format valid", "✓".green());
                }
                checks_passed += 1;
            } else {
                println!("  {} Path should start with '/'", "✗".yellow());
                checks_failed += 1;
            }
        }
        
        // Check method
        if let Some(YamlValue::String(method)) = map.get(&YamlValue::String("method".to_string())) {
            let valid_methods = vec!["GET", "POST", "PUT", "DELETE", "PATCH"];
            if valid_methods.contains(&method.as_str()) {
                if verbose {
                    println!("  {} HTTP method valid", "✓".green());
                }
                checks_passed += 1;
            } else {
                println!("  {} Invalid HTTP method: {}", "✗".red(), method);
                checks_failed += 1;
            }
        }
        
        // Check logic is array
        if let Some(YamlValue::Sequence(_)) = map.get(&YamlValue::String("logic".to_string())) {
            if verbose {
                println!("  {} Logic is an array", "✓".green());
            }
            checks_passed += 1;
        } else if map.contains_key(&YamlValue::String("logic".to_string())) {
            println!("  {} Logic should be an array", "✗".red());
            checks_failed += 1;
        }
    } else {
        eprintln!("{} Route should be a mapping/object", "❌".red());
        process::exit(1);
    }
    
    println!("\n{}", "═".repeat(60).cyan());
    println!("{}", "Validation Summary:".cyan().bold());
    println!("  Passed: {}", checks_passed.to_string().green());
    println!("  Failed: {}", checks_failed.to_string().red());
    println!("{}", "═".repeat(60).cyan());
    
    if checks_failed == 0 {
        println!("\n{} Route is valid!", "✅".green().bold());
    } else {
        println!("\n{} Route has {} issue(s)", "⚠️".yellow(), checks_failed);
        process::exit(1);
    }
}

fn handle_diff(json_path: &Path, yaml_path: &Path) {
    println!("{}", "🔍 Comparing JSON and YAML files...".cyan().bold());
    
    // Read JSON
    let json_content = match fs::read_to_string(json_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} Failed to read JSON file: {}", "❌".red(), e);
            process::exit(1);
        }
    };
    
    // Read YAML
    let yaml_content = match fs::read_to_string(yaml_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{} Failed to read YAML file: {}", "❌".red(), e);
            process::exit(1);
        }
    };
    
    // Parse both
    let json_value: JsonValue = match serde_json::from_str(&json_content) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("{} Invalid JSON: {}", "❌".red(), e);
            process::exit(1);
        }
    };
    
    let yaml_value: YamlValue = match serde_yaml::from_str(&yaml_content) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("{} Invalid YAML: {}", "❌".red(), e);
            process::exit(1);
        }
    };
    
    // Convert both to JSON for comparison
    let json_normalized = serde_json::to_string_pretty(&json_value).unwrap();
    let yaml_as_json = serde_json::to_string_pretty(&yaml_value).unwrap();
    
    if json_normalized == yaml_as_json {
        println!("{} Files are semantically identical!", "✅".green().bold());
        
        // Show file sizes
        if let (Ok(json_meta), Ok(yaml_meta)) = (
            fs::metadata(json_path),
            fs::metadata(yaml_path)
        ) {
            let json_size = json_meta.len();
            let yaml_size = yaml_meta.len();
            let reduction = ((json_size as f64 - yaml_size as f64) / json_size as f64 * 100.0) as i32;
            
            println!("\n{}", "📊 File Size Comparison:".cyan());
            println!("  JSON: {} bytes", json_size);
            println!("  YAML: {} bytes", yaml_size);
            
            if reduction > 0 {
                println!("  YAML is {}% smaller", reduction.to_string().green());
            } else {
                println!("  YAML is {}% larger", (-reduction).to_string().yellow());
            }
        }
    } else {
        println!("{} Files have semantic differences!", "⚠️".yellow().bold());
        println!("\nNormalized JSON representation:");
        println!("{}", "─".repeat(80).yellow());
        println!("{}", json_normalized);
        println!("{}", "─".repeat(80).yellow());
        println!("\nYAML as JSON:");
        println!("{}", "─".repeat(80).yellow());
        println!("{}", yaml_as_json);
        println!("{}", "─".repeat(80).yellow());
    }
}
