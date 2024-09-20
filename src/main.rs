const MB: usize = 1024 * 1024;

fn alloc_buffer(size: usize) -> Box<[u8]> {
    println!("Allocating {} MiB buffer", size / MB);
    vec![0; size].into_boxed_slice()
}

mod alloc_1_mb {
    use super::*;
    pub(crate) fn alloc() -> Box<[u8]> {
        alloc_buffer(1 * MB)
    }
}
mod alloc_10_mb {
    use super::*;
    pub(crate) fn alloc() -> Box<[u8]> {
        alloc_buffer(10 * MB)
    }
}
mod alloc_100_mb {
    use super::*;
    pub(crate) fn alloc() -> Box<[u8]> {
        alloc_buffer(100 * MB)
    }
}

mod wpr {
    use std::process::Command;

    fn run_cmd(mut cmd: Command) {
        println!(
            "> {} {}",
            format!("{:?}", cmd.get_program()).trim_matches('"'),
            format!(
                "{:?}",
                cmd.get_args()
                    .collect::<Vec<_>>()
                    .join(std::ffi::OsStr::new(" "))
            )
            .trim_matches('"')
        );
        let status = cmd.status().unwrap();
        assert!(status.success());
    }

    pub(crate) struct WprSession {
        pid: u32,
        output_file: String,
    }

    impl WprSession {
        pub(crate) fn new(output_file: String) -> Self {
            let pid = std::process::id();
            println!("Starting profiling for process {pid}");
            Self::enable(pid);
            Self::start();            
            let this = Self { pid, output_file };
            this.make_snapshot();
            this
        }

        fn enable(pid: u32) {
            let mut cmd = Command::new("wpr");
            cmd.args(["-snapshotconfig", "heap"])
                .args(["-pid", &pid.to_string()])
                .arg("enable");
            run_cmd(cmd);
        }
        fn disable(&self) {
            let mut cmd = Command::new("wpr");
            cmd.args(["-snapshotconfig", "heap"])
                .args(["-pid", &self.pid.to_string()])
                .arg("disable");
            run_cmd(cmd);
        }
        fn start() {
            let mut cmd = Command::new("wpr");
            cmd.args(["-start", "heapsnapshot", "-filemode"]);
            run_cmd(cmd);
        }
        fn stop(&self) {
            let mut cmd = Command::new("wpr");
            cmd.args(["-stop", &self.output_file]);
            run_cmd(cmd);
        }
        pub(crate) fn make_snapshot(&self) {
            let mut cmd = Command::new("wpr");
            cmd.args(["-singlesnapshot", "heap", &self.pid.to_string()]);
            run_cmd(cmd);
        }
    }

    impl Drop for WprSession {
        fn drop(&mut self) {
            self.stop();
            self.disable();
        }
    }
}

fn main() {
    if !is_elevated::is_elevated() {
        println!("[error] wpr requires administrative priviliges to collect heap snapshots!");
        std::process::exit(1);
    }

    let output_path = std::env::args()
        .nth(1)
        .unwrap_or("heapsnapshot.etl".to_string());
    let wpr_session = wpr::WprSession::new(output_path);
    
    let mut buffers = Vec::new();

    for _ in 0..3 {
        buffers.push(alloc_1_mb::alloc());
        wpr_session.make_snapshot();
    }
    for _ in 0..3 {
        buffers.push(alloc_10_mb::alloc());
        wpr_session.make_snapshot();
    }
    for _ in 0..3 {
        buffers.push(alloc_100_mb::alloc());
        wpr_session.make_snapshot();
    }

    println!(
        "Expected RAM consumption {} MiB",
        buffers.iter().map(|b| b.len()).sum::<usize>() / MB
    );
    drop(wpr_session);
}
