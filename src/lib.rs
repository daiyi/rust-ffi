#[link(name = "math")]
extern "C" {
    pub fn fmax(x:f64, y:f64) -> f64;
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn it_works() {
        let m = unsafe { fmax(3.0, 4.0) };

        assert!(m == 4.0);
    }
}
