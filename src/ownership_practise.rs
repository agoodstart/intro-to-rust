pub fn func() {
    let sentence = "He figured a few sticks of dynamite were easier than a fishing pole to catch fish.";

    let v: Vec<&str> = sentence.split(' ').collect();

    println!("{:?}", v);
}