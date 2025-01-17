pub struct RingBuffer<'a> {
    buf: &'a mut [u8],
    start: usize,
    end: usize,
    empty: bool,
}

impl<'a> RingBuffer<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self {
            buf,
            start: 0,
            end: 0,
            empty: true,
        }
    }

    pub fn push_buf(&mut self) -> &mut [u8] {
        if self.start == self.end && !self.empty {
            trace!("  ringbuf: push_buf empty");
            return &mut self.buf[..0];
        }

        let n = if self.start <= self.end {
            self.buf.len() - self.end
        } else {
            self.start - self.end
        };

        trace!("  ringbuf: push_buf {:?}..{:?}", self.end, self.end + n);
        &mut self.buf[self.end..self.end + n]
    }

    pub fn push(&mut self, n: usize) {
        trace!("  ringbuf: push {:?}", n);
        if n == 0 {
            return;
        }

        self.end = self.wrap(self.end + n);
        self.empty = false;
    }

    pub fn pop_buf(&mut self) -> &mut [u8] {
        if self.empty {
            trace!("  ringbuf: pop_buf empty");
            return &mut self.buf[..0];
        }

        let n = if self.end <= self.start {
            self.buf.len() - self.start
        } else {
            self.end - self.start
        };

        trace!("  ringbuf: pop_buf {:?}..{:?}", self.start, self.start + n);
        &mut self.buf[self.start..self.start + n]
    }

    pub fn pop(&mut self, n: usize) {
        trace!("  ringbuf: pop {:?}", n);
        if n == 0 {
            return;
        }

        self.start = self.wrap(self.start + n);
        self.empty = self.start == self.end;
    }

    pub fn clear(&mut self) {
        self.start = 0;
        self.end = 0;
        self.empty = true;
    }

    fn wrap(&self, n: usize) -> usize {
        assert!(n <= self.buf.len());
        if n == self.buf.len() {
            0
        } else {
            n
        }
    }
}
