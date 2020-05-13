pub mod engine_io_parser;
pub mod engine_io_server;
pub mod random_id_generator;
pub mod socket_io_adaptor;
pub mod socket_io_websocket;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
