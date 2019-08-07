use std::{
    fs::canonicalize,
    env,
    collections::HashMap,
    fs,
    thread,
    path::{
        Path,
        PathBuf
    },
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
        mpsc::{Receiver, Sender}
    },
    thread::JoinHandle,
    time::{Duration, SystemTime}
};

pub struct ResourceListener {
    running: Arc<AtomicBool>,
    join: Option<JoinHandle<()>>,
    pair_register: Option<Sender<(Sender<(String, String)>, String, String)>>,
}

impl ResourceListener {
    pub fn new() -> Self {
        ResourceListener {
            running: Arc::new(AtomicBool::new(true)),
            join: None,
            pair_register: None,
        }
    }
}

impl ResourceListener {
    pub fn start(&mut self) {
        println!("Starting resource listener.");
        let running = self.running.clone();
        let (sn, rv) = mpsc::channel();
        self.pair_register = Some(sn);

        self.join = Some(thread::spawn(move || {
            let pair_listeners_receiver = rv;
            let mut map: HashMap<String, SystemTime> = HashMap::new();
            let mut pair_listeners: HashMap<(String, String), Sender<(String, String)>> = HashMap::new();

            while running.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(500));


                //adding to listeners
                if let Ok(r) = pair_listeners_receiver.try_recv() {
                    match fs::metadata(r.1.as_str()) {
                        Ok(m) => {
                            map.insert(r.1.clone(), m.modified().expect("file not found:"));
                            match fs::metadata(r.2.as_str()) {
                                Ok(m) => {
                                    map.insert(r.2.clone(), m.modified().expect("file not found:"));
                                    let content1 = fs::read_to_string(r.1.as_str()).expect("File 1").to_string();
                                    let content2 = fs::read_to_string(r.2.as_str()).expect("File 2").to_string();
                                    r.0.send((content1, content2));
                                    pair_listeners.insert((r.1.clone(), r.2.clone()), r.0);
                                }
                                Err(e) => { println!("{}", e) }
                            }
                        }
                        Err(e) => {
                            println!("{}", e)
                        }
                    }
                }
                //checking update
                for (path, s_time) in map.clone().iter_mut() {
                    let last_mod = fs::metadata(path.as_str())
                        .expect("File is deleted.").modified().expect("Error");
                    if last_mod != *s_time {
                        map.insert(path.to_owned(), last_mod);
                        println!("{} - {:?}", path.as_str(), last_mod);

                        for ((first, second), sender) in pair_listeners.iter() {
                            if first == path || second == path {
                                let content1 = fs::read_to_string(first).expect("File 1").to_string();
                                let content2 = fs::read_to_string(second).expect("File 2").to_string();
                                sender.send((content1, content2));
                            }
                        }
                    }
                }
            };
        }));
    }

    pub fn listen_pair(&self, first_file: &str, second_file: &str) -> Receiver<(String, String)> {
        let (sender, receiver) = mpsc::channel();

        println!("Adding {} and {} to listener.", first_file, second_file);
        self.pair_register.as_ref().unwrap()
            .send((sender, String::from(first_file), String::from(second_file)));

        receiver
    }

    pub fn close(&mut self) {
        self.running.swap(false, Ordering::Relaxed);
        self.join.take().unwrap().join();
    }
}

impl Drop for ResourceListener {
    fn drop(&mut self) {
        self.close();
    }
}

pub fn relative_path(relative: &str, target: &[&str]) -> PathBuf {
    let current_file = relative.to_owned().to_string();
    let mut base_path = canonicalize(&current_file)
        .expect("").parent()
        .unwrap().to_str().unwrap().to_string();

    let mut path = Path::new(&base_path).to_path_buf();
    for s in target {
        path.push(*s)
    }
    path
}

pub fn relative_to_current_path(target: &[&str]) -> PathBuf {
    let mut base_path = env::current_dir().unwrap();

    let mut path = base_path.to_path_buf();
    for s in target {
        path.push(*s)
    }
    path
}

