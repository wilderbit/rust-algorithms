#[derive(Debug)]
struct MaxPQ {
    pub data: Vec<i32>,
    pub size: usize,
    pub n: usize,
}

impl MaxPQ {
    pub fn new(size: usize) -> Self {
        MaxPQ {
            data: vec![0; size],
            size: 0,
            n: size
        }
    }

    pub fn del_max(&mut self) -> i32 {
        let elem = self.data[0];
        self.data.swap(0, self.size - 1);
        self.size -= 1;
        MaxPQ::sink(&mut self.data, 0, self.size);
        return elem;
    }

    pub fn sink(data: &mut [i32], k: usize, n: usize) {
        let mut k = k;
        while 2 * k + 1 < n {
            let mut swap_pos = 2 * k + 1;
            if swap_pos + 1 < n && MaxPQ::less(&data[swap_pos], &data[swap_pos + 1]) {
                swap_pos += 1
            }
            if MaxPQ::less(&data[k], &data[swap_pos]) {
                data.swap(k, swap_pos);
                k = swap_pos;
            } else {
                break
            }
        }
    }

    fn less(a: &i32, b: &i32) -> bool {
        a < b
    }

    pub fn swim(data: &mut [i32], k: usize) {
        let mut n = k;
        while n > 0 {
            let p = (n - 1) / 2;
            if MaxPQ::less(&data[p], &data[n]) {
                data.swap(p, n);
                n = p;
            } else {
                break;
            }
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn insert(&mut self, data: i32) {
        self.data[self.size] = data;
        MaxPQ::swim(&mut self.data, self.size);
        self.size += 1
    }

    pub fn max() {}
}

fn main() {
    let mut st = MaxPQ::new(11);
    let arr: [i32; 10] = [9, 19, 2, 3, 14, 12, 13, 7, 8, 15];
    for elem in arr.iter() {
        st.insert(*elem)
    }
//    st.del_max();
    println!("{:?}", st);
    st.del_max();
    println!("{:?}", st);
    st.del_max();
    println!("{:?}", st);
    st.del_max();
    println!("{:?}", st);
    //    let mut data = [7, 4, 5, 2, 1, 3, 8];
    //    MaxPQ::swim(&mut data, 6);
    //    let mut data = [1, 5, 4, 3 , 1];
    //    MaxPQ::sink(&mut data, 0, 5);
    //    println!("{:?}", data);
}
