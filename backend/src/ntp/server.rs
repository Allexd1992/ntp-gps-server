
use std::sync::Arc;



use tokio::sync::Mutex;
use tokio::net::UdpSocket;



use super::NtpPacket;
use super::NtpServerState;
use super::NtpTimestamp;
use super::NtpFracValue;

pub struct Server {
    state: Arc<Mutex<NtpServerState>>,
    sockets: Arc<Mutex<Vec<Arc<Mutex<UdpSocket>>>>>,
    debug: bool,
}

impl Server {
    pub async fn new(local_addrs: Vec<String>, debug: bool) -> Server {
        let state = NtpServerState {
            leap: 0,
            stratum: 0,
            precision: 0,
            ref_id: 0,
            ref_ts: NtpTimestamp::zero(),
            dispersion: NtpFracValue::zero(),
            delay: NtpFracValue::zero(),
        };

        let mut sockets = vec![];

        for addr in local_addrs {
            let socket = UdpSocket::bind(format!("{}:{}", addr, 123)).await.unwrap();
            debug!("{:?}", socket);
            sockets.push(Arc::new(Mutex::new(socket)));
        }

        Server {
            state: Arc::new(Mutex::new(state)),
            sockets: Arc::new(Mutex::new(sockets)),
            debug: debug,
        }
    }

    pub async fn process_requests(
        thread_id: u32,
        debug: bool,
        socket: Arc<Mutex<UdpSocket>>,
        state: Arc<Mutex<NtpServerState>>,
    ) {
        let mut last_update = NtpTimestamp::now();
        let mut cached_state: NtpServerState;
        cached_state = *state.lock().await;

        info!("Server thread #{} started", thread_id);
        let socket = socket.lock().await;
        loop {
            match NtpPacket::receive(&socket).await {
                Ok(request) => {
                    if debug {
                        info!("Thread #{} received {:?}", thread_id, request);
                    }

                    if request.local_ts.diff_to_sec(&last_update).abs() > 0.1 {
                        cached_state = *state.lock().await;
                        last_update = request.local_ts;
                        if debug {
                            info!("Thread #{} updated its state", thread_id);
                        }
                    }

                    match request.make_response(&cached_state) {
                        Some(response) => match response.send(&socket).await {
                            Ok(_) => {
                                info!("Thread #{} sent {:?}", thread_id, response);
                            }
                            Err(e) => error!(
                                "Thread #{} failed to send packet to {}: {}",
                                thread_id, response.remote_addr, e
                            ),
                        },
                        None => {}
                    }
                }
                Err(e) => {
                    error!("Thread #{} failed to receive packet: {}", thread_id, e);
                }
            }
        }
    }

    pub async fn update_state(&mut self, timestamp: NtpTimestamp) {
        let mut state = self.state.lock().await;

        state.ref_ts = timestamp;

        state.dispersion.increment();
    }

    pub async fn run(&self) {
        let mut threads = vec![];
        let mut id = 0;
        let quit = false;

        for socket in self.sockets.lock().await.iter() {
            id = id + 1;
            let state = Arc::clone(&self.state);
            let debug = self.debug;
            let cloned_socket = Arc::clone(socket);

            threads.push(tokio::spawn(async move {
                Server::process_requests(id, debug, cloned_socket, state).await;
            }));
        }
    }
}
