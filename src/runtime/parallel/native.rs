use rayon::prelude::*;

pub fn map_collect<T, U, F>(parallel: bool, items: &[T], f: F) -> Vec<U>
where
    T: Sync,
    U: Send,
    F: Fn(&T) -> U + Sync + Send,
{
    if parallel {
        items.par_iter().map(|item| f(item)).collect()
    } else {
        items.iter().map(|item| f(item)).collect()
    }
}

pub fn flat_map_collect<T, U, F>(parallel: bool, items: &[T], f: F) -> Vec<U>
where
    T: Sync,
    U: Send,
    F: Fn(&T) -> Vec<U> + Sync + Send,
{
    if parallel {
        items
            .par_iter()
            .flat_map(|item| f(item).into_par_iter())
            .collect()
    } else {
        items.iter().flat_map(|item| f(item).into_iter()).collect()
    }
}

pub fn for_each_index_chunk<T, F>(
    parallel: bool,
    indices: &[usize],
    values: &mut [T],
    chunk_len: usize,
    f: F,
)
where
    T: Send,
    F: Fn(usize, &mut [T]) + Sync + Send,
{
    if parallel {
        indices
            .par_iter()
            .zip(values.par_chunks_mut(chunk_len))
            .for_each(|(idx, chunk)| f(*idx, chunk));
    } else {
        indices
            .iter()
            .zip(values.chunks_mut(chunk_len))
            .for_each(|(idx, chunk)| f(*idx, chunk));
    }
}
