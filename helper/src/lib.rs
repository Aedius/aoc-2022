use easy_reader::EasyReader;
use std::{fs::File, io::Error};

pub trait InputReader {
    fn read(&mut self, path: &str) -> Result<(), Error> {
        let file = File::open(path)?;

        let mut reader = EasyReader::new(file)?;

        while let Some(line) = reader.next_line()? {
            self.add_line(line.as_str());
        }

        Ok(())
    }

    fn add_line(&mut self, line: &str);

    fn star1(self) -> String;
    fn star2(self) -> String;
}

#[macro_export]
macro_rules! aoc1{
    ($c:ty, $f:expr, $r: expr) =>{
        let mut container = <$c>::default();
        container.read(&*format!("./{}/test.txt", $f)).unwrap();
        assert_eq!(container.star1(), $r.to_string());

        let mut container = <$c>::default();
        container.read(&*format!("./{}/input.txt", $f)).unwrap();

        println!("Star 1 : {}", container.star1());
    }
}
#[macro_export]
macro_rules! aoc2{
    ($c:ty, $f:expr, $r: expr, $s: expr, $t: expr) =>{
        let mut container = <$c>::default();
        container.read(&*format!("./{}/test.txt", $f)).unwrap();
        assert_eq!(container.star1(), $r.to_string());

        let mut container = <$c>::default();
        container.read(&*format!("./{}/input.txt", $f)).unwrap();
        assert_eq!(container.star1(), $s.to_string());

        let mut container = <$c>::default();
        container.read(&*format!("./{}/test.txt", $f)).unwrap();
        assert_eq!(container.star2(), $t.to_string());

        let mut container = <$c>::default();
        container.read(&*format!("./{}/input.txt", $f)).unwrap();

        println!("Star 2 : {}", container.star2());

    }
}