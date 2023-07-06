use colored::Colorize;

pub fn env_check(vars: &[&str]) -> Result<(), ()> {
    let mut ret = "Successfully loaded env vars".to_string();
    let mut not_found: Vec<String> = Vec::new();

    for var in vars {
        if std::env::var(var).is_ok() {
            if std::env::var(var).unwrap_or_else(|_| panic!("Couldn't get env variable: {}", var)) == "" {
                not_found.insert(0, var.to_string());
            } else {
                ret += &format!("\n  - {}", var.green());
            }
        } else {
            not_found.insert(0, var.to_string());
        }
    }

    if !not_found.is_empty() {
        std::panic::set_hook(Box::new(move |_| {
            println!("Couldn't load the environmanet variable(s):");

            for nf in &not_found {
                println!("  - {}", nf.red());
            }

            println!("\n{}", "Stopping server".bold());
        }));

        return Err(());
    }

    println!("{}", ret);
    Ok(())
}
