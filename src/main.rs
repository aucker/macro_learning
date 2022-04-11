macro_rules! count_expr {
    () => (0);
    ($e:expr) => (1);
    // ($e0:expr, $e1:expr) => (1 + count_expr!($e1));
    // ($e0:expr, $e1:expr, $e2:expr) => (1 + count_expr!($e1, $e2));
    ($head:expr, $($tail:expr), *) => (1 + count_expr!($($tail), *));
}

macro_rules! recurrence {
    // ( a[n]:$sty:ty = $($inits:expr), + ;...; $recur:expr ) => {
    ( $seq:ident [$ind:ident]: $sty:ty = $($inits:expr), +; ...; $recur:expr ) => {
        // the `+` represents there is one or more repetitions here
        //
        /*
        let fib = recurrence![a[n]: u64 = 0, 1, ..., a[n-1] + a[n-2]];
        for e in fib.take(10) {println!("{}", e)}
         */
        {
            use std::ops::Index;

            const MEM_SIZE: usize = count_expr!($($inits), +);

        struct Recurrence {
            mem: [$sty; MEM_SIZE],
            pos: usize,
        }
        // This is the actual iterator type. `mem` will be the memo buffer to hold the last few values so the recurrence
        // can be computed. `pos` is to keep track of the value of `n`.

        struct IndexOffset<'a> {
            slice: &'a [$sty; MEM_SIZE],
            offset: usize,
        }
        impl<'a> Index<usize> for IndexOffset<'a> {
            type Output = $sty;

            fn index<'b>(&'b self, index: usize) -> &'b $sty {
                use std::num::Wrapping;

                let index = Wrapping(index);
                let offset = Wrapping(self.offset);
                let window = Wrapping(MEM_SIZE);

                let real_index = index - offset + window;
                &self.slice[real_index.0]
            }
        }

        /// Lifetime in Rust
        /// `'a` and `'b` are lifetime parameters that are used to track where a reference
        /// (i.e. a borrowed pointer to some data) is valid. In this case, `IndexOffset`
        /// borrows a reference to the iterator's data, so it needs to keep track of how long
        /// it's allowed to hold that reference for, using `'a`
        ///
        /// `'b` is used because the `Index::index` function (which is how subscript syntax is
        /// actually implemented) is *also* parameterized on a lifetime, on account of returning
        /// a borrowed reference. `'a` and `'b` are not necessarily the same thing in all cases.
        /// The borrow checker will make sure that even though we don't explicity relate `'a` and
        /// `'b` to one another, we don't accidentally violet memory safety.

        impl Iterator for Recurrence {
            type Item = $sty;
            fn next(&mut self) -> Option<$sty> {
                if self.pos < MEM_SIZE {
                    let next_val = self.mem[self.pos];
                    self.pos += 1;
                    Some(next_val)
                } else {
                    // // let a = /** */;
                    // let n = self.pos;
                    // let a = IndexOffset {slice: &self.mem, offset: n};
                    // let next_val = a[n-1] + a[n-2];

                    // we refactor this block
                    let next_val = {
                        // let n = self.pos;
                        let $ind = self.pos;
                        // let a = IndexOffset { slice: &self.mem, offset: n };
                        let $seq = IndexOffset { slice: &self.mem, offset: $ind };
                        $recur
                    };

                    // self.mem.TODO_shuffle_down_and_append(next_val.clone());
                    {
                        use std::mem::swap;

                        let mut swap_tmp = next_val;
                        for i in (0..MEM_SIZE).rev() {
                            swap(&mut swap_tmp, &mut self.mem[i]);
                        }
                    }

                    self.pos += 1;
                    Some(next_val)
                }
            }
        }
        // Recurrence {mem:[0, 1], pos: 0}
        Recurrence { mem: [$($inits), +], pos: 0 }
        }
    };
}

/// During compile time, the macro is indicated after the AST was established
fn main() {
    // println!("Hello, world!");
    // A example use is Fibonacci
    // let fib = recurrence![a[n] = 0, 1, ..., a[n-1] + a[n-2]];
    // for e in fib.take(10) { println!("{}", e) }
    // this is what the invocation should look like

    // let fib = recurrence![a[n]: u64 = 0, 1, ..., a[n-1] + a[n-2]];
    // note: this won't work, we will get local ambiguity error

    // let fib = recurrence![a[n]: u64 = 0, 1; ... ;a[n-1] + a[n-2]];
    // for e in fib.take(10) {
    //     println!("{}", e)
    // }

    for e in recurrence!(f[i]: f64 = 1.0; ...; f[i-1] * i as f64).take(10) {
        println!("{}", e);
    }
}
