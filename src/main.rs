use std::fs::{self, DirEntry };
use sysinfo::{SystemExt, System, ProcessExt, CpuExt, Process, Pid};

fn main() {
    // Create system information reader
    let mut system = System::new();
    // Initialize system information reader
    system.refresh_all();
    // Wait for the next update
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);

    let mut ordered_procs = system
        .processes()
        .iter()
        .collect::<Vec<(&Pid, &Process)>>();

    ordered_procs.sort_by(|(pid, _), (next_pid, _)| pid.cmp(next_pid));

    for (pid, proc) in ordered_procs {
        println!("{}: {} => cpu: {:?}", pid, proc.name(), proc.cpu_usage());
    }

    //? Reports seem false, researching
    for (cpu_id, cpu) in system.cpus().iter().enumerate() {
        println!("CPU {}: {}%", cpu_id, cpu.cpu_usage());
    }

    let unit_power: u8= 3; /* each power increases unit by one factor (kilo -> mega, etc) */
    let mem_denominator = 1000u64.pow(unit_power.into());
    let units = match unit_power {
        0 => "",
        1 => "KB",
        2 => "MB",
        3 => "GB",
        4 => "TB",
        5 => "PB",
        6 => "EB",
        _ => ">EB"
    };

    println!(
        "Mem: {}{units}/{}{units}",
        system.used_memory() / mem_denominator,
        system.total_memory() / mem_denominator
    );
}
#[allow(dead_code, for_loops_over_fallibles, unused_parens)]
fn manual_read() {
    let Ok(proc_dir) = fs::read_dir("/proc") else {
        eprintln!("Unable to open /proc");
        return;
    };

    let proc_dirs: Vec<DirEntry> = proc_dir
        .filter_map(|f| {
            f.ok().and_then(|dir| {
                dir.file_name()
                    .to_string_lossy()
                    .parse::<i128>()
                    .is_ok()
                    .then_some(dir)
            })
        })
        .collect();
    
    let mut system = System::new();
    system.refresh_memory();
    let mut total: f64 = 0f64;

    println!("{}", system.total_memory());

    for i in proc_dirs {
        let comm_file = fs::read_to_string(i.path().join("comm")).unwrap();
        print!(
            "Name: {name}\nPID: {pid}\n",
            pid = i.file_name().to_string_lossy(),
            name = comm_file.strip_suffix('\n').unwrap()
        );
        
        /*-------Memory-------*/

        let mut proc_total: u64 = 0;

        for map in fs::read_dir(i.path().join("map_files")) {
            for vals_dir in map {
                let Ok(file) = vals_dir else {
                    eprintln!("Unable to access map file in {}", i.file_name().to_string_lossy());
                    continue;
                };
                let file_name = file.file_name();
                let file_name = file_name.to_string_lossy();
                let vals: Vec<&str> = file_name.split('-').collect();
                proc_total += (u64::from_str_radix(vals[1], 16).unwrap() - u64::from_str_radix(vals[0], 16).unwrap());
            }
        }

        println!("{}K\n", proc_total / 1000);
       total += proc_total as f64;
    }

    println!("Total Allocated Bytes: {total}");
    println!("Reported Memory: {}%", system.used_memory() as f64 / system.total_memory() as f64 * 100f64);
}
