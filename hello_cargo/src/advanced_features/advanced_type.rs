
mod test0 {

    mod test0_0 {
        use std::io::Error;
        use std::fmt;
    
        pub trait Write {
            fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
            fn flush(&mut self) -> Result<(), Error>;
    
            fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
            fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
        }
    }

    mod test0_1 {
        use std::fmt;

        // 取别名，减少重复，让代码简写易懂
        type Result<T> = std::result::Result<T, std::io::Error>;
    
        pub trait Write {
            fn write(&mut self, buf: &[u8]) -> Result<usize>;
            fn flush(&mut self) -> Result<()>;
    
            fn write_all(&mut self, buf: &[u8]) -> Result<()>;
            fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
        }
    }


    #[test]
    fn test() {
        
    }
}


/// 从不返回的 never Type
mod test1 {

    fn main() {
        // 因为continue是 `!`类型，所以编译器判断Err分支不返回值，那么guess就是u32类型
        let guess = "3";
        loop {
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            break;
        }
    }

}