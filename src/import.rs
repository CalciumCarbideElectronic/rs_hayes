
extern {
    pub fn uart_send(data: *const  u8, size: usize)->i32;
    pub fn uart_recv(buffer: *mut u8) ->();
}