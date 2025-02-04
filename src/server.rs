pub trait Server {
    fn process_packet(buf: &[u8]) -> Vec<u8>;
}
