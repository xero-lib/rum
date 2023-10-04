use std::fs;

fn main() {
    let Ok(proc_dir) = fs::read_dir("/proc") else {
        eprintln!("Unable to open /proc");
        return;
    };

    let proc_dirs: Vec<fs::DirEntry> = proc_dir.filter_map(
        |f|
                f.ok().and_then(|dir| dir.file_name().to_string_lossy().parse::<i128>().is_ok().then_some(dir))
    ).collect();

    for i in proc_dirs {
        let comm_file = fs::read_to_string(i.path().join("comm")).unwrap();
        println!("Name: {name}\nPID: {pid}\n",
            pid = i.file_name().to_string_lossy(),
            name = comm_file.strip_suffix('\n').unwrap()
        );
    }
}
