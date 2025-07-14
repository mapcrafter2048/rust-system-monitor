use anyhow::Result;
use sysinfo::{System, Pid, Networks, Disks};

pub type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub status: String,
    pub start_time: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SortBy {
    Pid,
    Name,
    Cpu,
    Memory,
}

pub struct App {
    pub system: System,
    pub networks: Networks,
    pub disks: Disks,
    pub processes: Vec<ProcessInfo>,
    pub selected_process: usize,
    pub current_tab: usize,
    pub sort_by: SortBy,
    pub sort_ascending: bool,
    pub cpu_history: Vec<f32>,
    pub memory_history: Vec<f32>,
    pub network_history: Vec<(u64, u64)>, // (received, transmitted)
    pub disk_usage: Vec<(String, u64, u64)>, // (name, used, total)
}

impl App {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        let networks = Networks::new_with_refreshed_list();
        let disks = Disks::new_with_refreshed_list();
        
        Self {
            system,
            networks,
            disks,
            processes: Vec::new(),
            selected_process: 0,
            current_tab: 0,
            sort_by: SortBy::Cpu,
            sort_ascending: false,
            cpu_history: Vec::new(),
            memory_history: Vec::new(),
            network_history: Vec::new(),
            disk_usage: Vec::new(),
        }
    }

    pub async fn update(&mut self) {
        self.system.refresh_all();
        self.networks.refresh();
        self.disks.refresh();
        
        // Update processes
        self.update_processes();
        
        // Update system metrics
        self.update_system_metrics();
        
        // Update network stats
        self.update_network_stats();
        
        // Update disk usage
        self.update_disk_usage();
    }

    fn update_processes(&mut self) {
        self.processes.clear();
        
        for (pid, process) in self.system.processes() {
            self.processes.push(ProcessInfo {
                pid: pid.as_u32(),
                name: process.name().to_string(),
                cpu_usage: process.cpu_usage(),
                memory: process.memory(),
                status: format!("{:?}", process.status()),
                start_time: process.start_time(),
            });
        }
        
        // Sort processes
        self.sort_processes();
        
        // Ensure selected process is within bounds
        if self.selected_process >= self.processes.len() {
            self.selected_process = self.processes.len().saturating_sub(1);
        }
    }

    fn sort_processes(&mut self) {
        match self.sort_by {
            SortBy::Pid => {
                if self.sort_ascending {
                    self.processes.sort_by(|a, b| a.pid.cmp(&b.pid));
                } else {
                    self.processes.sort_by(|a, b| b.pid.cmp(&a.pid));
                }
            }
            SortBy::Name => {
                if self.sort_ascending {
                    self.processes.sort_by(|a, b| a.name.cmp(&b.name));
                } else {
                    self.processes.sort_by(|a, b| b.name.cmp(&a.name));
                }
            }
            SortBy::Cpu => {
                if self.sort_ascending {
                    self.processes.sort_by(|a, b| a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));
                } else {
                    self.processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal));
                }
            }
            SortBy::Memory => {
                if self.sort_ascending {
                    self.processes.sort_by(|a, b| a.memory.cmp(&b.memory));
                } else {
                    self.processes.sort_by(|a, b| b.memory.cmp(&a.memory));
                }
            }
        }
    }

    fn update_system_metrics(&mut self) {
        // CPU usage
        let cpu_usage = self.system.global_cpu_info().cpu_usage();
        self.cpu_history.push(cpu_usage);
        if self.cpu_history.len() > 60 {
            self.cpu_history.remove(0);
        }

        // Memory usage
        let memory_usage = (self.system.used_memory() as f32 / self.system.total_memory() as f32) * 100.0;
        self.memory_history.push(memory_usage);
        if self.memory_history.len() > 60 {
            self.memory_history.remove(0);
        }
    }

    fn update_network_stats(&mut self) {
        let mut total_received = 0;
        let mut total_transmitted = 0;

        for (_interface_name, data) in &self.networks {
            total_received += data.total_received();
            total_transmitted += data.total_transmitted();
        }

        self.network_history.push((total_received, total_transmitted));
        if self.network_history.len() > 60 {
            self.network_history.remove(0);
        }
    }

    fn update_disk_usage(&mut self) {
        self.disk_usage.clear();
        
        for disk in &self.disks {
            let name = disk.name().to_string_lossy().to_string();
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total - available;
            
            self.disk_usage.push((name, used, total));
        }
    }

    pub fn next_tab(&mut self) {
        self.current_tab = (self.current_tab + 1) % 4; // 4 tabs: Overview, Processes, Network, Disks
    }

    pub fn previous_tab(&mut self) {
        if self.current_tab > 0 {
            self.current_tab -= 1;
        } else {
            self.current_tab = 3;
        }
    }

    pub fn next_process(&mut self) {
        if !self.processes.is_empty() {
            self.selected_process = (self.selected_process + 1) % self.processes.len();
        }
    }

    pub fn previous_process(&mut self) {
        if !self.processes.is_empty() {
            if self.selected_process > 0 {
                self.selected_process -= 1;
            } else {
                self.selected_process = self.processes.len() - 1;
            }
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_all();
        self.networks.refresh();
        self.disks.refresh();
        self.update_processes();
    }

    pub fn toggle_sort(&mut self) {
        self.sort_by = match self.sort_by {
            SortBy::Pid => SortBy::Name,
            SortBy::Name => SortBy::Cpu,
            SortBy::Cpu => SortBy::Memory,
            SortBy::Memory => SortBy::Pid,
        };
        self.sort_processes();
    }

    pub fn kill_selected_process(&mut self) {
        if !self.processes.is_empty() && self.selected_process < self.processes.len() {
            let pid = self.processes[self.selected_process].pid;
            if let Some(process) = self.system.process(Pid::from(pid as usize)) {
                process.kill();
            }
        }
    }

    pub fn get_system_info(&self) -> SystemInfo {
        SystemInfo {
            cpu_count: self.system.cpus().len(),
            total_memory: self.system.total_memory(),
            used_memory: self.system.used_memory(),
            total_swap: self.system.total_swap(),
            used_swap: self.system.used_swap(),
            system_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
            kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
            host_name: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
            uptime: System::uptime(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub cpu_count: usize,
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub system_name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub host_name: String,
    pub uptime: u64,
}