use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("buffer has no free space")]
    NoSpaceLeft,
}

type Result<T> = std::result::Result<T, MyError>;

struct RingBuffer {
    read_idx: usize,
    write_idx: usize,
    size: usize,
    data: Vec<u8>,
}

fn create(size: usize) -> RingBuffer {
    RingBuffer {
        read_idx: 0,
        write_idx: 0,
        size: 0,
        data: Vec::<u8>::with_capacity(size),
    }
}

fn write(rb: &mut RingBuffer, data: &[u8]) -> Result<usize> {
    if rb.size == rb.data.capacity() {
        return Err(MyError::NoSpaceLeft);
    }
    let count = std::cmp::min(data.len(), rb.data.capacity() - rb.size);
    for item in data.iter().take(count) {
        match rb.data.get(rb.write_idx) {
            Some(_) => rb.data[rb.write_idx] = *item,
            None => rb.data.push(*item),
        }
        if rb.write_idx + 1 == rb.data.capacity() {
            rb.write_idx = 0;
        } else {
            rb.write_idx += 1;
        }
        rb.size += 1;
    }
    Ok(count)
}

fn read(rb: &mut RingBuffer, mut count: usize) -> Option<Vec<u8>> {
    if rb.size == 0 {
        return None;
    }
    let mut arr: Vec<u8> = Vec::new();
    count = std::cmp::min(count, rb.size);
    for _ in 0..count {
        arr.push(rb.data[rb.read_idx]);
        if rb.read_idx + 1 == rb.data.capacity() {
            rb.read_idx = 0;
        } else {
            rb.read_idx += 1;
        }
        rb.size -= 1;
    }
    Some(arr)
}

fn main() {
    let mut rb = create(5);
    assert!(read(&mut rb, 5).is_none());

    assert_eq!(write(&mut rb, &[1]).unwrap(), 1);
    assert_eq!(write(&mut rb, &[2, 3]).unwrap(), 2);
    assert_eq!(rb.data, [1, 2, 3]);

    assert_eq!(read(&mut rb, 1).unwrap(), [1]);
    assert_eq!(rb.data, [1, 2, 3]);
    assert_eq!(write(&mut rb, &[4, 5, 6, 10, 20]).unwrap(), 3);

    assert_eq!(rb.data, [6, 2, 3, 4, 5]);
    assert!(write(&mut rb, &[10]).is_err());
    assert_eq!(rb.data, [6, 2, 3, 4, 5]);
    assert_eq!(read(&mut rb, 1).unwrap(), [2]);
    assert_eq!(rb.data, [6, 2, 3, 4, 5]);
    assert_eq!(write(&mut rb, &[10]).unwrap(), 1);
    assert_eq!(rb.data, [6, 10, 3, 4, 5]);

    assert!(write(&mut rb, &[20]).is_err());
    assert_eq!(read(&mut rb, 4).unwrap(), [3, 4, 5, 6]);
    assert_eq!(write(&mut rb, &[20]).unwrap(), 1);
    assert_eq!(rb.data, [6, 10, 20, 4, 5]);
    assert_eq!(read(&mut rb, 3).unwrap(), [10, 20]);
    assert!(read(&mut rb, 1).is_none());

    assert_eq!(write(&mut rb, &[30, 40, 50]).unwrap(), 3);
    assert_eq!(rb.data, [50, 10, 20, 30, 40]);
}
