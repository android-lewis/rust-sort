use clap::arg_enum;

arg_enum! {
    #[derive(Debug)]
    pub enum Sort {
        Quick,
        Bubble,
        Merge,
        Insertion,
        Selection,
        Heap,
        Radix,
        Bucket
    }
}
