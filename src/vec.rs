pub fn _run() {
    let mut src1: Vec<char> = vec!['j','{','"','i','m','m','y','"','}'];
    // to String
    let string1: String = src1.iter().collect::<String>();
    // to str
    let str1: &str = &src1.iter().collect::<String>();
    // to vec of byte
    let byte1 = src1.iter_mut().map(|c| *c as u8).collect::<Vec<_>>();
    println!("Vec<char>:{:?} | String:{:?}, str:{:?}, Vec<u8>:{:?}", src1, string1, str1, byte1);
  
}