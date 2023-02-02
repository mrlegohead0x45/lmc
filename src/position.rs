// use getset::Getters;

// #[derive(Debug, Getters)]
// pub(crate) struct Position {
//     line: usize,
//     column: usize,
//     #[getset(get = "pub(crate)")]
//     filename: String,
// }

// impl Position {
//     fn new(filename: &str) -> Self {
//         Self {
//             line: 1,
//             column: 1,
//             filename: filename.to_string(),
//         }
//     }

//     pub(crate) fn advance(&mut self, c: char) {
//         if c == '\n' {
//             self.line += 1;
//             self.column = 1;
//         } else {
//             self.column += 1;
//         }
//     }
// }
