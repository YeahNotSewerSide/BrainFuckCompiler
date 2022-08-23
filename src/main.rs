use std::io::{self, Read, Write};

macro_rules! compile_brainfuck {
    {} => {};

    {
        $stack:tt,
        $pointer:tt,
    } => {};

    // some people actually use () in code
    {
        $stack:tt,
        $pointer:tt,
        ($($inner:tt)*)
        $($tail:tt)*
    } => {
        compile_brainfuck!($stack,$pointer,$($inner)*);
        compile_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        +
        $($tail:tt)*
    } => {
        $stack[$pointer] += 1;
        compile_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        -
        $($tail:tt)*
    } => {
        $stack[$pointer] -= 1;
        compile_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        ->
        $($tail:tt)*
    } => {
        $stack[$pointer] -= 1;
        $pointer += 1;
        compile_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        <-
        $($tail:tt)*
    } => {
        $pointer -= 1;
        $stack[$pointer] -= 1;
        compile_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        <
        $($tail:tt)*
    } => {
        $pointer -= 1;
        compile_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        <<
        $($tail:tt)*
    } => {
        $pointer -= 2;
        compile_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        >
        $($tail:tt)*
    } => {
        $pointer += 1;
        compile_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        >>
        $($tail:tt)*
    } => {
        $pointer += 2;
        compile_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        .
        $($tail:tt)*
    } => {
        io::stdout().write(&[$stack[$pointer] as u8]).unwrap();
        compile_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        ..
        $($tail:tt)*
    } => {
        io::stdout().write(&[$stack[$pointer] as u8,$stack[$pointer] as u8]).unwrap();
        compile_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        ,
        $($tail:tt)*
    } => {
        io::stdout().flush().unwrap();
        let input: i8 = std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as i8).unwrap();
        $stack[$pointer] = input;
        compile_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        [
        $($inner:tt)*
        ]
        $($tail:tt)*
    } => {
        while $stack[$pointer] != 0{
            compile_brainfuck!($stack,$pointer,$($inner)*);
        }
        compile_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        cmp $stack_size:expr,$($tail:tt)*
    } => {
        fn main() {
            let mut stack = vec![0i8;$stack_size];
            let mut pointer:usize = 0;
            compile_brainfuck!(stack,pointer,$($tail)*);
            io::stdout().flush().unwrap();
        }
    };

}

compile_brainfuck!(cmp 30000, >>>->+>+++++>(++++++++++)[[>>>+<<<-]>+++++>+>>+[<<+>>>>>+<<<-]<-]>>>>[
  [>>>+>+<<<<-]+++>>+[<+>>>+>+<<<-]>>[>[[>>>+<<<-]<]<<++>+>>>>>>-]<-
]+++>+>[[-]<+<[>+++++++++++++++++<-]<+]>>[
  [+++++++++.-------->>>]+[-<<<]>>>[>>,----------[>]<]<<[
    <<<[
      >--[<->>+>-<<-]<[[>>>]+>-[+>>+>-]+[<<<]<-]>++>[<+>-]
      >[[>>>]+[<<<]>>>-]+[->>>]<-[++>]>[------<]>+++[<<<]>
    ]<
  ]>[
    -[+>>+>-]+>>+>>>+>[<<<]>->+>[
      >[->+>+++>>++[>>>]+++<<<++<<<++[>>>]>>>]<<<[>[>>>]+>>>]
      <<<<<<<[<<++<+[-<<<+]->++>>>++>>>++<<<<]<<<+[-<<<+]+>->>->>
    ]<<+<<+<<<+<<-[+<+<<-]+<+[
      ->+>[-<-<<[<<<]>[>>[>>>]<<+<[<<<]>-]]
      <[<[<[<<<]>+>>[>>>]<<-]<[<<<]]>>>->>>[>>>]+>
    ]>+[-<<[-]<]-[
      [>>>]<[<<[<<<]>>>>>+>[>>>]<-]>>>[>[>>>]<<<<+>[<<<]>>-]>
    ]<<<<<<[---<-----[-[-[<->>+++<+++++++[-]]]]<+<+]>
  ]>>
]);
