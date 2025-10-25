pub fn map_collect<T, U, F>(_parallel: bool, items: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    items.iter().map(|item| f(item)).collect()
}

pub fn flat_map_collect<T, U, F>(_parallel: bool, items: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> Vec<U>,
{
    items.iter().flat_map(|item| f(item).into_iter()).collect()
}

pub fn for_each_index_chunk<T, F>(
    _parallel: bool,
    indices: &[usize],
    values: &mut [T],
    chunk_len: usize,
    f: F,
)
where
    F: Fn(usize, &mut [T]),
{
    indices
        .iter()
        .zip(values.chunks_mut(chunk_len))
        .for_each(|(idx, chunk)| f(*idx, chunk));
}
