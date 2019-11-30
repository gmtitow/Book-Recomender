
/**
 * mode : 
 *          DEBUG - 0
 *          RELEASE - 1
 */
pub struct Manager {
    mode : i32
}

impl Manager {

    pub fn new(mode: i32)->Manager{
        Manager {
            mode: mode
        }
    }

    pub fn setDebug(&mut self){
        self.mode = 0;
    }

    pub fn setRelease(&mut self){
        self.mode = 1;
    }

    pub fn isDebug(&self)-> bool {
        self.mode == 0
    }

    pub fn isRelease(&self)-> bool {
        self.mode == 1
    }
}