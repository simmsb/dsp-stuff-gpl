pub struct Scanl<I, St, F> {
    iter: I,
    f: F,
    state: St,
}

pub struct ScanlN<I, St, F, const N: usize> {
    iter: I,
    f: F,
    state: [St; N],
}

pub trait IterScanl: Iterator {
    fn scanl<St, F>(self, initial: St, f: F) -> Scanl<Self, St, F>
    where
        Self: Sized,
        St: Copy,
        F: Fn(St, Self::Item) -> St,
    {
        Scanl {
            iter: self,
            f,
            state: initial,
        }
    }

    fn scanln<St, F, const N: usize>(self, initial: [St; N], f: F) -> ScanlN<Self, St, F, N>
    where
        Self: Sized,
        St: Copy,
        F: Fn([St; N], Self::Item) -> St,
    {
        ScanlN {
            iter: self,
            f,
            state: initial,
        }
    }
}

impl<T: Iterator> IterScanl for T {}

impl<I: Iterator, St, F> Iterator for Scanl<I, St, F>
where
    St: Copy,
    F: Fn(St, I::Item) -> St,
{
    type Item = St;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let v = self.iter.next()?;

        self.state = (self.f)(self.state, v);

        Some(self.state)
    }
}

impl<I: Iterator, St, F, const N: usize> Iterator for ScanlN<I, St, F, N>
where
    St: Copy,
    F: Fn([St; N], I::Item) -> St,
{
    type Item = St;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let v = self.iter.next()?;

        let v = (self.f)(self.state, v);
        self.state.rotate_left(1);
        *self.state.last_mut().unwrap() = v;

        Some(v)
    }
}
