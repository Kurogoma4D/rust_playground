fn main() {
    println!("Hello World!");
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{number:>width$}", number=1, width=6);
    println!("{number:>0width$}", number=1, width=6);

    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    struct Structure(i32);
    impl Structure {
        fn to_string(&self) -> String {
            format!("{}", self.0)
        }
    }

    println!("This struct `{}` won't print...", Structure(3).to_string());

    #[derive(Debug)]
    struct DebugPrintable(i32);

    #[derive(Debug)]
    struct Deep(DebugPrintable);

    println!("{:#?} months is a year.", DebugPrintable(3));
    println!("{:#?}", Deep(DebugPrintable(4)));
}