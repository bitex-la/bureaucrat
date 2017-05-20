// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {
    errors {
        InvalidCbuFormat
        InvalidCbuChecksum
        InvalidCuitFormat
        InvalidCuitChecksum
        InvalidCuitKind

        //Commented
        InvalidRequestFormat
        InvalidRequestEmpty
        InvalidRequestResource
    }

    foreign_links {
        InvalidRequestString(::std::str::Utf8Error);
    }
}
