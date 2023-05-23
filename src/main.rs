#![recursion_limit = "10000"]
#[allow(unused_imports)]
use std::io::{self, Read, Write};

#[allow(unused_macros)]
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
            let mut stack = vec![0i8;$stack_size].into_boxed_slice();
            let mut pointer:usize = 0;
            compile_brainfuck!(stack,pointer,$($tail)*);
            io::stdout().flush().unwrap();
        }
    };

}

#[allow(unused_macros)]
macro_rules! compile_unsafe_brainfuck {
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
        compile_unsafe_brainfuck!($stack,$pointer,$($inner)*);
        compile_unsafe_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        +
        $($tail:tt)*
    } => {
        let cell = unsafe{$stack.get_unchecked_mut($pointer)};
        *cell = *cell + 1;
        compile_unsafe_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        -
        $($tail:tt)*
    } => {
        let cell = unsafe{$stack.get_unchecked_mut($pointer)};
        *cell = *cell - 1;
        compile_unsafe_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        ->
        $($tail:tt)*
    } => {
        let cell = unsafe{$stack.get_unchecked_mut($pointer)};
        *cell = *cell - 1;
        $pointer += 1;
        compile_unsafe_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        <-
        $($tail:tt)*
    } => {
        $pointer -= 1;
        let cell = unsafe{$stack.get_unchecked_mut($pointer)};
        *cell = *cell - 1;
        compile_unsafe_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        <
        $($tail:tt)*
    } => {
        $pointer -= 1;
        compile_unsafe_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        <<
        $($tail:tt)*
    } => {
        $pointer -= 2;
        compile_unsafe_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        >
        $($tail:tt)*
    } => {
        $pointer += 1;
        compile_unsafe_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        >>
        $($tail:tt)*
    } => {
        $pointer += 2;
        compile_unsafe_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        .
        $($tail:tt)*
    } => {
        let _ = io::stdout().write(&[unsafe{*$stack.get_unchecked($pointer)} as u8]);
        compile_unsafe_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        ..
        $($tail:tt)*
    } => {
        let symb = unsafe{*$stack.get_unchecked($pointer)};
        let _ = io::stdout().write(&[symb as u8,symb as u8]);
        compile_unsafe_brainfuck!($stack,$pointer,$($tail)*);
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
        unsafe{*$stack.get_unchecked_mut($pointer) = input;}
        compile_unsafe_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        $stack:tt,
        $pointer:tt,
        [
        $($inner:tt)*
        ]
        $($tail:tt)*
    } => {
        while unsafe{*$stack.get_unchecked($pointer)} != 0{
            compile_unsafe_brainfuck!($stack,$pointer,$($inner)*);
        }
        compile_unsafe_brainfuck!($stack,$pointer,$($tail)*);
    };

    {
        cmp $stack_size:expr,$($tail:tt)*
    } => {
        fn main() {
            let mut stack = vec![0i8;$stack_size].into_boxed_slice();
            let mut pointer:usize = 0;

            // let stdout = std::io::stdout();
            // let lock = stdout.lock();
            // let mut buf = std::io::BufWriter::new(lock);
            compile_unsafe_brainfuck!(stack,pointer,$($tail)*);
            let _ = io::stdout().flush();
        }
    };

}

compile_unsafe_brainfuck!(cmp 30000, >>>>+>+++>+++>>>>>+++[
>,+>++++[>++++<-]>[<<[-[->]]>[<]>-]<<[
>+>+>>+>+[<<<<]<+>>[+<]<[>]>+[[>>>]>>+[<<<<]>-]+<+>>>-[
<<+[>]>>+<<<+<+<--------[
<<-<<+[>]>+<<-<<-[
<<<+<-[>>]<-<-<<<-<----[
<<<->>>>+<-[
<<<+[>]>+<<+<-<-[
<<+<-<+[>>]<+<<<<+<-[
<<-[>]>>-<<<-<-<-[
<<<+<-[>>]<+<<<+<+<-[
<<<<+[>]<-<<-[
<<+[>]>>-<<<<-<-[
>>>>>+<-<<<+<-[
>>+<<-[
<<-<-[>]>+<<-<-<-[
<<+<+[>]<+<+<-[
>>-<-<-[
<<-[>]<+<++++[<-------->-]++<[
<<+[>]>>-<-<<<<-[
<<-<<->>>>-[
<<<<+[>]>+<<<<-[
<<+<<-[>>]<+<<<<<-[
>>>>-<<<-<-
]]]]]]]]]]]]]]]]]]]]]]>[>[[[<<<<]>+>>[>>>>>]<-]<]>>>+>>>>>>>+>]<
]<[-]<<<<<<<++<+++<+++[
[>]>>>>>>++++++++[<<++++>++++++>-]<-<<[-[<+>>.<-]]<<<<[
-[-[>+<-]>]>>>>>[.[>]]<<[<+>-]>>>[<<++[<+>--]>>-]
<<[->+<[<++>-]]<<<[<+>-]<<<<
]>>+>>>--[<+>---]<.>>[[-]<<]<
]);
