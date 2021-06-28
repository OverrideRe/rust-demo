
/// 裸指针
mod test0 {

    unsafe fn dangeraous(num: i32) {
        println!("num : {}", num);
    }

    #[test]
    fn test0() {
        let mut num = 5;

        let r1 = &num as *const i32;    // 不可变裸指针
        let r2 = &mut num as *mut i32;    // 可变裸指针

        let address = 0x012345usize;
        let r3 = address as *const i32; // 可能是无效地址转换成裸指针


        // 编译会报错，会说这种操作是unsafe的，需要放入unsafe函数或者代码块中
        // println!("r1 is {}", *r1);       
        // dangeraous(r1);                  

        unsafe {
            println!("r1 is {}", *r1);
            dangeraous(*r2);
        }
    }
}

/// 创建不安全代码的安全抽象
mod test1 {
    use core::slice;

    #[test]
    fn test1() {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        // let (a, b) = r.split_at_mut(3);
        let (a, b) = split_at_mut0(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    fn split_at_mut0(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        // 获取slice的裸指针
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                // 指针地址操作，前一个参数是地址起始位置，后一个参数是长度
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid)
            )
        }
    }

    // fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = slice.len();

    //     assert!(mid <= len);

    //     // 会报错：slice被借用了两次。尽管我们知道是借用了slice没有重叠的不同部分，但是rust不知道
    //     (&mut slice[..mid], &mut slice[mid..])
    // }
}



/// 使用extern函数调用外部（其它语言编写）的代码
mod test2 {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    #[test]
    fn test() {
        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

    // 提供给其它语言调用的接口，no_mangle告诉编译器不要将方法名打乱，extern后面跟着指定的ABI(application binary interface)
    #[no_mangle]
    extern "C" fn call_from_c() {
        println!("Just called a Rust function from C");
    }
}

/// 访问或修改可变静态变量
mod test3 {
    
}
