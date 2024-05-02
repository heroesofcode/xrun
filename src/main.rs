use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "set -o pipefail && xcodebuild -project DeliveryApp.xcodeproj \
             -scheme DeliveryApp \
             -destination platform=iOS\\ Simulator,OS=17.4,name=iPhone\\ 15 \
             clean test | xcpretty"))
        .current_dir("DeliveryApp-iOS")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Falha ao iniciar o comando");

    let mut passed_tests = 0;
    let mut failed_tests = 0;

    if let Some(stdout) = output.stdout {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.contains("✓") {
                    passed_tests += 1;
                    println!("{}", line);
                } else if line.contains("❌") {
                    failed_tests += 1;
                    println!("{}", line);
                }
            }
        }
    }

    let duration = start_time.elapsed();

    println!("Tempo de execução: {:.2?}", duration);

    println!("Resumo de Testes:");
    println!("Total de Testes: {}", passed_tests + failed_tests);
    println!("Passados: {}", passed_tests);
    println!("Falhados: {}", failed_tests);
}