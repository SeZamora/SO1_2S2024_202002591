use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use ctrlc;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct SystemInfo {
    #[serde(rename = "Total RAM")]
    total_ram: u64,
    #[serde(rename = "Free RAM")]
    free_ram: u64,
    #[serde(rename = "RAM Uso")]
    ram_usage: u64,
    #[serde(rename = "Procesos")]
    processes: Vec<Process>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Process {
    #[serde(rename = "PID")]
    pid: u32,
    #[serde(rename = "Nombre")]
    name: String,
    #[serde(rename = "Linea de Comando")]
    cmd_line: String,
    #[serde(rename = "Vsz")]
    vsz: u64, 
    #[serde(rename = "Rss")]
    rss: u64, 
    #[serde(rename = "Memoria Usada")]
    memory_usage: f64,
    #[serde(rename = "Cpu Usado")]
    cpu_usage: f64,
}

#[derive(Debug, Serialize, Clone)]
struct LogProcess {
    pid: u32,
    name: String,
    cmd_line: String,
    vsz: u64,
    rss: u64,
    memory_usage: f64,
    cpu_usage: f64,
}


impl Process {
    fn get_container_id(&self) -> &str {
        &self.cmd_line
    }
}

impl Eq for Process {}  

impl Ord for Process {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cpu_usage.partial_cmp(&other.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| self.memory_usage.partial_cmp(&other.memory_usage).unwrap_or(std::cmp::Ordering::Equal))
    }
}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn kill_container(id: &str) -> std::process::Output {
    let  output = std::process::Command::new("sudo")
        .arg("docker")
        .arg("stop")
        .arg(id)
        .output()
        .expect("failed to execute process");

    println!("Matando contenedor con id: {}", id);

    output
}

fn run_docker_logs() -> std::process::Output {
    let  output = std::process::Command::new("sudo")
        .arg("docker")
        .arg("run")
        .arg("-d")
        .arg("-v")
        .arg("/var/logs:/app/logs")
        .arg("-p")
        .arg("8000:8000")
        .arg("admin_log")
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        println!("Container started successfully!");
    } else {
        eprintln!("Error starting container: {:?}", output);
    }

    output
}

fn enviar_logs(logs: &[LogProcess]) -> Result<(), reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let res = client.post("http://localhost:8000/logs")
        .json(&logs)  
        .send()?;

    println!("Enviando logs: {:?}", res);
    Ok(())
}

fn graficar_logs()-> Result<(), reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let res = client.get("http://localhost:8000/graficar")
        .send()?;

    println!("Graficando logs: {:?}", res);
    Ok(())
}



fn read_proc_file(file_name: &str) -> io::Result<String> {
    let path  = Path::new("/proc").join(file_name);
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn parse_proc_to_struct(json_str: &str) -> Result<SystemInfo, serde_json::Error> {
    let system_info: SystemInfo = serde_json::from_str(json_str)?;

    Ok(system_info)
}

fn eliminar_cronjob() -> std::process::Output {
    let output = std::process::Command::new("sudo")
        .arg("crontab")
        .arg("-r")
        .output()
        .expect("failed to execute process");

    println!("Cronjob eliminado");
    output
}

fn analyzer( system_info:  SystemInfo) {


    let mut log_proc_list: Vec<LogProcess> = Vec::new();

    let mut processes_list: Vec<Process> = system_info.processes;

    let mut highest_list: Vec<&Process> = Vec::new();
    let mut lowest_list: Vec<&Process> = Vec::new();

    processes_list.sort();


    for process in &processes_list {
        if process.memory_usage > 5.00 || process.cpu_usage > 30.00 {
            highest_list.push(process);
        }else {
            lowest_list.push(process);
        }
    }


    // Hacemos un print de los contenedores de bajo consumo en las listas.
    println!("Bajo consumo");
    for process in &lowest_list {
        println!("PID: {}, Name: {}, container ID: {}, Memory Usage: {}, CPU Usage: {}", process.pid, process.name, process.get_container_id(), process.memory_usage, process.cpu_usage);
    }

    println!("------------------------------");

    println!("Alto consumo");
    for process in &highest_list {
        println!("PID: {}, Name: {}, Icontainer ID {}, Memory Usage: {}, CPU Usage: {}", process.pid, process.name,process.get_container_id(),process.memory_usage, process.cpu_usage);
    }

    println!("------------------------------");

    if lowest_list.len() > 3 {
        for process in lowest_list.iter().skip(3) {
            let log_process = LogProcess {
                pid: process.pid,
                name: process.get_container_id().to_string(),
                cmd_line: process.cmd_line.clone(),
                vsz: process.vsz,
                rss: process.rss,
                memory_usage: process.memory_usage,
                cpu_usage: process.cpu_usage,
            };

            log_proc_list.push(log_process.clone());

            // Matamos el contenedor.
            let _output = kill_container(&process.get_container_id());
        }
    }

    if highest_list.len() > 2 {
        for process in highest_list.iter().take(highest_list.len() - 2) {
            let log_process = LogProcess {
                pid: process.pid,
                name: process.get_container_id().to_string(),
                cmd_line: process.cmd_line.clone(),
                vsz: process.vsz,
                rss: process.rss,
                memory_usage: process.memory_usage,
                cpu_usage: process.cpu_usage
            };
    
            log_proc_list.push(log_process.clone());

            // Matamos el contenedor.
            let _output = kill_container(&process.get_container_id());

        }
    }

    if !log_proc_list.is_empty() {
        // Enviar logs al contenedor de logs
        match enviar_logs(&log_proc_list) {
            Ok(_) => println!("Logs enviados correctamente"),
            Err(e) => eprintln!("Error al enviar logs: {}", e),
        }
    }

    println!("Contenedores matados");
    for process in log_proc_list {
        println!("PID: {}, Name: {}, Memory Usage: {}, CPU Usage: {} ", process.pid, process.name,  process.memory_usage, process.cpu_usage);
    }

    println!("------------------------------");

    
}

fn main() {

    let running = Arc::new(AtomicBool::new(true));
    let r = Arc::clone(&running);

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let _output = run_docker_logs();
    while running.load(Ordering::SeqCst) {
        // Lógica principal del programa
        let system_info: Result<SystemInfo, _>;

        let json_str = read_proc_file("sysinfo_202002591").unwrap();

        system_info = parse_proc_to_struct(&json_str);

        match system_info {
            Ok(info) => {
                analyzer(info);
            }
            Err(e) => println!("Failed to parse JSON: {}", e),
        }

        std::thread::sleep(std::time::Duration::from_secs(10));
    }
    match graficar_logs() {
        Ok(_) => println!("Logs enviados correctamente"),
        Err(e) => eprintln!("Error al enviar logs: {}", e),
    }
    eliminar_cronjob();
    println!("Servicio finalizado correctamente.");
    

}