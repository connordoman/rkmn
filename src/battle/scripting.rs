#[derive(Clone, Copy, Debug)]
pub struct BattleScriptStack {
    ptr: *mut [u8; 8],
    size: u8,
}

impl BattleScriptStack {
    pub fn new() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            size: 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BattleCallbacksStack {
    func_stack: [fn(); 8],
    size: u8,
}

impl BattleCallbacksStack {
    pub fn new() -> Self {
        Self {
            func_stack: [|| {}; 8],
            size: 0,
        }
    }
}
