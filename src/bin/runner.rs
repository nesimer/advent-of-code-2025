use std::process::Command;
use std::time::{Duration, Instant};
use std::path::Path;

fn main() {
    // ASCII Art de Noël
    println!("\x1b[32m"); // Vert
    println!(r#"
        .   *   .       .   .   *      .   .
      .   *  .  .   *   .   .      *   .
         .   /\   .   *    .    *   .   .
       *    /  \      .   .    *   .
           /    \   *   .     .   *
          /      \     .   *       .
         /  ~  ~  \   .       .   .
        /  ~  ~  ~ \      *   .
       /____________\  .     .
            |  |   .   *   .   .
         ___|__|___    .   .
    "#);
    println!("\x1b[0m"); // Reset
    println!("\x1b[1;31m🎄 Advent of Code 2025 - Runner & Benchmark 🎄\x1b[0m\n");

    // 1. Build de tous les binaires en mode release
    println!("🔨 \x1b[1mCompilation de tous les jours en mode release...\x1b[0m");
    let status = Command::new("cargo")
        .args(&["build", "--release", "--bins"])
        .status()
        .expect("Échec de l'exécution de cargo build");

    if !status.success() {
        eprintln!("❌ La compilation a échoué.");
        return;
    }
    println!("✅ Compilation terminée.\n");

    let days = 1..=12;
    let mut total_duration = Duration::new(0, 0);
    let mut results = Vec::new();

    // En-tête du tableau
    println!("┌{:─<10}┬{:─<15}┬{:─<15}┐", "", "", "");
    println!("│ {:<8} │ {:<13} │ {:<13} │", "Jour", "Statut", "Temps");
    println!("├{:─<10}┼{:─<15}┼{:─<15}┤", "", "", "");

    for day in days {
        let day_str = format!("day{:02}", day);
        // Le chemin dépend de l'OS, mais sous Linux c'est target/release/dayXX
        let bin_path = format!("target/release/{}", day_str);
        
        if !Path::new(&bin_path).exists() {
             results.push((day, "Manquant".to_string(), Duration::new(0, 0)));
             println!("│ {:<8} │ \x1b[33m{:<13}\x1b[0m │ {:<13} │", day_str, "⚠️ Manquant", "-");
             continue;
        }

        let start = Instant::now();
        let output = Command::new(&bin_path)
            .output();
        let duration = start.elapsed();

        match output {
            Ok(out) => {
                if out.status.success() {
                    results.push((day, "Succès".to_string(), duration));
                    total_duration += duration;
                    println!("│ {:<8} │ \x1b[32m{:<13}\x1b[0m│ {:<13.2?} │", day_str, "✅ Succès", duration);
                } else {
                    results.push((day, "Échec".to_string(), duration));
                    println!("│ {:<8} │ \x1b[31m{:<13}\x1b[0m│ {:<13.2?} │", day_str, "❌ Échec", duration);
                }
            }
            Err(_) => {
                results.push((day, "Erreur".to_string(), Duration::new(0, 0)));
                println!("│ {:<8} │ \x1b[31m{:<13}\x1b[0m │ {:<13} │", day_str, "⚠️ Erreur", "-");
            }
        }
    }

    println!("└{:─<10}┴{:─<15}┴{:─<15}┘", "", "", "");

    println!("\n\x1b[1mTemps Total:\x1b[0m {:.2?}", total_duration);
    if !results.is_empty() {
        let avg = total_duration / results.len() as u32;
        println!("\x1b[1mMoyenne par jour:\x1b[0m {:.2?}", avg);
    }
    println!("\n\x1b[1;31m🎅 Joyeux Noël et bon code ! 🎅\x1b[0m");
}
