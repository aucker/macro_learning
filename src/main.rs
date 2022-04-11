
macro_rules! recurrence {
    ( a[n]:$sty:ty = $($inits:expr), +, ..., $recur:expr ) => {
        // the `+` represents there is one or more repetitions here
        // 
        /*
        let fib = recurrence![a[n]: u64 = 0, 1, ..., a[n-1] + a[n-2]];
        for e in fib.take(10) {println!("{}", e)}
         */
    };
}

/// During compile time, the macro is indicated after the AST was established
fn main() {
    // println!("Hello, world!");
    // A example use is Fibonacci
    // let fib = recurrence![a[n] = 0, 1, ..., a[n-1] + a[n-2]];
    // for e in fib.take(10) { println!("{}", e) }
    // this is what the invocation should look like

    let fib = {
        use std::ops::Index;

        struct Recurrence {
            mem: [u64; 2],
            pos: usize,
        }
        // This is the actual iterator type. `mem` will be the memo buffer to hold the last few values so the recurrence 
        // can be computed. `pos` is to keep track of the value of `n`.

        struct IndexOffset<'a> {
            slice: &'a [u64; 2],
            offset: usize,
        }
        impl<'a> Index<usize> for IndexOffset<'a> {
            type Output = u64;

            fn index<'b>(&'b self, index: usize) -> &'b u64 {
                use std::num::Wrapping;

                let index = Wrapping(index);
                let offset = Wrapping(self.offset);
                let window = Wrapping(2);

                let real_index = index - offset + window;
                &self.slice[real_index.0]
            }
        }

        impl Iterator for Recurrence {
            type Item = u64;
            fn next(&mut self) -> Option<u64> {
                if self.pos < 2 {
                    let next_val = self.mem[self.pos];
                    self.pos += 1;
                    Some(next_val)
                } else {
                    // let a = /** */;
                    let n = self.pos;
                    let a = IndexOffset {slice: &self.mem, offset: n};                   
                    let next_val = a[n-1] + a[n-2];

                    // self.mem.TODO_shuffle_down_and_append(next_val.clone());
                    {
                        use std::mem::swap;

                        let mut swap_tmp = next_val;
                        for i in (0..2).rev() {
                            swap(&mut swap_tmp, &mut self.mem[i]);
                        }
                    }

                    self.pos += 1;
                    Some(next_val)
                }
            }
        }
        Recurrence {mem:[0, 1], pos: 0}
    };
    for e in fib.take(10) {println!("{}", e)}
}
