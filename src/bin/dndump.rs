//! `dndump` — inspect a .NET PE file from the command line.
//!
//! Mirrors the Python `dndump.py` example shipped with `malwarefrank/dnfile`
//! but with a `capa`-style tabular layout.

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use clap::Parser;
use dnfile::DnPe;
use prettytable::{Cell, Row, Table, format, row};

/// Inspect a .NET PE file: CLR header, metadata streams, tables, methods.
#[derive(Parser, Debug)]
#[command(
    name = "dndump",
    version,
    about = "Inspect a .NET PE file (CLR header, streams, metadata tables, methods).",
    long_about = None,
)]
struct Args {
    /// Path to the .NET PE file.
    file: PathBuf,

    /// Dump all decoded user strings (#US heap).
    #[arg(long)]
    strings: bool,

    /// Print up to this many methods. Use 0 to disable the methods table.
    #[arg(long, default_value_t = 20)]
    methods: usize,

    /// Emit the full structured result as JSON instead of tables.
    #[arg(long)]
    json: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();
    match run(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let path = args.file.canonicalize().unwrap_or(args.file.clone());
    let pe = DnPe::new(path.to_str().ok_or("file path is not valid UTF-8")?)?;

    if args.json {
        let s = serde_json::to_string_pretty(&pe)?;
        println!("{s}");
        return Ok(());
    }

    let clr = pe.net()?;

    print_file_properties(&path)?;
    print_clr_header(clr);
    print_streams(clr);
    print_tables(clr);

    if args.methods > 0 {
        print_methods(clr, args.methods);
    }

    if args.strings {
        print_user_strings(clr);
    }

    println!(
        "\nTime taken (seconds): {:.6}s",
        start.elapsed().as_secs_f64()
    );
    Ok(())
}

fn print_file_properties(path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let meta = std::fs::metadata(path)?;
    let mut table = capa_style_table();
    table.set_titles(row![bH2c => "File Properties"]);
    table.add_row(row!["path", path.display()]);
    table.add_row(row!["size", format!("{} bytes", meta.len())]);
    println!();
    table.printstd();
    Ok(())
}

fn print_clr_header(clr: &dnfile::ClrData) {
    let mut table = capa_style_table();
    table.set_titles(row![bH2c => "CLR Header"]);

    let flags: Vec<String> = clr.flags.iter().map(|f| format!("{f:?}")).collect();
    let flags_cell = if flags.is_empty() {
        "(none)".to_string()
    } else {
        flags.join(", ")
    };
    table.add_row(row!["flags", flags_cell]);
    table.add_row(row![
        "il-only",
        clr.flags.contains(&dnfile::ClrHeaderFlags::IlOnly)
    ]);
    table.add_row(row![
        "32-bit-required",
        clr.flags.contains(&dnfile::ClrHeaderFlags::BitRequired32)
    ]);
    table.add_row(row![
        "strong-name-signed",
        clr.flags
            .contains(&dnfile::ClrHeaderFlags::StrongNamesSigned)
    ]);
    table.add_row(row!["function-count", clr.functions().len()]);
    println!();
    table.printstd();
}

fn print_streams(clr: &dnfile::ClrData) {
    let mut table = capa_style_table();
    table.set_titles(row![bH3c => "Streams"]);
    table.add_row(row![b => "name", "rva", "size"]);

    let mut streams: Vec<_> = clr.metadata.streams.values().collect();
    streams.sort_by_key(|s| s.rva);

    for s in streams {
        table.add_row(Row::new(vec![
            Cell::new(s.name()),
            Cell::new(&format!("0x{:08X}", s.rva)),
            Cell::new(&format!("{} bytes", s.size)),
        ]));
    }
    println!();
    table.printstd();
}

fn print_tables(clr: &dnfile::ClrData) {
    const KNOWN_TABLES: &[&str] = &[
        "Module",
        "TypeRef",
        "TypeDef",
        "Field",
        "MethodDef",
        "Param",
        "InterfaceImpl",
        "MemberRef",
        "Constant",
        "CustomAttribute",
        "FieldMarshal",
        "DeclSecurity",
        "ClassLayout",
        "FieldLayout",
        "StandAloneSig",
        "EventMap",
        "Event",
        "PropertyMap",
        "Property",
        "MethodSemantics",
        "MethodImpl",
        "ModuleRef",
        "TypeSpec",
        "ImplMap",
        "FieldRva",
        "Assembly",
        "AssemblyRef",
        "File",
        "ExportedType",
        "ManifestResource",
        "NestedClass",
        "GenericParam",
        "GenericMethod",
        "GenericParamConstraint",
    ];

    let mut table = capa_style_table();
    table.set_titles(row![bH2c => "Metadata Tables (non-empty)"]);
    table.add_row(row![b => "name", "rows"]);

    let mut shown = 0;
    for name in KNOWN_TABLES {
        if let Ok(t) = clr.md_table(name) {
            if t.row_count() > 0 {
                table.add_row(row![*name, t.row_count()]);
                shown += 1;
            }
        }
    }
    if shown == 0 {
        table.add_row(row!["(none)", "0"]);
    }
    println!();
    table.printstd();
}

fn print_methods(clr: &dnfile::ClrData, limit: usize) {
    let mut table = capa_style_table();
    table.set_titles(row![bH3c => format!("Methods (first {limit})")]);
    table.add_row(row![b => "#", "offset", "instructions"]);

    for (i, func) in clr.functions().iter().take(limit).enumerate() {
        table.add_row(Row::new(vec![
            Cell::new(&i.to_string()),
            Cell::new(&format!("0x{:08X}", func.offset)),
            Cell::new(&func.instructions.len().to_string()),
        ]));
    }
    println!();
    table.printstd();
}

fn print_user_strings(clr: &dnfile::ClrData) {
    let mut table = capa_style_table();
    table.set_titles(row![bH2c => "User Strings (#US)"]);
    table.add_row(row![b => "rid", "string"]);

    // RID 0 is the empty string; start from 1 and stop at first error
    // (heap end or invalid entry).
    let mut rid: usize = 1;
    let mut shown = 0;
    while let Ok(s) = clr.get_us(rid) {
        let escaped = s.replace('\n', "\\n").replace('\r', "\\r");
        let truncated = if escaped.chars().count() > 120 {
            let mut t: String = escaped.chars().take(117).collect();
            t.push_str("...");
            t
        } else {
            escaped
        };
        table.add_row(row![format!("0x{rid:X}"), truncated]);
        rid += s.len().max(1);
        shown += 1;
        if shown > 2_000 {
            // Safety brake on pathological inputs.
            break;
        }
    }
    if shown == 0 {
        table.add_row(row!["(none)", ""]);
    }
    println!();
    table.printstd();
}

fn capa_style_table() -> Table {
    let mut t = Table::new();
    t.set_format(*format::consts::FORMAT_BOX_CHARS);
    t
}
