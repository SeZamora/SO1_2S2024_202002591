use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use serde::{Deserialize, Serialize};


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


impl Process {
    fn get_container_id(&self) -> &str {
        &self.name
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


fn analyzer( system_info:  SystemInfo) {



    let mut processes_list: Vec<Process> = system_info.processes;

    let mut highest_list: Vec<&Process> = Vec::new();
    let mut lowest_list: Vec<&Process> = Vec::new();

    processes_list.sort();


    for process in &processes_list {
        if process.memory_usage > 5.00 || process.cpu_usage > 100.00 {
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


    
}

fn main() {
    let system_info: Result<SystemInfo, _>;
    let json_str = read_proc_file("sysinfo_2000").unwrap();
    system_info = parse_proc_to_struct(&json_str);
    match system_info {
        Ok(info) => {
            analyzer(info);
        }
        Err(e) => println!("Failed to parse JSON: {}", e),
    }

}