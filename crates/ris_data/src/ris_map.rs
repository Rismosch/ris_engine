// https://nullprogram.com/blog/2022/08/08/

fn ht_lookup(hash: u64, exp: usize, idx: usize) -> usize {
    let mask: usize = ((1u32 << exp) - 1) as usize;
    let step: usize = ((hash >> (64 - exp)) | 1) as usize;
    (idx + step) & mask
}

fn hash(s: &str) -> u64 {
    let mut h = 0x100u64;

    for c in s.chars() {
        h ^= c as u64 & 255;
        h = u64::wrapping_mul(h, 1111111111111111111);
    }

    h ^ h >> 32
}

const EXP: usize = 15;
const GRAVESTONE: &str = "__deleted__";

pub struct OutOfMemory;
pub struct NotFound;

impl std::fmt::Debug for OutOfMemory
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OutOfMemory")
    }
}

impl std::fmt::Debug for NotFound
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NotFound")
    }
}

struct Ht {
    ht: Vec<Option<String>>,
    len: i32,
}

impl Ht{
    fn intern(&mut self, key: &str) -> Result<usize, OutOfMemory> {
        let h = hash(key);
        let mut i = h as usize;
        let mut destination = None;
        loop {
            i = ht_lookup(h, EXP, i);

            if self.ht[i].is_none() {
                // empty, insert here
                
                if self.len + 1 == 1 << EXP {
                    return Err(OutOfMemory);
                }

                let index = match destination {
                    Some(index) => index,
                    None => i,
                };
                
                self.len += 1;
                self.ht[index] = Some(String::from(key));

                return Ok(index);
            } else if let Some(existing_key) = &self.ht[i] {
                if existing_key.eq(GRAVESTONE) {
                    if destination.is_none() {
                        destination = Some(i);
                    }
                } else if existing_key.eq(key) {
                    // found, return canonical instance
                    return Ok(i);
                }
            }
        }
    }

    fn unintern(&mut self, key: &str) -> Result<usize, NotFound> {
        let h = hash(key);
        let mut i = h as usize;
        loop {
            i = ht_lookup(h, EXP, i);

            if self.ht[i].is_none() {
                return Err(NotFound);
            } else if let Some(existing_key) = &self.ht[i] {
                if existing_key.eq(GRAVESTONE) {
                    // skip over
                } else if existing_key.eq(key) {
                    self.ht[i] = Some(String::from(GRAVESTONE));
                    return Ok(i);
                }
            }
        }
    }
}

pub struct RisMap<T> {
    ht: Ht,
    len: usize,
    buf: Vec<Option<T>>,
}

impl<T> RisMap<T>{
    /// inserts the key and assigns the item to it. overwrites the item that is already assigned to
    /// that key
    pub fn assign(&mut self, key: &str, item: T) -> Result<(), OutOfMemory> {
        let result = self.ht.intern(key);
        if let Ok(index) = result {
            self.len += 1;
            self.buf[index] = Some(item);
            Ok(())
        } else {
            Err(OutOfMemory)
        }
    }

    /// remove a key and its assigned value from the map
    pub fn remove(&mut self, key: &str) -> Result<(), NotFound> {
        let result = self.ht.unintern(key);
        if let Ok(index) = result {
            self.len -= 1;
            self.buf[index] = None;
            Ok(())
        } else {
            Err(NotFound)
        }
    }

    /// indexing inserts the key into the hashtable, which requires it to be mutable. therefore, no
    /// immutable version of this method exists
    pub fn find<'a>(&'a mut self, key: &str) -> Result<Option<&'a mut T>, OutOfMemory> {
        let result = self.ht.intern(key);
        match result {
            Ok(index) => Ok(self.buf[index].as_mut()),
            Err(OutOfMemory) => Err(OutOfMemory),
        }
    }
}

impl<T: Copy> Default for RisMap<T> {
    fn default() -> Self {
        let mut ht_init = Vec::with_capacity(1<<EXP);
        let mut buf_init = Vec::with_capacity(1<<EXP);

        for _ in 0..1<<EXP
        {
            ht_init.push(None);
            buf_init.push(None);
        }

        let ht = Ht {
            ht: ht_init,
            len: 0,
        };

        Self {
            ht,
            len: 0,
            buf: buf_init,
        }
    }
}
