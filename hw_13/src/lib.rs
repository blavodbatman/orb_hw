use std::sync::Mutex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
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

pub struct SyncRingBuffer {
    synced_rb: Mutex<RingBuffer>,
}

pub fn create(size: usize) -> SyncRingBuffer {
    SyncRingBuffer {
        synced_rb: Mutex::new(RingBuffer {
            read_idx: 0,
            write_idx: 0,
            size: 0,
            data: Vec::<u8>::with_capacity(size),
        }),
    }
}

pub fn write(srb: &SyncRingBuffer, data: &[u8]) -> Result<usize> {
    let mut rb = srb.synced_rb.lock().unwrap();

    if rb.size == rb.data.capacity() {
        return Err(MyError::NoSpaceLeft);
    }
    let count = std::cmp::min(data.len(), rb.data.capacity() - rb.size);
    for item in data.iter().take(count) {
        let index = rb.write_idx;
        match rb.data.get(index) {
            Some(_) => rb.data[index] = *item,
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

pub fn read(srb: &SyncRingBuffer, mut count: usize) -> Option<Vec<u8>> {
    let mut rb = srb.synced_rb.lock().unwrap();

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

/*fn main() {
    let srb = create(5);
    assert!(read(&srb, 5).is_none());

    assert_eq!(write(&srb, &[1]).unwrap(), 1);
    assert_eq!(write(&srb, &[2, 3]).unwrap(), 2);
    assert_eq!(srb.synced_rb.lock().unwrap().data, [1, 2, 3]);

    assert_eq!(read(&srb, 1).unwrap(), [1]);
    assert_eq!(srb.synced_rb.lock().unwrap().data, [1, 2, 3]);
    assert_eq!(write(&srb, &[4, 5, 6, 10, 20]).unwrap(), 3);

    assert_eq!(srb.synced_rb.lock().unwrap().data, [6, 2, 3, 4, 5]);
    assert!(write(&srb, &[10]).is_err());
    assert_eq!(srb.synced_rb.lock().unwrap().data, [6, 2, 3, 4, 5]);
    assert_eq!(read(&srb, 1).unwrap(), [2]);
    assert_eq!(srb.synced_rb.lock().unwrap().data, [6, 2, 3, 4, 5]);
    assert_eq!(write(&srb, &[10]).unwrap(), 1);
    assert_eq!(srb.synced_rb.lock().unwrap().data, [6, 10, 3, 4, 5]);

    assert!(write(&srb, &[20]).is_err());
    assert_eq!(read(&srb, 4).unwrap(), [3, 4, 5, 6]);
    assert_eq!(write(&srb, &[20]).unwrap(), 1);
    assert_eq!(srb.synced_rb.lock().unwrap().data, [6, 10, 20, 4, 5]);
    assert_eq!(read(&srb, 3).unwrap(), [10, 20]);
    assert!(read(&srb, 1).is_none());

    assert_eq!(write(&srb, &[30, 40, 50]).unwrap(), 3);
    assert_eq!(srb.synced_rb.lock().unwrap().data, [50, 10, 20, 30, 40]);
}*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_threaded_read_write() {
        let srb = Arc::new(create(50));

        let srb_write = Arc::clone(&srb);
        let writing_threas = thread::spawn(move || {
            for _ in 0..10000 {
                loop {
                    if write(&srb_write, &[1]).is_ok() {
                        break;
                    }
                }
            }
        });

        let srb_read = Arc::clone(&srb);
        let reading_threads = thread::spawn(move || {
            let mut total_count = 0;
            while total_count < 10000 {
                if let Some(arr) = read(&srb_read, 10) {
                    total_count += arr.len();
                }
            }
            assert_eq!(total_count, 10000);
        });

        writing_threas.join().unwrap();
        reading_threads.join().unwrap();
    }
}
