use std::net::SocketAddr;
use std::process;

extern crate num_cpus;

use crate::app;
use crate::color;

pub fn startup_message(addr: &SocketAddr, tls: bool, route_counter: usize) {
    // Alias colors
    let colors = color::DEFAULT_COLORS;

    let value = |s: String, width: usize| -> String {
        let pad = width - s.len();
        let mut string = "".to_string();
        for _i in 0..pad {
            string += ".";
        }
        string += format!(" {}{}{}", colors.cyan, s, colors.black).as_str();
        string
    };

    let center = |s: String| -> String { format!("{:^49}", s) };

    let center_value =
        |s: String| -> String { format!("{}{:^49}{}", colors.cyan, s, colors.black) };

    let mut host = addr.ip().to_string();
    let port = addr.port();
    if host == "" {
        if addr.is_ipv6() {
            host = "[::1]".to_string();
        } else {
            host = "0.0.0.0".to_string();
        }
    }

    let mut scheme = "http";
    if tls {
        scheme = "https";
    }

    let mut main_logo = format!(
        "{}{}",
        colors.black, " ┌───────────────────────────────────────────────────┐\n"
    );
    /*if app_name != "" {
        main_logo = format!("{}{}{}{}", main_logo, " │ ", center_value(app_name), " │\n");
    }*/
    main_logo = format!(
        "{}{}{}{}",
        main_logo,
        " │ ",
        center_value(format!("Graphul v{}", app::VERSION)),
        " │\n"
    );

    if host == "0.0.0.0" {
        main_logo = format!(
            "{}{}{}{}",
            main_logo,
            " │ ",
            center(format!("{}://127.0.0.1:{}", scheme, port)),
            " │\n"
        );
        main_logo = format!(
            "{}{}{}{}",
            main_logo,
            " │ ",
            center(format!("(bound on host 0.0.0.0 and port {})", port)),
            " │\n"
        );
    } else {
        main_logo = format!(
            "{}{}{}{}",
            main_logo,
            " │ ",
            center(format!("{}://{}:{}", scheme, host, port)),
            " │\n"
        );
    }

    main_logo = format!("{}{}{}{}", main_logo, " │ ", center("".to_string()), " │\n");
    main_logo = format!(
        "{}{}{}{}",
        main_logo,
        " │ ",
        format!(
            "Handlers {}  Processes {}",
            value(route_counter.to_string(), 14),
            value(num_cpus::get().to_string(), 12)
        ),
        " │\n"
    );
    main_logo = format!(
        "{}{}{}{}",
        main_logo,
        " │ ",
        format!(
            "Runtime .{}  PID ....{}",
            value("Tokio".to_string(), 14),
            value(process::id().to_string(), 14)
        ),
        " │\n"
    );
    main_logo = format!(
        "{}{}{}",
        main_logo, " └───────────────────────────────────────────────────┘", colors.reset
    );

    println!("{}", main_logo);
}
