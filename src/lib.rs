use std::io::{Error, Read, Write};

/*
    Parse the /etc/rc.local file.
    Possibly could be used to parse others as well
*/

/// A very basic representation of a shell script. There is an interpreter, some comments
/// and a list of commands
#[derive(Debug)]
pub struct ShellScript {
    ///the interpreter to use
    pub interpreter: String,
    /// Any comments here will be joined with newlines when written back out
    pub comments: Vec<String>,
    /// Any commands here will be joined with newlines when written back out
    pub commands: Vec<String>,
}

impl ShellScript {
    /// Write the run control struct back out to a file
    pub fn write<T: Write>(&self, f: &mut T) -> Result<usize, Error> {
        let mut bytes_written = 0;
        bytes_written += f.write(format!("{}\n", self.interpreter).as_bytes())?;
        bytes_written += f.write(self.comments.join("\n").as_bytes())?;
        bytes_written += f.write(&"\n".as_bytes())?;
        bytes_written += f.write(self.commands.join("\n").as_bytes())?;
        bytes_written += f.write(&"\n".as_bytes())?;
        Ok(bytes_written)
    }
}

#[test]
fn test_parse() {
    let shell_script = r#"
#!/bin/sh -e
#
# rc.local
#
# This script is executed at the end of each multiuser runlevel.
# Make sure that the script will "exit 0" on success or any other
# value on error.
#
# In order to enable or disable this script just change the execution
# bits.
#
# By default this script does nothing.

exit 0
    "#
            .as_bytes();
    let mut c = ::std::io::Cursor::new(&shell_script);
    let result = parse(&mut c).unwrap();
    println!("Result: {:?}", result);

    let mut buff: Vec<u8> = Vec::new();
    let result2 = result.write(&mut buff).unwrap();
}

pub fn parse<T: Read>(f: &mut T) -> Result<ShellScript, Error> {
    let mut comments: Vec<String> = Vec::new();
    let mut commands: Vec<String> = Vec::new();
    let mut interpreter = String::new();

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    for line in buf.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("#!") {
            interpreter = trimmed.to_string();
        } else if trimmed.starts_with("#") {
            comments.push(trimmed.to_string());
        } else {
            if !trimmed.is_empty() {
                commands.push(trimmed.to_string());
            }
        }
    }

    Ok(ShellScript {
           interpreter: interpreter,
           comments: comments,
           commands: commands,
       })
}
