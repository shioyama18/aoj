// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_9_C

fn extract_max(heap: &mut Vec<usize>) -> usize {
    if heap.len() == 1 { panic!("Heap is empty"); }
    let max = heap[1];
    heap[1] = heap[heap.len()-1];
    heap.pop();
    max_heapify(heap, 1);
    max
}

fn max_heapify(heap: &mut Vec<usize>, i: usize) {
    let l = i*2;
    let r = i*2+1;
    let mut largest: usize;

    if l <= heap.len()-1 && heap[l] > heap[i] {
        largest = l;
    } else {
        largest = i;
    }

    if r <= heap.len()-1 && heap[r] > heap[largest] {
        largest = r;
    }
    
    if largest != i {
        heap.swap(i, largest);
        max_heapify(heap, largest);
    }
}

fn insert(heap: &mut Vec<usize>, key: usize) {
    heap.push(key);
    let mut i = heap.len()-1;
    while i > 1 && heap[parent(i)] < heap[i] {
        heap.swap(parent(i), i);
        i = parent(i)
    }    
}

fn parent(i: usize) -> usize {
    i / 2
}

fn main() {
    let mut heap  = Vec::new();
    heap.push(0);
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).ok();
        let buffer: Vec<&str> = buffer.split_whitespace().collect();
        if buffer[0] == "end" {
            break;
        } else if buffer[0] == "extract" {
            println!("{}", extract_max(&mut heap));
        } else {
            let key = buffer[1].parse::<usize>().unwrap();
            insert(&mut heap, key);
        }
    }
}
