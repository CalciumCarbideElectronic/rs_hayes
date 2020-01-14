
#[cfg(not(test))]
extern {
    pub fn uart_send(data: *const  u8, size: usize)->i32;
    pub fn strlen(p: *const u8)->u32;
}

#[cfg(test)]
pub fn uart_send(data: *const  u8, size: usize)->i32{
    0
}
#[cfg(test)]
extern{
    pub fn strlen(p: *const u8)->u32;

}
