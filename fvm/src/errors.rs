//! Errors related specifically to the VM functionality

error_chain!{
    errors {
        InvalidToolchainName(t: String) {
            description("Invalid Memory index")
            display("Invalid Memory Index: '{}'", t)
        }
    }
}
