use std::env;
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() {
    let program_start_timer = Instant::now();
    let command_line_arguments: Vec<String> = env::args().collect();

    let target_ip_address = if command_line_arguments.len() > 1 {
        command_line_arguments[1].clone()
    } else {
        String::from("scanme.nmap.org")
    };

    println!("🚀 Starting High-Speed Port Scan against: {}", target_ip_address);

    let maximum_concurrent_scans: usize = 500;
    
    // We will scan the most common ports (1 through 10,000)
    let ports_to_scan_iterator = 1..=10000;

    let port_scanning_stream = stream::iter(ports_to_scan_iterator).map(|target_port_number| {
        let cloned_target_ip = target_ip_address.clone();
        
        async move {
            let socket_address_string = format!("{}:{}", cloned_target_ip, target_port_number);
            
            let maximum_connection_timeout = Duration::from_millis(1000);
            
            let connection_attempt_future = TcpStream::connect(&socket_address_string);
            
            let timeout_result = tokio::time::timeout(maximum_connection_timeout, connection_attempt_future).await;

            match timeout_result {
                Ok(Ok(_successful_tcp_stream)) => {
                    println!("🟢 Port OPEN: {}", target_port_number);
                    Some(target_port_number)
                }
                
                Ok(Err(_connection_rejected_error)) => {
                    None 
                }
                
                Err(_timeout_elapsed_error) => {
                    None 
                }
            }
        }
    });

    let aggregated_scan_results: Vec<Option<u16>> = port_scanning_stream
        .buffer_unordered(maximum_concurrent_scans)
        .collect()
        .await;

   
    let mut final_open_ports_list: Vec<u16> = Vec::new();
    for scan_result_option in aggregated_scan_results {
        if let Some(valid_open_port) = scan_result_option {
            final_open_ports_list.push(valid_open_port);
        }
    }

    final_open_ports_list.sort_unstable();

    let total_scan_duration = program_start_timer.elapsed();
    
    println!("\n✅ Scan Complete in {:?}", total_scan_duration);
    println!("Open Ports Discovered: {:?}", final_open_ports_list);
}