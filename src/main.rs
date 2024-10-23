use clap::Parser;
use colored::*;
use serde::Serialize;
use serde_json::Value;
use std::process::Command;

#[derive(Parser)]
struct Args {
    /// number of top services to display
    #[arg(short, long)]
    top_n: Option<u32>,

    /// predicate by which to filter services (MEDIUM or EXPOSED)
    #[arg(short, long)]
    predicate: Option<String>,

    /// only return services with the "OK" predicate
    #[arg(long)]
    ok: bool,

    /// only return services with the "MEDIUM" predicate
    #[arg(long)]
    medium: bool,

    /// only return services with the "EXPOSED" predicate
    #[arg(long)]
    exposed: bool,

    /// only return services with the "UNSAFE" predicate
    #[arg(long)]
    unsafe_: bool,

    /// enable debug mode to print the raw json output
    #[arg(long)]
    debug: bool,

    /// output results in json format
    #[arg(long)]
    json: bool,
}

// store unit details in a struct
#[derive(Debug, Clone, Serialize)]
struct Service {
    /// name of the unit
    unit: String,

    /// exposure value of the unit
    exposure: f64,

    /// exposure predicate of the unit
    predicate: String,

    /// happiness score of the unit, represented
    /// by emojis: 😀, 🙂, 😐, 🙁, 😨
    happy: String,
}

#[derive(Debug, Serialize)]
struct AnalysisResult {
    average_exposure: f64,
    average_happiness: f64,
    top_services: Vec<Service>,
}

fn run_systemd_analyze(debug: bool) -> Vec<Service> {
    let output = Command::new("systemd-analyze")
        .args(&["security", "--json=short", "--no-pager"])
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        panic!("systemd-analyze failed: {}", err);
    }

    if debug {
        println!("{}", "Raw JSON output:".bold().yellow());
        println!("{}", String::from_utf8_lossy(&output.stdout).green());
    }

    let json_output: Value = serde_json::from_slice(&output.stdout).expect("failed to parse json");
    let mut services: Vec<Service> = Vec::new();

    if let Some(entries) = json_output.as_array() {
        for entry in entries {
            let exposure = match entry.get("exposure") {
                Some(Value::Number(num)) => num.as_f64(),
                Some(Value::String(s)) => s.parse::<f64>().ok(),
                _ => None,
            };

            if let (Some(unit), Some(exposure), Some(predicate), Some(happy)) = (
                entry.get("unit").and_then(|v| v.as_str()),
                exposure,
                entry.get("predicate").and_then(|v| v.as_str()),
                entry.get("happy").and_then(|v| v.as_str()),
            ) {
                services.push(Service {
                    unit: unit.to_string(),
                    exposure,
                    predicate: predicate.to_string(),
                    happy: happy.to_string(),
                });
            } else {
                println!("Warning: could not parse entry: {:?}", entry);
            }
        }
    }
    services
}

fn calculate_exposure_average(services: &[Service]) -> f64 {
    if services.is_empty() {
        return f64::NAN;
    }
    let total_exposure: f64 = services.iter().map(|s| s.exposure).sum();
    total_exposure / services.len() as f64
}

fn calculate_happiness_average(services: &[Service]) -> f64 {
    let happiness_map = vec![
        ("😀", 5.0),
        ("🙂", 4.0),
        ("😐", 3.0),
        ("🙁", 2.0),
        ("😨", 1.0),
    ];
    let mut total_happiness = 0.0;
    let mut count = 0;

    for service in services {
        if let Some(&score) =
            happiness_map
                .iter()
                .find_map(|(h, s)| if service.happy == *h { Some(s) } else { None })
        {
            total_happiness += score;
            count += 1;
        } else {
            println!("Warning: unmatched happy value '{}'", service.happy);
        }
    }

    if count == 0 {
        f64::NAN
    } else {
        total_happiness / count as f64
    }
}

fn colorize_predicate(predicate: &str) -> ColoredString {
    match predicate {
        "OK" => predicate.green(),
        "MEDIUM" => predicate.white(),
        "EXPOSED" => predicate.yellow(),
        "UNSAFE" => predicate.red(),
        _ => predicate.normal(),
    }
}

fn main() {
    let args = Args::parse();
    let services = run_systemd_analyze(args.debug);
    let exposure_avg = calculate_exposure_average(&services);
    let happiness_avg = calculate_happiness_average(&services);

    // If you don't like this, let me remind you that our alternative
    // is a large if else block.
    let predicate = match (args.ok, args.medium, args.exposed, args.unsafe_) {
        (true, _, _, _) => Some("OK"),
        (_, true, _, _) => Some("MEDIUM"),
        (_, _, true, _) => Some("EXPOSED"),
        (_, _, _, true) => Some("UNSAFE"),
        _ => args.predicate.as_deref(),
    };

    let mut filtered_services = if let Some(pred) = predicate {
        services
            .iter()
            .filter(|s| s.predicate == pred)
            .cloned()
            .collect()
    } else {
        services.clone()
    };

    // Apply --top-n after filtering by predicate
    // Since we're using a Vec, we can just sort
    // and take the top n elements. This is better
    // than my previous approach.
    if let Some(top_n) = args.top_n {
        filtered_services.sort_by(|a, b| {
            b.exposure
                .partial_cmp(&a.exposure)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        filtered_services = filtered_services.into_iter().take(top_n as usize).collect();
    }

    // Output in JSON format if --json is set, for future parsing
    // in CI/CD environments.
    if args.json {
        let result = AnalysisResult {
            average_exposure: exposure_avg,
            average_happiness: happiness_avg,
            top_services: filtered_services,
        };
        let json_output =
            serde_json::to_string_pretty(&result).expect("failed to serialize to json");
        println!("{}", json_output);
    } else {
        println!(
            "{}\n\n{} {:.2} | {} {:.2}",
            "# Systemd Security Analysis".bold().cyan(),
            "Average Exposure:",
            exposure_avg,
            "Average Happiness:",
            happiness_avg
        );

        println!(
            "\n{} {} {} '{}'\n",
            "## Top".bold().cyan(),
            filtered_services.len(),
            "services for predicate:".bold().cyan(),
            predicate.map_or("N/A".normal(), |pred| colorize_predicate(pred))
        );

        for service in filtered_services {
            println!(
                "{} {} {} ({} {:.2})",
                "•".green(),
                service.unit.bold(),
                "->".blue(),
                colorize_predicate(&service.predicate),
                service.exposure
            );
        }
    }
}
