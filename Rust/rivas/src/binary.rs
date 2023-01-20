

mod binary {
    pub // file
    // store information about the input and output file
    struct file {
        name_in: String,
        name_out: String,
        cont_in: Vec<String>,
        cont_out: Vec<String>,
    }

    // Implementation of file
    // new, set_out, translate, write
    impl file {
        pub fn new(input_filename: &str) -> file {
            self.name_in = input_filename.to_owned();
            self.name_out = input_filename.to_owned().push(".rvb");
            file
        }

        pub fn set_out(output_filename: &str) {
            self.name_out = output_filename.to_owned();
        }

        pub fn translate() {

        }

        pub fn write() {

        }
    }
}
