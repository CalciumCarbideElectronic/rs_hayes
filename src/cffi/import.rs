extern {
    pub fn uart_send(data: *const  u8, size: usize)->i32;
    pub fn strlen(p: *const u8)->u32;
}