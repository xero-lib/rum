use std::fs;
use std::fs::read_to_string;

fn main() {
    let Ok(proc_dir) = fs::read_dir("/proc") else {
        eprintln!("Unable to open /proc");
        return;
    };

    // let procs: Vec<i128> = proc_dir.filter(
    //     |dir|
    //         dir.is_ok()
    // ).filter_map(
    //     |f|
    //         f.unwrap().file_name().to_string_lossy().parse::<i128>().ok()
    // ).collect();

    let proc_dirs: Vec<fs::DirEntry> = proc_dir.filter_map(
        |f|
                f.ok().and_then(|dir| dir.file_name().to_string_lossy().parse::<i128>().is_ok().then_some(dir))
    ).collect();

    for i in proc_dirs {
        // println!("{}", i.path().iter().last().unwrap().to_str().unwrap());
        // let comm_file = fs::read_dir(i).unwrap().find(|sd| sd.is_ok_and(|dir| dir.file_name().to_string_lossy() == "comm".into())).unwrap().unwrap();
        let comm_file = read_to_string(i.path().join("comm")).unwrap();
        println!("Name: {name}\nPID: {pid}\n",
            pid = i.file_name().to_string_lossy(),
            name = comm_file.strip_suffix('\n').unwrap()
        );
    }
}
