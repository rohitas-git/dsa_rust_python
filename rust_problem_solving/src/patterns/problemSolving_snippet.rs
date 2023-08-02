// IMPL:parse_line
macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
  })}
  
  // IMPL:parse_list
  macro_rules! parse_list { ($t: ty) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let list: Vec<$t> = line.split_whitespace()
    .map(|w| w.parse::<$t>().unwrap()).collect(); list
  })}
  
  // IMPL:parse_n_lines
  macro_rules! parse_n_lines { ($n :expr; $($t :ty),+) => ({
  use std::io::BufRead; let stdin = std::io::stdin();
  let vec = stdin.lock().lines().take($n).map(|l| {
    let l = l.unwrap(); let mut iter = l.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
  }).collect::<Vec<_>>(); vec })}
  
  // IMPL:for_lines
  // usage
  // for_lines! {(x :usize); {
  //   /* do something */
  // }}
  macro_rules! for_lines {
  ($ts:tt; $b:block) => ({ use std::io::BufRead;
    let n = parse_line!(usize); let stdin = std::io::stdin();
    for l in stdin.lock().lines().take(n) { for_lines_parse!(l, $b, $ts); }
  })}
  macro_rules! for_lines_parse {
  ($l:expr, $b:block, ($($x:ident : $t:ty),+)) => ({
    let line = $l.unwrap(); let mut iter = line.split_whitespace();
    $(let $x = iter.next().unwrap().parse::<$t>().unwrap();)+ $b;
  })}
  
  // IMPL:max
  macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::max($x, max!($($z),*)));
  }
  
  // IMPL:min
  macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::min($x, min!($($z),*)));
  }
  
  // IMPL:parse_line_
  // like parse_line! but able to parse as optionals
  // e.g. let (x, oy) = parse_line_!(!usize, ?u8);
  macro_rules! parse_word {
      ($w :expr, !, $t: ty) => ({ $w.unwrap().parse::<$t>().unwrap()});
      ($w :expr, ?, $t: ty) => ({ $w.and_then(|s| s.parse::<$t>().ok()) });
  }
  macro_rules! parse_line_ { ($($o: tt $t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(parse_word!(iter.next(), $o, $t)),+)
  })}
  
  // IMPL:cut
  use std::iter::{once, Once, Chain};
  trait CutInFrontIteratorExt: Iterator {
  fn cut<T>(self, x :T) -> Chain<Once<T>, <Self as std::iter::IntoIterator>::IntoIter>
  where Self :IntoIterator<Item = T> + std::marker::Sized { once(x).chain(self) }}
  impl<I: Iterator> CutInFrontIteratorExt for I {}
  
  // IMPL:repeat_while
  #[must_use = "iterators are lazy and do nothing unless consumed"]
  pub struct RepeatWhile<F> { repeater: F }
  impl<A, F: FnMut() -> Option<A>> Iterator for RepeatWhile<F>
  { type Item = A; #[inline] fn next(&mut self) -> Option<A> { (self.repeater)() } }
  #[inline] fn repeat_while<A, F>(repeater: F) -> RepeatWhile<F>
  where F : FnMut() -> Option<A> { RepeatWhile { repeater } }
  
  // IMPL:iterate
  // saddly, iterate is not part of rust, https://github.com/rust-lang/rust/commit/669d1c
  // following code stolen from itertools/0.8.0 package
  #[must_use = "iterators are lazy and do nothing unless consumed"]
  pub struct Iterate<St, F> { st: St, f: F }
  impl<St, F> Iterator for Iterate<St, F> where F: FnMut(&St) -> Option<St>
  { type Item = St; #[inline] fn next(&mut self) -> Option<Self::Item> {
  (self.f)(&self.st).map(|nst| std::mem::replace(&mut self.st, nst)) }}
  pub fn iterate<St, F>(ini: St, f: F) -> Iterate<St, F>
  where F: FnMut(&St) -> Option<St> { Iterate { st: ini, f: f } }
  
  
  // IMPL:inspect
  fn inspect<T :std::fmt::Debug>(x :T) -> T { println!("i> {:?}", &x); x }  
  
  // IMPL:ins IMPL:pect IMPL:DebugInspect
  trait DebugInspect: std::marker::Sized + std::fmt::Debug {
    fn ins(self) -> Self { self.ins_("i> ") }
    fn ins_(self, s :&str) -> Self { println!("{}{:?}", s, &self); self }
    fn pect(self) -> Self { self.pect_(" ") }
    fn pect_(self, s :&str) -> Self { print!("{:?}{}", &self, s); self }
  } impl<T: std::marker::Sized + std::fmt::Debug> DebugInspect for T {}
  fn nl() { println!(""); }
  
  
  // scanl
  //mimics haskell's scanl
  #[must_use = "iterators are lazy and do nothing unless consumed"]
  struct Scanl<I, St, F> { iter :I, f :F, state :St }
  impl<I, St, F> Iterator for Scanl<I, St, F>
  where I :Iterator, St :Copy, F :Fn(St, I::Item) -> St { type Item = St;
  #[inline] fn next(&mut self) -> Option<St> {
      if let Some(x) = self.iter.next() {
        self.state = (self.f)(self.state, x);
        Some(self.state)
      } else {None}
    }
  }
  
  fn scanl<I, St, F>(iter: I, state: St, f: F) -> Scanl<I, St, F> {
    Scanl { iter, state, f }
  }
  
  use std::iter::repeat_with;
  fn digits(mut x :usize) -> usize {
    repeat_with(||{x/=10; x}).take_while(|&a| a > 0).count() + 1
  }
  
  
  struct Collaps<I, T> {
    orig :I,
    head :Option<T>,
  }
  
  impl<I :Iterator> Iterator for Collaps<I, I::Item>
    where I::Item :std::cmp::PartialEq {
    type Item = (usize, I::Item);
  
    fn next(&mut self) -> Option<Self::Item> {
      if self.head.is_none() { return None; }
      // https://stackoverflow.com/a/52031060
      let mut ret = (1, self.head.take().unwrap());
  
      loop { match self.orig.next() {
        // https://stackoverflow.com/a/29885937
        Some(ref x) if *x == ret.1 => ret.0 += 1,
        None => { self.head = None; break; }
        some => { self.head = some; break; }
      }}
      return Some(ret);
    }
  }
  
  trait MyIterExt: Sized {
    type T;
    fn collaps(self) -> Collaps<Self, Self::T>;
  }
  
  impl<I: Iterator> MyIterExt for I {
    type T = I::Item;
    fn collaps(mut self) -> Collaps<I, I::Item> {
      let ret = self.next();
      Collaps { orig: self, head: ret }
    }
  }
  
  
  // IMPL:compact_in
  use std::io::BufRead;
  let mut buf = Vec::new();
  let stdin = std::io::stdin()
  let mut bi = std::io::BufReader::new(stdin.lock());
  let x = unsafe {
      buf.clear(); bi.read_until(b' ', &mut buf).unwrap();
      std::str::from_utf8_unchecked(&buf[..]).trim().parse::<i32>().unwrap()
  };
  
  // IMPL:fast_out
  use std::io::Write;
  let stdout = std::io::stdout();
  let mut bo = std::io::BufWriter::new(stdout.lock());
  writeln!(bo, "{}", i).unwrap();
  
  // IMPL:binsearch_range
  // returns closed range of values equal to x.
  // if (st,en) = binsearch_range(ls,x), then all elems in ls[st..=en] is x.
  // otherwise, returns a position that where x can be inserted while keeping the order.
  // (Vec methods returns a index randomly selected within the range)
  fn binsearch_range<T :Ord>(ls :&Vec<T>, x :&T) -> Result<(usize,usize), usize> {
    if ls.len() == 0 || ls[ls.len()-1] < *x { return Err(0)};
    let st = { let (mut s, mut e) = (0, ls.len()-1);
      while s < e { let m = (s+e)/2; if *x <= ls[m] {e=m} else {s=m+1} } s };
    let en = { let (mut s, mut e) = (st, ls.len()-1);
      while s < e { let m = (s+e)/2 + 1; if *x < ls[m] {e=m-1} else {s=m} } e };
    if ls[st] == *x { Ok((st,en)) } else { Err(en) }
  }
  
  // IMPL:is_intercept
  // check if two line segment (p1,p2), (p3,p4) intercepts
  fn is_intercept(p1:(i32,i32), p2:(i32,i32), p3:(i32,i32), p4:(i32,i32)) -> bool {
    let ccw = |p:(i32,i32),q:(i32,i32),r:(i32,i32)| ((q.1-p.1)*(r.0-q.0)-(q.0-p.0)*(r.1-q.1)).signum();
    let (p1p2, p3p4) = (ccw(p1,p2,p3)*ccw(p1,p2,p4), ccw(p3,p4,p1)*ccw(p3,p4,p2));
    if p1p2 != 0 || p3p4 != 0 { p1p2 <= 0 && p3p4 <= 0 } else {
      let ((p1,p2),(p3,p4)) = (if p1>p2{(p2,p1)}else{(p1,p2)}, if p3>p4{(p4,p3)}else{(p3,p4)});
      p3 <= p2 && p1 <= p4
    }
  }