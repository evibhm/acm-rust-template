use std::io::{self, BufRead, Write};

fn solve(mut input: impl BufRead, mut output: impl Write) {
    let mut buf = String::new();
    input.read_line(&mut buf).expect("input");
    let arr: Vec<usize> = buf
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("parse"))
        .collect();
    let a = arr[0];
    let b = arr[1];

    let ans = a + b;

    write!(output, "{}", ans).expect("output");
}

fn main() -> io::Result<()> {
    let input = io::stdin();
    let output = io::stdout();
    solve(input.lock(), &output);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use io::{BufReader, BufWriter, Read};

    use super::*;

    fn test_by_file(id: usize) {
        let input_fname = format!("./data/{}.in", id);
        let output_fname = format!("./data/{}.out", id);
        let answer_fname = format!("./data/{}.ans", id);
        let input = File::open(input_fname).expect("input file");
        let output = File::create(output_fname.clone()).expect("output file");
        solve(BufReader::new(input), BufWriter::new(output));

        let mut output = File::open(output_fname).expect("output file");
        let mut answer = File::open(answer_fname).expect("answer file");
        let mut output_string = String::new();
        let mut answer_string = String::new();
        output
            .read_to_string(&mut output_string)
            .expect("read output file");
        answer
            .read_to_string(&mut answer_string)
            .expect("read answer file");

        assert_eq!(
            output_string.split_whitespace().collect::<Vec<_>>(),
            answer_string.split_whitespace().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test1() {
        test_by_file(1)
    }
}
