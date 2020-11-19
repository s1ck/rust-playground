#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Read, ErrorKind};
    use std::{io, fs};

    #[test]
    fn crash_and_burn() {
        panic!("crash and burn");
    }

    #[test]
    fn array_index() {
        let a = vec![1, 2, 3];
        a[42];
    }

    #[test]
    fn fopen() {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(file) => file,
                    Err(error) => panic!("Problem opening the file: {:?}", error),
                }
                other_error => panic!("Problem opening the file: {:?}", other_error),
            }
        };

        let mut buf = String::new();
        let x = f.read_to_string(&mut buf);
        buf.lines().for_each(|l| println!("{}", l));
    }

    #[test]
    fn fopen_unwrap() {
        let f = File::open("hello.txt");
        let mut f = f.unwrap_or_else(|error| if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| panic!("Problem opening the file: {:?}", error))
        } else {
            panic!("Problem opening the file: {:?}", error)
        });

        let mut buf = String::new();
        let x = f.read_to_string(&mut buf);
        buf.lines().for_each(|l| println!("{}", l));
    }

    #[test]
    fn fopen_expect() {
        let f = File::open("foobar").expect("File foobar does not exist");
    }

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("username.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(err) => return Err(err),
        };
        let mut buffer = String::new();

        match f.read_to_string(&mut buffer) {
            Ok(_) => Ok(buffer),
            Err(err) => Err(err)
        }
    }

    fn read_username_from_file_short() -> Result<String, io::Error> {
        let mut f = File::open("username.txt")?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        Ok(buffer)

    }

    fn read_username_from_file_shortest() -> Result<String, io::Error> {
        fs::read_to_string("username.txt")
    }

}
