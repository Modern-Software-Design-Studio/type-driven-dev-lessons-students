use std::cmp::Ord;

struct List<T> {
    data: Vec<T>,
}

impl<T: Clone + std::fmt::Debug + Ord> List<T> {
    fn new() -> Self {
        List { data: Vec::new() }
    }

    // Intended contract: if the list is sorted, the result stays sorted.
    fn insert(mut self, value: T) -> Self {
        let pos = self.data.partition_point(|x| x <= &value);
        self.data.insert(pos, value);
        self
    }

    // Intended contract: if both lists are sorted, the result is sorted.
    fn merge(self, other: Self) -> Self {
        let mut result = Vec::with_capacity(self.data.len() + other.data.len());
        let mut i = 0;
        let mut j = 0;

        while i < self.data.len() && j < other.data.len() {
            if self.data[i] <= other.data[j] {
                result.push(self.data[i].clone());
                i += 1;
            } else {
                result.push(other.data[j].clone());
                j += 1;
            }
        }

        while i < self.data.len() {
            result.push(self.data[i].clone());
            i += 1;
        }

        while j < other.data.len() {
            result.push(other.data[j].clone());
            j += 1;
        }

        List { data: result }
    }

    fn concat(self, other: Self) -> Self {
        let mut data = self.data;
        data.extend(other.data);
        List { data }
    }

    fn push(mut self, value: T) -> Self {
        self.data.push(value);
        self
    }

    fn sort(mut self) -> Self {
        self.data.sort();
        self
    }

    fn print(&self, label: &str) {
        println!("  {}: {:?}", label, self.data);
    }
}

fn main() {
    let a = List::<i32>::new().insert(1).insert(3).insert(5);
    let b = List::<i32>::new().push(9).push(2);

    a.print("a (built with insert)");
    b.print("b (built with push)");

    // b is not sorted, but nothing stops us from merging it:
    let c = a.merge(b);
    c.print("a.merge(b)");

    let d = List::<i32>::new().insert(1).insert(3).insert(5);
    let e = List::<i32>::new().insert(2).insert(4).insert(6);
    let f = d.concat(e);
    f.print("d.concat(e)");

    let g = f.sort();
    g.print("f.sort()");
}
