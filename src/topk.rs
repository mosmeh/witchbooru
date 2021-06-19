use std::collections::BinaryHeap;

pub trait IteratorExt: Iterator
where
    <Self as Iterator>::Item: Ord,
{
    fn topk(self, k: usize) -> TopK<<Self as Iterator>::Item>
    where
        Self: Sized,
    {
        TopK {
            heap: self.collect(),
            remaining: k,
        }
    }
}

impl<I> IteratorExt for I
where
    I: Iterator,
    <I as Iterator>::Item: Ord,
{
}

pub struct TopK<T: Ord> {
    heap: BinaryHeap<T>,
    remaining: usize,
}

impl<T: Ord> Iterator for TopK<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            self.heap.pop()
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.heap.len().min(self.remaining);
        (exact, Some(exact))
    }
}

#[cfg(test)]
mod tests {
    use super::IteratorExt;

    #[test]
    fn works() {
        let x = &[5, 2, 9, 6, 4];
        assert!(x.iter().topk(0).next().is_none());
        assert_eq!(x.iter().topk(3).copied().collect::<Vec<_>>(), &[9, 6, 5]);
        assert_eq!(
            x.iter().topk(10).copied().collect::<Vec<_>>(),
            &[9, 6, 5, 4, 2]
        );
    }
}
