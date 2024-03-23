use std::fs;
use anyhow::Result;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Hardware {
    pub cpu_brend: String,
    pub cpu_cores: u8,
    pub ram_size: u64,
    pub kernel_ver: String,
    pub os_name: String,
}

impl Hardware {
    pub fn new() -> Result<Self> {
        let cpuinfo = fs::read_to_string("/proc/cpuinfo")?;
        let meminfo = fs::read_to_string("/proc/meminfo")?;
        let os_rel = fs::read_to_string("/etc/os-release")?;

        Ok(Self {
            cpu_brend: Self::get_cpu_model(&cpuinfo),
            cpu_cores: Self::get_cpu_cores(&cpuinfo),
            ram_size: Self::get_ram_size(&meminfo),
            kernel_ver: fs::read_to_string("/proc/version").unwrap_or("Linux version UNKNOWN".to_string()),
            os_name: Self::get_os_name(&os_rel),
        })
    }

    fn get_cpu_model(cpuinfo: &str) -> String {
        let cpu = cpuinfo.lines()
            .filter(|line| line.contains("model name"))
            .collect::<Vec<_>>();
        let model: Vec<&str> = cpu[0].split(':').collect();
        model[1].trim_start().to_string()
    }

    fn get_cpu_cores(cpuinfo: &str) -> u8 {
        let cpu = cpuinfo.lines()
            .filter(|line| line.contains("processor"))
            .collect::<Vec<_>>();

        if cpu.is_empty() {
            return 0;
        }

        let mut cores = 0;

        for _ in cpu {
            cores += 1;
        }

        cores
    }

    fn get_ram_size(meminfo: &str) -> u64 {
        let mem = meminfo.lines()
            .filter(|line| line.contains("MemTotal"))
            .collect::<Vec<_>>();

        if mem.is_empty() {
            return 0;
        }

        let parts: Vec<&str> = mem[0].splitn(2, ':').collect();

        match parts[1].trim_start().split(' ').next() {
            Some(kb) => {
                if let Ok(val) = kb.parse::<u64>() {
                    val / 1024
                } else {
                    0
                }
            },
            None => 0,
        }
    }

    fn get_os_name(os_rel: &str) -> String {
        let os = os_rel.lines()
            .filter(|line| line.contains("NAME="))
            .collect::<Vec<_>>();

        if os.is_empty() {
            return "Unknown Linux system".to_string();
        }

        let parts: Vec<&str> = os[0].split('=').collect();
        parts[1].replace('"', "").to_string()
    }
}
